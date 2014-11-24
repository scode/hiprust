use std::io;

#[must_use]
pub fn barf_bytes(path: &Path, contents: &[u8]) -> Result<(), io::IoError> {
    let mut file = try!(io::File::open_mode(path, io::Open, io::Write));
    try!(file.write(contents));

    return file.flush();
}

#[must_use]
pub fn barf_string(path: &Path, contents: &str) -> Result<(), io::IoError> {
    return barf_bytes(path, contents.as_bytes());
}

#[must_use]
pub fn slurp_bytes(path: &Path) -> Result<Vec<u8>, io::IoError> {
    let mut f = try!(io::File::open(path));
    return try!(Ok(f.read_to_end()));
}

#[must_use]
pub fn slurp_string(path: &Path) -> Result<String, io::IoError> {
    let mut f = try!(io::File::open(path));
    return try!(Ok(f.read_to_string()));
}

#[cfg(test)]
mod test {
    use std::io;

    #[test]
    fn test_barf_slurp_string() {
        let tmp_dir = io::TempDir::new("ioutilstest").unwrap();
        let mut p = tmp_dir.path().clone();
        p.push("testfile");

        super::barf_string(&p, "test contents").unwrap();
        let slurped = super::slurp_string(&p).unwrap();

        assert!(slurped.as_slice() == "test contents".as_slice());
    }
}
