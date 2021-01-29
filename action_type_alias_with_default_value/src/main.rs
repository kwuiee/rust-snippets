// with default type.
type Result1<I, E = std::io::Error> = Result<I, E>;
// associated type not allowed here.
// type Result2<I> = Result<I, E = std::io::Error>;
type Result3<I> = Result<I, std::io::Error>;

fn proc10() -> Result1<u8> {
    Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "1"))
}

fn proc11() -> Result1<u8, std::io::Error> {
    Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "1"))
}

fn proc3() -> Result3<u8> {
    Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "1"))
}

fn main() {}
