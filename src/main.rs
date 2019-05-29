use std::io::{stdin, stdout, Read, StdinLock, Write};

fn scan<T>(s: &mut StdinLock, initial_value: T, fold_callback: &Fn(T, char) -> T) -> T {
    s.by_ref()
        .bytes()
        .map(|byte| byte.unwrap() as char)
        .skip_while(|character| character.is_whitespace())
        .take_while(|character| !character.is_whitespace())
        .fold(initial_value, fold_callback)
}

fn scan_u(s: &mut StdinLock) -> usize {
    scan(s, 0, &|accumulator, character| {
        (character as u8 - b'0') as usize + accumulator * 10
    })
}

fn scan_s(s: &mut StdinLock) -> String {
    s.by_ref()
        .bytes()
        .map(|byte| byte.unwrap() as char)
        .skip_while(|character| character.is_whitespace())
        .take_while(|character| !character.is_whitespace())
        .collect::<String>()
}

struct CharacterOutput {
    string_buffer: String,
}

impl CharacterOutput {
    fn push_s(&mut self, s: &str) {
        self.string_buffer += s;
    }

    fn add_s(&mut self, s: &str, separator: &str) {
        self.push_s(s);
        self.push_s(separator);
    }

    fn push<T: ToString>(&mut self, n: T) {
        self.push_s(&n.to_string());
    }

    fn add<T: ToString>(&mut self, n: T, separator: &str) {
        self.push(n);
        self.push_s(separator);
    }

    fn flush(&mut self) {
        let out = stdout();
        let mut out = out.lock();
        out.write_all(self.string_buffer.as_bytes()).unwrap();
        self.string_buffer.clear();
    }
}

fn create_cout(capacity: usize) -> CharacterOutput {
    CharacterOutput {
        string_buffer: String::with_capacity(capacity),
    }
}

// ------------------------------------------------------------------------

fn process(cin: &mut StdinLock, cout: &mut CharacterOutput) {
    // numbers A, B, C
    cout.add((0..3).fold(0, |acc, _| acc + scan_u(cin)), " ");

    // string S
    cout.add_s(&scan_s(cin), "\n");
}

// ------------------------------------------------------------------------

fn main() {
    let cin = stdin();
    let mut cin = cin.lock();
    let cin = &mut cin;
    let cout = &mut create_cout(0);

    process(cin, cout);

    cout.flush();
}
