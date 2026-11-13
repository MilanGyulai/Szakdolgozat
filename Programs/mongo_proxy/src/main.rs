mod latency;
mod proxy;
mod wire;

const DEFAULT_LISTEN_ADDR: &str = "127.0.0.1:27018";
const DEFAULT_UPSTREAM_ADDR: &str = "127.0.0.1:27017";
#[tokio::main]
async fn main() -> std::io::Result<()> {
    let mut args = std::env::args().skip(1);
    let listen_addr = args.next().unwrap_or_else(|| DEFAULT_LISTEN_ADDR.to_owned());
    let upstream_addr = args.next().unwrap_or_else(|| DEFAULT_UPSTREAM_ADDR.to_owned());

    proxy::run(&listen_addr, upstream_addr).await
}
