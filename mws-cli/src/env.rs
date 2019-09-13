use std::env;

pub struct Env {
  pub seller_id: String,
  pub region_id: String,
  pub access_key_id: String,
  pub secret_key: String,
  pub auth_token: Option<String>,
}

impl Env {
  pub fn from_env() -> Self {
    Env {
      seller_id: env::var("SellerId").unwrap(),
      region_id: env::var("RegionId").unwrap(),
      access_key_id: env::var("AWSAccessKeyId").unwrap(),
      secret_key: env::var("SecretKey").unwrap(),
      auth_token: env::var("MWSAuthToken").ok(),
    }
  }
}
