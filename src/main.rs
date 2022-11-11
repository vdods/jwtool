/// This is just a representation useful for printing the JWT as JSON for use in formatting on the CLI via `jq .`
pub struct Jwt {
    pub header: String,
    pub payload: String,
    // non-base64-decoded.
    pub signature: String,
}

impl std::fmt::Display for Jwt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{{\"header\":{},\"payload\":{},\"signature\":\"{}\"}}", self.header, self.payload, self.signature)
    }
}

/// This only decodes, but does not verify, the JWT.
fn decode_jwt(s: &str) -> Result<Jwt, anyhow::Error> {
    let mut v: Vec<&str> = s.split('.').collect();
    if v.len() != 3 {
        return Err(anyhow::anyhow!("JWT expected 3 tokens split at '.', but got {} tokens", v.len()));
    }

    let signature = String::from(v.pop().unwrap());
    let payload = String::from_utf8(base64::decode_config(v.pop().unwrap(), base64::URL_SAFE_NO_PAD)?)?;
    let header = String::from_utf8(base64::decode_config(v.pop().unwrap(), base64::URL_SAFE_NO_PAD)?)?;
    assert!(v.is_empty());

    Ok(Jwt { header, payload, signature })
}

fn main() -> Result<(), anyhow::Error> {
    let mut s = String::new();
    use std::io::Read;
    std::io::stdin().read_to_string(&mut s).unwrap();
    let s_trimmed = s.trim();

    println!("{}", decode_jwt(s_trimmed)?);
    Ok(())
}
