use anyhow::{anyhow, Result};
use num_format::{Locale, ToFormattedString};
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

    let response = response.json::<serde_json::Value>().await?;

    let response_array = response.as_array().unwrap();
    let success = response_array[0]["successYn"].as_str().unwrap();

    if success != "Y" {
        return Err(anyhow!("Failed to get result"));
    }

    for item in response_array.iter().skip(1) {
        let date = item.get("useDate").unwrap().as_str().unwrap();
        let price = item.get("usePrice").unwrap().as_u64().unwrap();
        let kind = item.get("storeKname").unwrap().as_str().unwrap();
        let store = item.get("storeName").unwrap().as_str().unwrap();

        println!("{} {} {} {}", date, kind, store, price);

        let store_kind = item.get("storeKind").unwrap().as_str().unwrap();
        let store_code = item.get("storeCode").unwrap().as_str().unwrap();
        let card_type = item.get("cardType").unwrap().as_str().unwrap();
        let process_num = item.get("processNum").unwrap().as_str().unwrap();
        let level_cd = item.get("levelCd").unwrap().as_str().unwrap();
        let key_code = item.get("keyCode").unwrap().as_str().unwrap().replace('_', " ");
        let use_date = item.get("useDate").unwrap().as_str().unwrap();
        let corp_code = item.get("corpCode").unwrap().as_str().unwrap();
        let price_comma = price.to_formatted_string(&Locale::en);

        let response = client
            .post(format!(
                "https://{}.ezwel.com/cuser/mypage/offlinecard/ajax/offCardUseInsertAjax.ez",
                company
            ))
            .form(&[
                ("channelType", "1001"),
                ("clientCd", company),
                ("userKey", user_key),
                ("cardType", card_type),
                ("isOffCardClosed", "false"),
                ("usableExPoint0", "0"),
                ("usableExPoint1", "0"),
                ("usableExPoint2", "0"),
                ("usableExPoint3", "0"),
                ("usableExPoint4", "0"),
                ("checkIdx", "0"),
                ("keyCode0", &key_code),
                ("processNum0", process_num),
                ("useDate0", use_date),
                ("usePrice0", &price.to_string()),
                ("cardType0", card_type),
                ("quota0", "00"),
                ("storeCode0", store_code),
                ("storeName0", store),
                ("storeKind0", store_kind),
                ("storeKname0", kind),
                ("levelCd0", level_cd),
                ("corpCode0", corp_code),
                ("bpreqPrice0", &price.to_string()),
                ("spreqPrice0", "0"),
                ("requestPrice0", &price_comma),
                ("reqExPrice0", "0"),
                ("usableExPoint0", "0"),
            ])
            .send()
            .await?;

        println!("{}", response.text().await?);
    }

    Ok(())
}
