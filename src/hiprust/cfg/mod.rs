use serialize::json;
use std::io;

use ioutils;

#[deriving(Decodable, Encodable)]
pub struct Config {
    auth_token: String,
    api_host: String,
}

#[must_use]
fn from_string(s: &str) -> Result<Config, String> {
    let decoded = json::decode::<Config>(s);
    match decoded {
        Ok(x) => Ok(x),
        Err(msg) => Err(format!("decoding json failed: {}", msg))
    }
}

#[must_use]
fn from_file(p: &Path) -> Result<Config, String> {
    let s = ioutils::slurp_string(p);
    match s {
        Ok(x) => from_string(x.as_slice()),
        Err(msg) => Err(format!("opening/reading file failed: {}", msg))
    }
}

#[cfg(test)]
mod test {
    use std::io;

    use ioutils;

    fn valid_cfg_json() -> &'static str {
        "{\
            \"auth_token\": \"test\",\
            \"api_host\": \"api.hipchat.com\"\
         }"
    }

    #[test]
    fn test_from_string() {
        super::from_string(valid_cfg_json()).unwrap();
    }

    #[test]
    fn test_from_file() {
        let tmp_dir = io::TempDir::new("jsontest").unwrap();
        let mut cfg_path = tmp_dir.path().clone();
        cfg_path.push("cfg");

        ioutils::barf_string(&cfg_path, valid_cfg_json()).unwrap();

        super::from_file(&cfg_path).unwrap();
    }
}
