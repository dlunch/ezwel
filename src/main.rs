use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let client = reqwest::Client::builder().cookie_store(true).build()?;

    // set cookie
    client
        .get("https://sendbird.ezwel.com/cuser/login/loginForm.ez")
        .send()
        .await?;

    // login

    Ok(())
}
