use serialize::json;
use std::io;

#[deriving(Decodable, Encodable)]
pub struct Config {
    auth_token: String,
    api_host: String,
}

fn from_string(s: &str) -> Config {
    return json::decode::<Config>(s).unwrap();

}

fn from_file(p: &Path) -> Config {
    from_string(io::BufferedReader::new(io::File::open(p)).read_to_string().unwrap().as_slice())
}

#[cfg(test)]
mod test {
    use std::io;

    fn valid_cfg_json() -> &'static str {
        "{\
            \"auth_token\": \"test\",\
            \"api_host\": \"api.hipchat.com\"\
         }"
    }

    #[test]
    fn test_from_string() {
        super::from_string(valid_cfg_json());
    }

    #[test]
    fn test_from_file() {
        // XXX(scode): The hoop jumping to create tmp_path seems wrong and ugly.
        let tmp_dir = io::TempDir::new("jsontest").unwrap();
        let mut tmp_path = Path::new(tmp_dir.path());
        tmp_path.push("tmpcfg");

        let mut tmp_out = io::BufferedWriter::new(io::File::open_mode(&tmp_path, io::Open, io::Write)).unwrap();

        tmp_out.write_str(valid_cfg_json()).unwrap();
        tmp_out.flush().unwrap();

        super::from_file(&tmp_path);
    }
}
