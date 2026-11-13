use std::io;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

use crate::latency::RequestTracker;
use crate::wire::{self, Header, HEADER_LEN};

pub async fn run(listen_addr: &str, upstream_addr: String) -> io::Result<()> {
    let listener = TcpListener::bind(listen_addr).await?;
    println!("sniffer listening on {listen_addr}, forwarding to {upstream_addr}");

    let mut next_connection_id: u64 = 0;
    loop {
        let (client, peer) = match listener.accept().await {
            Ok(accepted) => accepted,
            Err(error) => {
                eprintln!("accept failed: {error}");
                continue;
            }
        };

        next_connection_id += 1;
        let connection_id = next_connection_id;
        let upstream_addr = upstream_addr.clone();

        tokio::spawn(async move {
            println!("[conn {connection_id}] opened from {peer}");
            if let Err(error) = handle_connection(client, &upstream_addr, connection_id).await {
                eprintln!("[conn {connection_id}] error: {error}");
            }
            println!("[conn {connection_id}] closed");
        });
    }
}

async fn handle_connection(client: TcpStream, upstream_addr: &str, connection_id: u64) -> io::Result<()> {
    let server = TcpStream::connect(upstream_addr).await?;
    client.set_nodelay(true)?;
    server.set_nodelay(true)?;

    let tracker = RequestTracker::default();
    let (client_reader, client_writer) = client.into_split();
    let (server_reader, server_writer) = server.into_split();

    tokio::try_join!(
        pump(client_reader, server_writer, |frame| on_request(connection_id, &tracker, frame)),
        pump(server_reader, client_writer, |frame| on_response(connection_id, &tracker, frame)),
    )?;
    Ok(())
}

async fn pump<R, W>(mut reader: R, mut writer: W, mut inspect: impl FnMut(&[u8])) -> io::Result<()>
where
    R: AsyncRead + Unpin,
    W: AsyncWrite + Unpin,
{
    while let Some(frame) = read_frame(&mut reader).await? {
        writer.write_all(&frame).await?;
        inspect(&frame);
    }
    writer.shutdown().await
}

async fn read_frame<R: AsyncRead + Unpin>(reader: &mut R) -> io::Result<Option<Vec<u8>>> {
    let mut header = [0u8; HEADER_LEN];
    match reader.read_exact(&mut header).await {
        Ok(_) => {}
        Err(error) if error.kind() == io::ErrorKind::UnexpectedEof => return Ok(None),
        Err(error) => return Err(error),
    }

    let length = Header::parse(&header)
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "not a MongoDB message (TLS?)"))?
        .length;

    let mut frame = vec![0u8; length];
    frame[..HEADER_LEN].copy_from_slice(&header);
    reader.read_exact(&mut frame[HEADER_LEN..]).await?;
    Ok(Some(frame))
}

fn on_request(connection_id: u64, tracker: &RequestTracker, frame: &[u8]) {
    let Some(message) = wire::decode(frame) else {
        return;
    };
    let label = message.label();
    println!("[conn {connection_id}] -> {label} ({} B)", frame.len());

    if !message.more_to_come {
        tracker.start(message.header.request_id, label, frame.len());
    }
}

fn on_response(connection_id: u64, tracker: &RequestTracker, frame: &[u8]) {
    let Some(header) = Header::parse(frame) else {
        return;
    };

    match tracker.finish(header.response_to) {
        Some(request) => println!(
            "[conn {connection_id}] <- {} ({} B) latency {:.3} ms",
            request.label,
            frame.len(),
            request.latency.as_secs_f64() * 1000.0,
        ),
        None => println!("[conn {connection_id}] <- {} ({} B)", header.op_name(), frame.len()),
    }
}