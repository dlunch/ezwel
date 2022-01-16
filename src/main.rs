use anyhow::Result;
use regex::Regex;

#[tokio::main]
async fn main() -> Result<()> {
    let id = "";
    let password = "";
    let company = "";

    let client = reqwest::Client::builder().cookie_store(true).build()?;

    // set cookie
    client
        .get(format!("https://{}.ezwel.com/cuser/login/loginForm.ez", company))
        .send()
        .await?;

    // login
    let response = client
        .post(format!("https://{}.ezwel.com/cuser/login/loginAction.ez", company))
        .form(&[
            ("loginSearchBean.userId", id),
            ("loginSearchBean.password", &base64::encode(password)),
            ("loginSearchBean.loginType", "S"),
            ("clientCd", "sendbird"),
        ])
        .send()
        .await?;

    let response_content = response.text().await?;
    let re = Regex::new("var userKey = \"(\\d*)\";")?;
    let user_key = re.captures(&response_content).unwrap().get(1).unwrap().as_str();

    // get usage list

    let response = client
        .post(format!(
            "https://{}.ezwel.com/cuser/mypage/offlinecard/ajax/offCardUseListAjax.ez",
            company
        ))
        .form(&[
            ("status", "1001"),
            ("clientCd", company),
            ("userKey", user_key),
            ("specialPointUseYn", "N"),
            ("jsonYn", "Y"),
            ("startDt1", "2022-01-01"),
            ("endDt1", "2022-01-16"),
            ("startDt", "20220101"),
            ("endDt", "20220116"),
        ])
        .send()
        .await?;

    println!("{}", response.text().await?);

    Ok(())
}
