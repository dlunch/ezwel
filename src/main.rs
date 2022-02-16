use anyhow::{anyhow, Result};
use chrono::{Datelike, Local, TimeZone};
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
    let today = Local::today();
    let today_str1 = today.format("%Y-%m-%d").to_string();
    let today_str = today.format("%Y%m%d").to_string();

    let first_date = Local.ymd(today.year(), 1, 1);
    let first_date_str1 = first_date.format("%Y-%m-%d").to_string();
    let first_date_str = first_date.format("%Y%m%d").to_string();

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
            ("startDt1", &first_date_str1),
            ("endDt1", &today_str1),
            ("startDt", &first_date_str),
            ("endDt", &today_str),
        ])
        .send()
        .await?;

    // [{"successYn":"Y","totalCnt":"3"},{"channelType":"1001","filtering":"","spUseYn":"N","agreeYn":"","callDt":"","sumRctPrice":0,"noAllowedStore":"","deptNm":"","bpUseYn":"Y","userKey":"1008307884","branchNm":"","rctPriceC":0,"rctPriceB":0,"endDt":"","storeTel":"","spreqPrice":0,"useDate":"20220108","refundType":"","userNm":"","statusNm":"","useYn":"","cmsCardSeq":"","medicalSeq":"","prgCode":"","fromNm":"","limitPrice":"","storeType":"","spresPrice":0,"selectSeq":"","deptCd":"","cancelYn":"","totalRemainPrice":0,"revSeq":"","cmsYn":"","storeOname":"","rrnSub":"","sumPayAmount":0,"mgrId":"","requestYn":"","checkYn2":"","fromMail":"","checkYn1":"","clientDeductType":"","GET_PREFIX":"get","orderCancel":"","note":"","clientCd":"sendbird","prvCd":"","teamNm":"","partCd":"","empNum":"","usePrice":14690,"updateResultPrice":"","medicalRefundPoint":0,"rctPrice":0,"spPointShortNm":"","moveinDd":"","resultPrice":0,"statusMsg":"","storeAddr":"","requestPrice":0,"tooltipMsg":"","confirmIp":"","storeKname":"할인점","controlSpUnit":"","refundDay":"","isCancelYn":"","isPrint":"","bankOrder":"","writeDate":"","ctiCardType":"","noWelfareCardNm":"","storeKind":"3102","payAmountC":0,"isQuota":"","payAmountB":0,"mailYn":"","userType":"","linkCheckType":"","gradeCd":"","writeIp":"","branchCd":"","isPaging":"Y","agreeDt":"","menuNm":"","corpCode":"13727","bankNm":"","serialVersionUID":4669330459912837323,"sPReqPriceSum":0,"bankNum":"","partNm":"","revTypeCd":"","status":"1001","sPResPriceSum":0,"noWelfareCard":"","display":"","confirmDate":"","cardType":"1019","revType":"","processNum":"70054717215","storeCode":"1448503416","cardTypeNm13":"","mobile":"","sex":"","autoYn":"","offlineCardNum":"","nextKey":"N","showYn":"Y","exoffUseYn":"","menuFlag":"2","cardNm":"롯데","cardDiv":"1","keyCode":"2022011014228001_____00000088847","quota":"00","mgrType":"","startDt":"","bPResPriceSum":0,"cardLastNo":"","bPReqPriceSum":0,"bpresPrice":0,"payAmount":0,"rrn":"","storeName":"롯데마트 판교점","bankCd":"","teamCd":"","bpreqPrice":0,"controlSpPoint":"","cardTypeNm":"","gradeNm":"","commText":"","commCd":"","fileCnt":0,"levelCd":"1045","rctDt":"","userDeductType":"","currentPage":"1","resultDate":"","deducted":""},{"channelType":"1001","filtering":"","spUseYn":"N","agreeYn":"","callDt":"","sumRctPrice":0,"noAllowedStore":"","deptNm":"","bpUseYn":"Y","userKey":"1008307884","branchNm":"","rctPriceC":0,"rctPriceB":0,"endDt":"","storeTel":"","spreqPrice":0,"useDate":"20220110","refundType":"","userNm":"","statusNm":"","useYn":"","cmsCardSeq":"","medicalSeq":"","prgCode":"","fromNm":"","limitPrice":"","storeType":"","spresPrice":0,"selectSeq":"","deptCd":"","cancelYn":"","totalRemainPrice":0,"revSeq":"","cmsYn":"","storeOname":"","rrnSub":"","sumPayAmount":0,"mgrId":"","requestYn":"","checkYn2":"","fromMail":"","checkYn1":"","clientDeductType":"","GET_PREFIX":"get","orderCancel":"","note":"","clientCd":"sendbird","prvCd":"","teamNm":"","partCd":"","empNum":"","usePrice":2400,"updateResultPrice":"","medicalRefundPoint":0,"rctPrice":0,"spPointShortNm":"","moveinDd":"","resultPrice":0,"statusMsg":"","storeAddr":"","requestPrice":0,"tooltipMsg":"","confirmIp":"","storeKname":"PG(온라인)","controlSpUnit":"","refundDay":"","isCancelYn":"","isPrint":"","bankOrder":"","writeDate":"","ctiCardType":"","noWelfareCardNm":"","storeKind":"3306","payAmountC":0,"isQuota":"","payAmountB":0,"mailYn":"","userType":"","linkCheckType":"","gradeCd":"","writeIp":"","branchCd":"","isPaging":"Y","agreeDt":"","menuNm":"","corpCode":"13727","bankNm":"","serialVersionUID":4669330459912837323,"sPReqPriceSum":0,"bankNum":"","partNm":"","revTypeCd":"","status":"1001","sPResPriceSum":0,"noWelfareCard":"","display":"","confirmDate":"","cardType":"1019","revType":"","processNum":"70054717215","storeCode":"1208765763","cardTypeNm13":"","mobile":"","sex":"","autoYn":"","offlineCardNum":"","nextKey":"N","showYn":"Y","exoffUseYn":"","menuFlag":"2","cardNm":"롯데","cardDiv":"1","keyCode":"2022011113213002_____00000191018","quota":"00","mgrType":"","startDt":"","bPResPriceSum":0,"cardLastNo":"","bPReqPriceSum":0,"bpresPrice":0,"payAmount":0,"rrn":"","storeName":"(주)우아한형제들","bankCd":"","teamCd":"","bpreqPrice":0,"controlSpPoint":"","cardTypeNm":"","gradeNm":"","commText":"","commCd":"","fileCnt":0,"levelCd":"1045","rctDt":"","userDeductType":"","currentPage":"1","resultDate":"","deducted":""},{"channelType":"1001","filtering":"","spUseYn":"N","agreeYn":"","callDt":"","sumRctPrice":0,"noAllowedStore":"","deptNm":"","bpUseYn":"Y","userKey":"1008307884","branchNm":"","rctPriceC":0,"rctPriceB":0,"endDt":"","storeTel":"","spreqPrice":0,"useDate":"20220111","refundType":"","userNm":"","statusNm":"","useYn":"","cmsCardSeq":"","medicalSeq":"","prgCode":"","fromNm":"","limitPrice":"","storeType":"","spresPrice":0,"selectSeq":"","deptCd":"","cancelYn":"","totalRemainPrice":0,"revSeq":"","cmsYn":"","storeOname":"","rrnSub":"","sumPayAmount":0,"mgrId":"","requestYn":"","checkYn2":"","fromMail":"","checkYn1":"","clientDeductType":"","GET_PREFIX":"get","orderCancel":"","note":"","clientCd":"sendbird","prvCd":"","teamNm":"","partCd":"","empNum":"","usePrice":22020,"updateResultPrice":"","medicalRefundPoint":0,"rctPrice":0,"spPointShortNm":"","moveinDd":"","resultPrice":0,"statusMsg":"","storeAddr":"","requestPrice":0,"tooltipMsg":"","confirmIp":"","storeKname":"편의점","controlSpUnit":"","refundDay":"","isCancelYn":"","isPrint":"","bankOrder":"","writeDate":"","ctiCardType":"","noWelfareCardNm":"","storeKind":"3105","payAmountC":0,"isQuota":"","payAmountB":0,"mailYn":"","userType":"","linkCheckType":"","gradeCd":"","writeIp":"","branchCd":"","isPaging":"Y","agreeDt":"","menuNm":"","corpCode":"13727","bankNm":"","serialVersionUID":4669330459912837323,"sPReqPriceSum":0,"bankNum":"","partNm":"","revTypeCd":"","status":"1001","sPResPriceSum":0,"noWelfareCard":"","display":"","confirmDate":"","cardType":"1019","revType":"","processNum":"70054717215","storeCode":"6902900105","cardTypeNm13":"","mobile":"","sex":"","autoYn":"","offlineCardNum":"","nextKey":"N","showYn":"Y","exoffUseYn":"","menuFlag":"2","cardNm":"롯데","cardDiv":"1","keyCode":"2022011213215002_____00000122799","quota":"00","mgrType":"","startDt":"","bPResPriceSum":0,"cardLastNo":"","bPReqPriceSum":0,"bpresPrice":0,"payAmount":0,"rrn":"","storeName":"GS25 양재언남점","bankCd":"","teamCd":"","bpreqPrice":0,"controlSpPoint":"","cardTypeNm":"","gradeNm":"","commText":"","commCd":"","fileCnt":0,"levelCd":"1045","rctDt":"","userDeductType":"","currentPage":"1","resultDate":"","deducted":""}]
    let response = response.json::<serde_json::Value>().await?;

    let response_array = response.as_array().unwrap();
    let success = response_array[0]["successYn"].as_str().unwrap();

    if success != "Y" {
        return Err(anyhow!("Failed to get result"));
    }

    if response_array.len() == 1 {
        return Ok(());
    }

    let card_type = response_array[1].get("cardType").unwrap().as_str().unwrap();

    let mut form_data = vec![
        ("channelType".into(), "1001".into()),
        ("clientCd".into(), company.to_owned()),
        ("userKey".into(), user_key.to_owned()),
        ("cardType".into(), card_type.to_owned()),
        ("isOffCardClosed".into(), "false".into()),
        ("usableExPoint0".into(), "0".into()),
        ("usableExPoint1".into(), "0".into()),
        ("usableExPoint2".into(), "0".into()),
        ("usableExPoint3".into(), "0".into()),
        ("usableExPoint4".into(), "0".into()),
    ];

    for (i, item) in response_array.iter().skip(1).enumerate() {
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

        form_data.push(("checkIdx".into(), i.to_string()));
        form_data.push((format!("keyCode{}", i), key_code));
        form_data.push((format!("processNum{}", i), process_num.to_owned()));
        form_data.push((format!("useDate{}", i), use_date.to_owned()));
        form_data.push((format!("usePrice{}", i), price.to_string()));
        form_data.push((format!("cardType{}", i), card_type.to_owned()));
        form_data.push((format!("quota{}", i), "00".into()));
        form_data.push((format!("storeCode{}", i), store_code.to_owned()));
        form_data.push((format!("storeName{}", i), store.to_owned()));
        form_data.push((format!("storeKind{}", i), store_kind.to_owned()));
        form_data.push((format!("storeKname{}", i), kind.to_owned()));
        form_data.push((format!("levelCd{}", i), level_cd.to_owned()));
        form_data.push((format!("corpCode{}", i), corp_code.to_owned()));
        form_data.push((format!("bpreqPrice{}", i), price.to_string()));
        form_data.push((format!("spreqPrice{}", i), "0".into()));
        form_data.push((format!("requestPrice{}", i), price_comma));
        form_data.push((format!("reqExPrice{}", i), "0".into()));
        form_data.push((format!("usableExPoint{}", i), "0".into()));
    }
    let response = client
        .post(format!(
            "https://{}.ezwel.com/cuser/mypage/offlinecard/ajax/offCardUseInsertAjax.ez",
            company
        ))
        .form(&form_data)
        .send()
        .await?;
    println!("{}", response.text().await?);

    Ok(())
}
