use std::io::{PipeReader, PipeWriter, Read, Write};
use std::process::{ChildStderr, ChildStdin, ChildStdout, Stdio};

pub(crate) enum ReadPipe {
    Stderr(ChildStderr),
    Stdout(ChildStdout),
    Reader(PipeReader),
}

impl Read for ReadPipe {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        match self {
            ReadPipe::Stderr(stderr) => stderr.read(buf),
            ReadPipe::Stdout(stdout) => stdout.read(buf),
            ReadPipe::Reader(reader) => reader.read(buf),
        }
    }
}

impl From<ReadPipe> for Stdio {
    fn from(value: ReadPipe) -> Self {
        match value {
            ReadPipe::Stderr(stderr) => stderr.into(),
            ReadPipe::Stdout(stdout) => stdout.into(),
            ReadPipe::Reader(stdin) => stdin.into(),
        }
    }
}

pub(crate) enum WritePipe {
    Stdin(ChildStdin),
    Writer(PipeWriter),
}

impl Write for WritePipe {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match self {
            WritePipe::Stdin(stdin) => stdin.write(buf),
            WritePipe::Writer(writer) => writer.write(buf),
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        match self {
            WritePipe::Stdin(stdin) => stdin.flush(),
            WritePipe::Writer(writer) => writer.flush(),
        }
    }
}

impl From<WritePipe> for Stdio {
    fn from(value: WritePipe) -> Self {
        match value {
            WritePipe::Stdin(stdin) => stdin.into(),
            WritePipe::Writer(writer) => writer.into(),
        }
    }
}