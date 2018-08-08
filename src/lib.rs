#[derive(Clone)]
struct Offsets {
    command: usize,
    pid: usize,
    user: usize,
    fd: usize,
    type_: usize,
    device: usize,
    size_off: usize,
    node: usize,
    name: usize,
}

#[derive(Debug)]
struct Row {
    command: String,
    pid: String,
    user: String,
    fd: String,
    type_: String,
    device: String,
    size_off: String,
    node: String,
    name: String,
}

struct Parser<I> {
    orig: I,
    parser: Offsets,
}

impl Offsets {
    fn from_str(s: &str) -> Option<Self> {
        Some(Offsets {
            command: 0,
            pid: s.find("  PID")?,
            user: s.find("USER")?,
            fd: s.find("FD")?,
            type_: s.find("TYPE")?,
            device: s.find("DEVICE")?,
            size_off: s.find("SIZE/OFF")?,
            node: s.find("NODE")?,
            name: s.find("NAME")?,
        })
    }

    fn parser<I>(&self, iter: I) -> Parser<I> {
        Parser {
            orig: iter,
            parser: self.clone()
        }
    }

    fn parse_line(&self, line: &str) -> Option<Row> {
        Some(Row {
            command: line[..self.pid].trim_matches(' ').to_string(),
            pid: line[self.pid..self.pid + 5].trim_matches(' ').to_string(),
            user: line[self.pid + 5..self.user + 4].trim_matches(' ').to_string(),
            fd: line[self.user + 4..self.fd + 3].trim_matches(' ').to_string(),
            type_: line[self.fd + 3..self.type_ + 4].trim_matches(' ').to_string(),
            device: line[self.type_ + 4..self.device + 6].trim_matches(' ').to_string(),
            size_off: line[self.device + 6..self.size_off + 8].trim_matches(' ').to_string(),
            node: line[self.size_off + 8..self.node + 4].trim_matches(' ').to_string(),
            name: line[self.name..].trim_matches(' ').to_string(),
        })
    }
}

impl<'a, I> Iterator for Parser<I> where I: Iterator<Item=&'a str> {
    type Item = Row;

    fn next(&mut self) -> Option<Row> {
        match self.orig.next() {
            Some(line) => self.parser.parse_line(line),
            _ => None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_headers() {
        let offsets = Offsets::from_str("COMMAND     PID     USER   FD      TYPE             DEVICE   SIZE/OFF    NODE NAME").unwrap();
        assert_eq!(offsets.command, 0);
        assert_eq!(offsets.pid, 10);
        assert_eq!(offsets.user, 20);
        assert_eq!(offsets.fd, 27);
        assert_eq!(offsets.type_, 35);
        assert_eq!(offsets.device, 52);
        assert_eq!(offsets.size_off, 61);
        assert_eq!(offsets.node, 73);
        assert_eq!(offsets.name, 78);
    }

    #[test]
    fn parses_line() {
        let offsets = Offsets::from_str("COMMAND     PID     USER   FD      TYPE             DEVICE   SIZE/OFF    NODE NAME").unwrap();
        let lines = "loginwind    89 jjaggars  cwd       DIR                1,4        896       2 /".lines();
        let mut parser = offsets.parser(lines);
        let r = parser.next().unwrap();
        assert_eq!(r.command, "loginwind");
        assert_eq!(r.pid, "89");
        assert_eq!(r.user, "jjaggars");
        assert_eq!(r.fd, "cwd");
        assert_eq!(r.type_, "DIR");
        assert_eq!(r.device, "1,4");
        assert_eq!(r.size_off, "896");
        assert_eq!(r.node, "2");
        assert_eq!(r.name, "/");
    }

    #[test]
    fn parses_with_gaps() {
        let offsets = Offsets::from_str("COMMAND     PID     USER   FD      TYPE             DEVICE   SIZE/OFF    NODE NAME").unwrap();
        let lines = "UserEvent   269 jjaggars    6u     unix 0x20369f28a64f7ddd        0t0         ->0x20369f28a64f7d15".lines();
        let mut parser = offsets.parser(lines);
        let r = parser.next().unwrap();
        assert_eq!(r.command, "UserEvent");
        assert_eq!(r.pid, "269");
        assert_eq!(r.user, "jjaggars");
        assert_eq!(r.fd, "6u");
        assert_eq!(r.type_, "unix");
        assert_eq!(r.device, "0x20369f28a64f7ddd");
        assert_eq!(r.size_off, "0t0");
        assert_eq!(r.node, "");
        assert_eq!(r.name, "->0x20369f28a64f7d15");
    }
}
