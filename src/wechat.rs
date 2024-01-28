use std::collections::HashMap;

use openssl::{
    pkey::{PKey, Private, Public},
    rsa::Rsa,
};

pub struct WeChatPay {
    pub mchid: String,
    pub serial: String,
    pub private_key: Rsa<Private>,
    pub certs: HashMap<String, PKey<Public>>,
}

impl WeChatPay {
    pub fn new(
        mchid: String,
        serial: String,
        private_key: Rsa<Private>,
        certs: HashMap<String, PKey<Public>>,
    ) -> Self {
        Self {
            mchid,
            serial,
            private_key,
            certs,
        }
    }
}

#[derive(Debug)]
pub struct Amount {
    pub total: i32,
    pub currency: String,
}
