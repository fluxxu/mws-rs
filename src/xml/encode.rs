//! XML Decode

error_chain! {

}

pub trait XMLEncode {
  type T;

  fn encode(self) -> Result<String>;
}