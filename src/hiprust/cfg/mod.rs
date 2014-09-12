use serialize::json;

#[deriving(Decodable, Encodable)]
pub struct Config {
    auth_token: String,
    api_host: String,
}

fn from_string(s: &str) -> Config {
    return json::decode::<Config>(s).unwrap();

}

#[cfg(test)]
mod test {
    use super::from_string;

    #[test]
    fn test_from_string() {
        from_string("{\
                     \"auth_token\": \"test\",\
                     \"api_host\": \"api.hipchat.com\"\
                    }");
    }
}
