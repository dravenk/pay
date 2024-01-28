use openssl::rsa::Rsa;
use openssl::x509::X509;
use pay::wechat::WeChatPay;
use reqwest::Client;
use serde_json::json;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 「商户API证书」的「证书序列号」
    // 从本地文件中加载「微信支付平台证书」(可使用证书下载工具得到），用来验证微信支付应答的签名
    let merchant_id = "190000****".to_string();
    let merchant_certificate_serial = "3775B6A45ACD588826D15E583A95F5DD********".to_string();

    let mut merchant_private_key_file = File::open("/path/to/merchant/apiclient_key.pem").unwrap();
    let mut merchant_private_key = vec![];
    merchant_private_key_file
        .read_to_end(&mut merchant_private_key)
        .unwrap();
    let merchant_private_key_instance = Rsa::private_key_from_pem(&merchant_private_key).unwrap();

    let mut platform_certificate_file = File::open("/path/to/wechatpay/cert.pem").unwrap();
    let mut platform_certificate = vec![];
    platform_certificate_file
        .read_to_end(&mut platform_certificate)
        .unwrap();
    let platform_public_key_instance = X509::from_pem(&platform_certificate)
        .unwrap()
        .public_key()
        .unwrap();

    let platform_certificate_serial = X509::from_pem(&platform_certificate)
        .unwrap()
        .serial_number()
        .to_bn()
        .expect("REASON");
    let mut certs = HashMap::new();
    certs.insert(
        platform_certificate_serial.to_string(),
        platform_public_key_instance,
    );


    let instance = WeChatPay::new(
        merchant_id,
        merchant_certificate_serial,
        merchant_private_key_instance,
        certs,
    );

    let client = Client::new();

    let payment = json!({
        "mchid": "1900006XXX",
        "out_trade_no": "native12177525012014070332333",
        "appid": "wxdace645e0bc2cXXX",
        "description": "Image形象店-深圳腾大-QQ公仔",
        "notify_url": "https://weixin.qq.com/",
        "amount": {
            "total": 1,
            "currency": "CNY"
        }
    });

    let resp = client
        .post("https://api.mch.weixin.qq.com/v3/pay/transactions/native")
        .json(&payment)
        .send()
        .await?;

    let body = resp.text().await?;

    println!("{}", body);

    Ok(())
}
