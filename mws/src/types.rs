use chrono::{DateTime, TimeZone};

pub trait ToIso8601 {
  fn to_iso8601(&self) -> String;
}

impl<Tz: TimeZone> ToIso8601 for DateTime<Tz> {
  fn to_iso8601(&self) -> String {
    self.naive_utc().format("%Y-%m-%dT%H:%M:%SZ").to_string()
  }
}

pub trait SerializeMwsParams {
  fn serialize_mws_params(&self, path: &str, include_name: bool, pairs: &mut Vec<(String, String)>);
  fn into_mws_params(self) -> Vec<(String, String)>
  where
    Self: Sized,
  {
    let mut pairs = vec![];
    self.serialize_mws_params("", true, &mut pairs);
    pairs
      .into_iter()
      .filter(|(_, ref v)| !v.is_empty())
      .collect()
  }
}

macro_rules! impl_serialize_mws_params_to_string {
  (& $ty:ty) => {
    impl<'a> SerializeMwsParams for &'a $ty {
      fn serialize_mws_params(
        &self,
        path: &str,
        _include_name: bool,
        pairs: &mut Vec<(String, String)>,
      ) {
        pairs.push((path.to_string(), self.to_string()))
      }
    }
  };

  ($ty:ty) => {
    impl SerializeMwsParams for $ty {
      fn serialize_mws_params(
        &self,
        path: &str,
        _include_name: bool,
        pairs: &mut Vec<(String, String)>,
      ) {
        pairs.push((path.to_string(), self.to_string()))
      }
    }
  };
}

impl_serialize_mws_params_to_string!(i32);
impl_serialize_mws_params_to_string!(i64);
impl_serialize_mws_params_to_string!(String);
impl_serialize_mws_params_to_string!(&str);
impl_serialize_mws_params_to_string!(bool);

impl<T> SerializeMwsParams for Vec<T>
where
  T: SerializeMwsParams,
{
  fn serialize_mws_params(
    &self,
    path: &str,
    _include_name: bool,
    pairs: &mut Vec<(String, String)>,
  ) {
    for (i, v) in self.iter().enumerate() {
      let path = format!("{}.{}", path, i + 1);
      v.serialize_mws_params(&path, true, pairs);
    }
  }
}

impl<T> SerializeMwsParams for Option<T>
where
  T: SerializeMwsParams,
{
  fn serialize_mws_params(
    &self,
    path: &str,
    _include_name: bool,
    pairs: &mut Vec<(String, String)>,
  ) {
    if let &Some(ref v) = self {
      v.serialize_mws_params(&path, false, pairs);
    }
  }
}

impl<Tz: TimeZone> SerializeMwsParams for DateTime<Tz> {
  fn serialize_mws_params(
    &self,
    path: &str,
    _include_name: bool,
    pairs: &mut Vec<(String, String)>,
  ) {
    pairs.push((path.to_string(), self.to_iso8601()))
  }
}

#[derive(Default)]
pub struct GenericResponse<T: Default> {
  pub payload: T,
  pub request_id: String,
}
