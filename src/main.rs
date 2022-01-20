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

    // [{"successYn":"Y","totalCnt":"3"},{"channelType":"1001","filtering":"","spUseYn":"N","agreeYn":"","callDt":"","sumRctPrice":0,"noAllowedStore":"","deptNm":"","bpUseYn":"Y","userKey":"1008307884","branchNm":"","rctPriceC":0,"rctPriceB":0,"endDt":"","storeTel":"","spreqPrice":0,"useDate":"20220108","refundType":"","userNm":"","statusNm":"","useYn":"","cmsCardSeq":"","medicalSeq":"","prgCode":"","fromNm":"","limitPrice":"","storeType":"","spresPrice":0,"selectSeq":"","deptCd":"","cancelYn":"","totalRemainPrice":0,"revSeq":"","cmsYn":"","storeOname":"","rrnSub":"","sumPayAmount":0,"mgrId":"","requestYn":"","checkYn2":"","fromMail":"","checkYn1":"","clientDeductType":"","GET_PREFIX":"get","orderCancel":"","note":"","clientCd":"sendbird","prvCd":"","teamNm":"","partCd":"","empNum":"","usePrice":14690,"updateResultPrice":"","medicalRefundPoint":0,"rctPrice":0,"spPointShortNm":"","moveinDd":"","resultPrice":0,"statusMsg":"","storeAddr":"","requestPrice":0,"tooltipMsg":"","confirmIp":"","storeKname":"할인점","controlSpUnit":"","refundDay":"","isCancelYn":"","isPrint":"","bankOrder":"","writeDate":"","ctiCardType":"","noWelfareCardNm":"","storeKind":"3102","payAmountC":0,"isQuota":"","payAmountB":0,"mailYn":"","userType":"","linkCheckType":"","gradeCd":"","writeIp":"","branchCd":"","isPaging":"Y","agreeDt":"","menuNm":"","corpCode":"13727","bankNm":"","serialVersionUID":4669330459912837323,"sPReqPriceSum":0,"bankNum":"","partNm":"","revTypeCd":"","status":"1001","sPResPriceSum":0,"noWelfareCard":"","display":"","confirmDate":"","cardType":"1019","revType":"","processNum":"70054717215","storeCode":"1448503416","cardTypeNm13":"","mobile":"","sex":"","autoYn":"","offlineCardNum":"","nextKey":"N","showYn":"Y","exoffUseYn":"","menuFlag":"2","cardNm":"롯데","cardDiv":"1","keyCode":"2022011014228001_____00000088847","quota":"00","mgrType":"","startDt":"","bPResPriceSum":0,"cardLastNo":"","bPReqPriceSum":0,"bpresPrice":0,"payAmount":0,"rrn":"","storeName":"롯데마트 판교점","bankCd":"","teamCd":"","bpreqPrice":0,"controlSpPoint":"","cardTypeNm":"","gradeNm":"","commText":"","commCd":"","fileCnt":0,"levelCd":"1045","rctDt":"","userDeductType":"","currentPage":"1","resultDate":"","deducted":""},{"channelType":"1001","filtering":"","spUseYn":"N","agreeYn":"","callDt":"","sumRctPrice":0,"noAllowedStore":"","deptNm":"","bpUseYn":"Y","userKey":"1008307884","branchNm":"","rctPriceC":0,"rctPriceB":0,"endDt":"","storeTel":"","spreqPrice":0,"useDate":"20220110","refundType":"","userNm":"","statusNm":"","useYn":"","cmsCardSeq":"","medicalSeq":"","prgCode":"","fromNm":"","limitPrice":"","storeType":"","spresPrice":0,"selectSeq":"","deptCd":"","cancelYn":"","totalRemainPrice":0,"revSeq":"","cmsYn":"","storeOname":"","rrnSub":"","sumPayAmount":0,"mgrId":"","requestYn":"","checkYn2":"","fromMail":"","checkYn1":"","clientDeductType":"","GET_PREFIX":"get","orderCancel":"","note":"","clientCd":"sendbird","prvCd":"","teamNm":"","partCd":"","empNum":"","usePrice":2400,"updateResultPrice":"","medicalRefundPoint":0,"rctPrice":0,"spPointShortNm":"","moveinDd":"","resultPrice":0,"statusMsg":"","storeAddr":"","requestPrice":0,"tooltipMsg":"","confirmIp":"","storeKname":"PG(온라인)","controlSpUnit":"","refundDay":"","isCancelYn":"","isPrint":"","bankOrder":"","writeDate":"","ctiCardType":"","noWelfareCardNm":"","storeKind":"3306","payAmountC":0,"isQuota":"","payAmountB":0,"mailYn":"","userType":"","linkCheckType":"","gradeCd":"","writeIp":"","branchCd":"","isPaging":"Y","agreeDt":"","menuNm":"","corpCode":"13727","bankNm":"","serialVersionUID":4669330459912837323,"sPReqPriceSum":0,"bankNum":"","partNm":"","revTypeCd":"","status":"1001","sPResPriceSum":0,"noWelfareCard":"","display":"","confirmDate":"","cardType":"1019","revType":"","processNum":"70054717215","storeCode":"1208765763","cardTypeNm13":"","mobile":"","sex":"","autoYn":"","offlineCardNum":"","nextKey":"N","showYn":"Y","exoffUseYn":"","menuFlag":"2","cardNm":"롯데","cardDiv":"1","keyCode":"2022011113213002_____00000191018","quota":"00","mgrType":"","startDt":"","bPResPriceSum":0,"cardLastNo":"","bPReqPriceSum":0,"bpresPrice":0,"payAmount":0,"rrn":"","storeName":"(주)우아한형제들","bankCd":"","teamCd":"","bpreqPrice":0,"controlSpPoint":"","cardTypeNm":"","gradeNm":"","commText":"","commCd":"","fileCnt":0,"levelCd":"1045","rctDt":"","userDeductType":"","currentPage":"1","resultDate":"","deducted":""},{"channelType":"1001","filtering":"","spUseYn":"N","agreeYn":"","callDt":"","sumRctPrice":0,"noAllowedStore":"","deptNm":"","bpUseYn":"Y","userKey":"1008307884","branchNm":"","rctPriceC":0,"rctPriceB":0,"endDt":"","storeTel":"","spreqPrice":0,"useDate":"20220111","refundType":"","userNm":"","statusNm":"","useYn":"","cmsCardSeq":"","medicalSeq":"","prgCode":"","fromNm":"","limitPrice":"","storeType":"","spresPrice":0,"selectSeq":"","deptCd":"","cancelYn":"","totalRemainPrice":0,"revSeq":"","cmsYn":"","storeOname":"","rrnSub":"","sumPayAmount":0,"mgrId":"","requestYn":"","checkYn2":"","fromMail":"","checkYn1":"","clientDeductType":"","GET_PREFIX":"get","orderCancel":"","note":"","clientCd":"sendbird","prvCd":"","teamNm":"","partCd":"","empNum":"","usePrice":22020,"updateResultPrice":"","medicalRefundPoint":0,"rctPrice":0,"spPointShortNm":"","moveinDd":"","resultPrice":0,"statusMsg":"","storeAddr":"","requestPrice":0,"tooltipMsg":"","confirmIp":"","storeKname":"편의점","controlSpUnit":"","refundDay":"","isCancelYn":"","isPrint":"","bankOrder":"","writeDate":"","ctiCardType":"","noWelfareCardNm":"","storeKind":"3105","payAmountC":0,"isQuota":"","payAmountB":0,"mailYn":"","userType":"","linkCheckType":"","gradeCd":"","writeIp":"","branchCd":"","isPaging":"Y","agreeDt":"","menuNm":"","corpCode":"13727","bankNm":"","serialVersionUID":4669330459912837323,"sPReqPriceSum":0,"bankNum":"","partNm":"","revTypeCd":"","status":"1001","sPResPriceSum":0,"noWelfareCard":"","display":"","confirmDate":"","cardType":"1019","revType":"","processNum":"70054717215","storeCode":"6902900105","cardTypeNm13":"","mobile":"","sex":"","autoYn":"","offlineCardNum":"","nextKey":"N","showYn":"Y","exoffUseYn":"","menuFlag":"2","cardNm":"롯데","cardDiv":"1","keyCode":"2022011213215002_____00000122799","quota":"00","mgrType":"","startDt":"","bPResPriceSum":0,"cardLastNo":"","bPReqPriceSum":0,"bpresPrice":0,"payAmount":0,"rrn":"","storeName":"GS25 양재언남점","bankCd":"","teamCd":"","bpreqPrice":0,"controlSpPoint":"","cardTypeNm":"","gradeNm":"","commText":"","commCd":"","fileCnt":0,"levelCd":"1045","rctDt":"","userDeductType":"","currentPage":"1","resultDate":"","deducted":""}]
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
        let key_code = item.get("keyCode").unwrap().as_str().unwrap().replace("_", " ");
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
