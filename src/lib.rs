use lambda_http::Request;

#[allow(dead_code)]
pub fn parse_bearer_token(request_with_header: &Request) -> anyhow::Result<String> {
    let authorization = request_with_header
        .headers()
        .get("Authorization")
        .ok_or(anyhow::anyhow!("failed to get event"))?;

    let authorization_str = authorization.to_str()?;

    if authorization_str.starts_with("Bearer ") {
        let trimed_str = authorization_str.trim_start_matches("Bearer ").to_string();
        Ok(trimed_str)
    } else {
        anyhow::bail!("failed to trim bearer")
    }
}
