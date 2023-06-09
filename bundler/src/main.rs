use std::io::{self, Read, Write};
use std::process::exit;
use std::{fmt, fs};
use wasm_blob_bundler::BlobBundler;

fn main() {
    let mut args = std::env::args();
    args.next().unwrap();
    let out_file = args.next_back().unwrap_or_else(help_and_exit);
    let out_file = (out_file != "-").then_some(out_file);
    if args.len() == 0 {
        help_and_exit()
    }

    let mut bundler = BlobBundler::new();
    for arg in args {
        let (name, mut file) = arg.split_once(':').unwrap_or_else(help_and_exit);
        let blob = if file == "-" {
            file = "stdin";
            let mut buf = Vec::new();
            io::stdin().lock().read_to_end(&mut buf).unwrap_with_file(file);
            buf
        } else {
            fs::read(file).unwrap_with_file(file)
        };
        bundler.blob(name, &blob).unwrap_with_file(file);
    }
    let module = bundler.build();

    if let Some(out_file) = &out_file {
        fs::write(out_file, module).unwrap_with_file(out_file);
    } else {
        let mut stdout = io::stdout().lock();
        stdout.write_all(&module).unwrap_or_exit();
        stdout.flush().unwrap_or_exit()
    }
}

trait ResultExt<T> {
    fn unwrap_or_exit(self) -> T;
    fn unwrap_with_file(self, file: &str) -> T;
}
impl<T, E: fmt::Display> ResultExt<T> for Result<T, E> {
    fn unwrap_or_exit(self) -> T {
        self.unwrap_or_else(|e| {
            eprintln!("{Prog}: {e}");
            exit(1)
        })
    }
    fn unwrap_with_file(self, file: &str) -> T {
        self.unwrap_or_else(|e| {
            eprintln!("{Prog}: {file}: {e}");
            exit(1)
        })
    }
}

struct Prog;
impl fmt::Display for Prog {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&std::env::args().next().unwrap())
    }
}

fn help_and_exit<T>() -> T {
    eprintln!("usage: {Prog} BLOB_NAME:(FILE_PATH|-)... OUT_FILE|-");
    exit(1)
}
