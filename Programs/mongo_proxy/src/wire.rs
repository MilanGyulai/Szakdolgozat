use bson::Document;

pub const HEADER_LEN: usize = 16;
pub const MAX_MESSAGE_LEN: usize = 48_000_000;

const OP_REPLY: i32 = 1;
const OP_QUERY: i32 = 2004;
const OP_COMPRESSED: i32 = 2012;
const OP_MSG: i32 = 2013;

const CHECKSUM_PRESENT: u32 = 1 << 0;
const MORE_TO_COME: u32 = 1 << 1;
const CHECKSUM_LEN: usize = 4;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Header {
    pub length: usize,
    pub request_id: i32,
    pub response_to: i32,
    pub op_code: i32,
}

impl Header {
    pub fn parse(bytes: &[u8]) -> Option<Self> {
        let length = usize::try_from(read_i32(bytes, 0)?).ok()?;
        if !(HEADER_LEN..=MAX_MESSAGE_LEN).contains(&length) {
            return None;
        }
        Some(Self {
            length,
            request_id: read_i32(bytes, 4)?,
            response_to: read_i32(bytes, 8)?,
            op_code: read_i32(bytes, 12)?,
        })
    }

    pub fn op_name(&self) -> &'static str {
        match self.op_code {
            OP_REPLY => "OP_REPLY",
            OP_QUERY => "OP_QUERY",
            OP_COMPRESSED => "OP_COMPRESSED",
            OP_MSG => "OP_MSG",
            _ => "UNKNOWN_OP",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Command {
    pub name: String,
    pub database: Option<String>,
    pub collection: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Message {
    pub header: Header,
    pub command: Option<Command>,
    pub more_to_come: bool,
}

impl Message {
    pub fn label(&self) -> String {
        let Some(command) = &self.command else {
            return self.header.op_name().to_owned();
        };
        match (&command.database, &command.collection) {
            (Some(db), Some(collection)) => format!("{} {db}.{collection}", command.name),
            (Some(db), None) => format!("{} {db}", command.name),
            _ => command.name.clone(),
        }
    }
}

pub fn decode(frame: &[u8]) -> Option<Message> {
    let header = Header::parse(frame)?;
    let body = frame.get(HEADER_LEN..header.length)?;

    let (command, more_to_come) = match header.op_code {
        OP_MSG => decode_op_msg(body),
        OP_QUERY => (decode_op_query(body), false),
        _ => (None, false),
    };

    Some(Message {
        header,
        command,
        more_to_come,
    })
}

fn decode_op_msg(body: &[u8]) -> (Option<Command>, bool) {
    let Some(flags) = read_i32(body, 0).map(|raw| raw as u32) else {
        return (None, false);
    };
    let more_to_come = flags & MORE_TO_COME != 0;

    let sections_end = if flags & CHECKSUM_PRESENT != 0 {
        body.len().saturating_sub(CHECKSUM_LEN)
    } else {
        body.len()
    };
    let sections = body.get(4..sections_end).unwrap_or_default();

    let command = find_body_section(sections).and_then(command_from);
    (command, more_to_come)
}

fn find_body_section(mut sections: &[u8]) -> Option<Document> {
    while let Some((&kind, rest)) = sections.split_first() {
        let size = usize::try_from(read_i32(rest, 0)?).ok()?;
        let section = rest.get(..size)?;
        if kind == 0 {
            return Document::from_reader(section).ok();
        }
        sections = &rest[size..];
    }
    None
}

fn decode_op_query(body: &[u8]) -> Option<Command> {
    let after_flags = body.get(4..)?;
    let name_end = after_flags.iter().position(|&byte| byte == 0)?;
    let full_collection_name = std::str::from_utf8(&after_flags[..name_end]).ok()?;
    let query = after_flags.get(name_end + 1 + 8..)?;

    let mut command = command_from(Document::from_reader(query).ok()?)?;
    if command.database.is_none() {
        command.database = full_collection_name.split('.').next().map(str::to_owned);
    }
    Some(command)
}

fn command_from(document: Document) -> Option<Command> {
    let (name, value) = document.iter().next()?;
    Some(Command {
        name: name.clone(),
        database: document.get_str("$db").ok().map(str::to_owned),
        collection: value.as_str().map(str::to_owned),
    })
}

fn read_i32(bytes: &[u8], offset: usize) -> Option<i32> {
    let raw = bytes.get(offset..offset + 4)?;
    Some(i32::from_le_bytes(raw.try_into().ok()?))
}

#[cfg(test)]
mod tests {
    use bson::doc;

    use super::*;

    fn frame(op_code: i32, request_id: i32, body: &[u8]) -> Vec<u8> {
        let length = (HEADER_LEN + body.len()) as i32;
        let mut bytes = Vec::new();
        for field in [length, request_id, 0, op_code] {
            bytes.extend_from_slice(&field.to_le_bytes());
        }
        bytes.extend_from_slice(body);
        bytes
    }

    fn op_msg(flags: u32, document: &Document) -> Vec<u8> {
        let mut body = flags.to_le_bytes().to_vec();
        body.push(0);
        document.to_writer(&mut body).unwrap();
        frame(OP_MSG, 1, &body)
    }

    #[test]
    fn decodes_command_from_op_msg() {
        let bytes = op_msg(0, &doc! { "find": "users", "filter": {}, "$db": "test" });

        let message = decode(&bytes).unwrap();

        assert_eq!(message.label(), "find test.users");
        assert!(!message.more_to_come);
    }

    #[test]
    fn detects_more_to_come_flag() {
        let bytes = op_msg(MORE_TO_COME, &doc! { "insert": "users", "$db": "test" });

        assert!(decode(&bytes).unwrap().more_to_come);
    }

    #[test]
    fn skips_kind_one_sections() {
        let mut documents = Vec::new();
        doc! { "name": "a" }.to_writer(&mut documents).unwrap();
        let identifier = b"documents\0";
        let section_size = (4 + identifier.len() + documents.len()) as i32;

        let mut body = 0u32.to_le_bytes().to_vec();
        body.push(1);
        body.extend_from_slice(&section_size.to_le_bytes());
        body.extend_from_slice(identifier);
        body.extend_from_slice(&documents);
        body.push(0);
        doc! { "insert": "users", "$db": "test" }.to_writer(&mut body).unwrap();

        let message = decode(&frame(OP_MSG, 1, &body)).unwrap();

        assert_eq!(message.label(), "insert test.users");
    }

    #[test]
    fn decodes_legacy_op_query() {
        let mut body = 0i32.to_le_bytes().to_vec();
        body.extend_from_slice(b"admin.$cmd\0");
        body.extend_from_slice(&0i32.to_le_bytes());
        body.extend_from_slice(&(-1i32).to_le_bytes());
        doc! { "isMaster": 1 }.to_writer(&mut body).unwrap();

        let message = decode(&frame(OP_QUERY, 1, &body)).unwrap();

        assert_eq!(message.label(), "isMaster admin");
    }

    #[test]
    fn compressed_messages_are_labelled_by_op_code() {
        let message = decode(&frame(OP_COMPRESSED, 1, &[0u8; 20])).unwrap();

        assert_eq!(message.label(), "OP_COMPRESSED");
    }

    #[test]
    fn rejects_implausible_length() {
        assert_eq!(Header::parse(b"GET / HTTP/1.1\r\n"), None);
    }
}