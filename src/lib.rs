// even with the bare minimum of commands,
// [line]a
// [line]d
// we already have a remarkable amount of work to do.
// - keep track of the buffer: use a Vec<String> as the simplest thing
//   that could work.
// - keep track of the current line.
// - keep track of two modes: command mode and append mode.

struct AppendMode;
struct CommandMode;

impl AppendMode {
    pub fn step(&self, line: String, buf: &mut Buf) -> ModeWrapper {
        if line == "." {
            ModeWrapper::CommandMode(CommandMode)
        } else {
            // push line
            buf.buf.insert(buf.addr, line);
            buf.addr += 1;
            ModeWrapper::AppendMode(AppendMode)
        }
    }
}

impl CommandMode {
    pub fn step(&self, line: String, buf: &mut Buf) -> ModeWrapper {
        if line == "a" {
            ModeWrapper::AppendMode(AppendMode)
        } else if line == ",p" {
            // dumb hack
            println!("{:?}", buf.buf);
            ModeWrapper::CommandMode(CommandMode)
        } else if line.ends_with("d") {
            let (a, _) = line.split_at(line.len() - 1);
            let addr: usize = a.parse().expect("number");
            buf.buf.remove(addr - 1);
            // delete nth or current line
            ModeWrapper::CommandMode(CommandMode)
        } else {
            println!("?");
            ModeWrapper::CommandMode(CommandMode)
        }
    }
}

enum ModeWrapper {
    AppendMode(AppendMode),
    CommandMode(CommandMode),
}

impl ModeWrapper {
    pub fn step(&self, line: String, buf: &mut Buf) -> Self {
        match self {
            &ModeWrapper::AppendMode(ref inner) => inner.step(line, buf),
            &ModeWrapper::CommandMode(ref inner) => inner.step(line, buf),
        }
    }
}

struct Buf {
    buf: Vec<String>,
    addr: usize,
}

impl Buf {
    pub fn new() -> Self {
        Buf {
            buf: vec![],
            addr: 0,
        }
    }
    pub fn concat_lines(&self) -> String {
        self.buf.iter().map(|x| format!("{}\n", x)).collect()
    }
}

pub struct Ed {
    buf: Buf,
    mode: ModeWrapper,
}

impl Ed {
    pub fn new() -> Self {
        Ed {
            buf: Buf::new(),
            mode: ModeWrapper::CommandMode(CommandMode),
        }
    }
    pub fn send(&mut self, line: String) {
        self.mode = self.mode.step(line, &mut self.buf);
    }
    pub fn get_buf(&self) -> String {
        self.buf.concat_lines()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let inp = "a
foo
bar
baz
.
2d
";
        let exp = "foo
baz
";
        let mut ed = ::Ed::new();
        for line in inp.lines() {
            ed.send(line.into());
        }
        assert_eq!(ed.get_buf(), exp);
    }
}
