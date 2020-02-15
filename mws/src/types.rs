use chrono::{DateTime, TimeZone};

pub trait ToIso8601 {
  fn to_iso8601(&self) -> String;
}

impl<Tz: TimeZone> ToIso8601 for DateTime<Tz> {
  fn to_iso8601(&self) -> String {
    self.naive_utc().format("%Y-%m-%dT%H:%M:%SZ").to_string()
  }
}

#[derive(Default, Clone)]
pub struct SerializeMwsParamsFieldConfig {
  /// When serialize list, MWS uses `ListFieldName.ItemTypeName.Index` as key
  pub list_item_type_name: Option<&'static str>,
}

#[derive(Default, Clone)]
pub struct SerializeMwsParamsContext {
  pub path: Option<String>,
  pub field_config: SerializeMwsParamsFieldConfig,
}

pub trait SerializeMwsParams {
  fn serialize_mws_params(
    &self,
    ctx: &SerializeMwsParamsContext,
    pairs: &mut Vec<(String, String)>,
  );
  fn into_mws_params(self) -> Vec<(String, String)>
  where
    Self: Sized,
  {
    let mut pairs = vec![];
    let ctx = Default::default();
    self.serialize_mws_params(&ctx, &mut pairs);
    pairs
      .into_iter()
      .filter(|(_, ref v)| !v.is_empty())
      .collect()
  }
}

impl SerializeMwsParams for Vec<(String, String)> {
  fn serialize_mws_params(
    &self,
    _ctx: &SerializeMwsParamsContext,
    pairs: &mut Vec<(String, String)>,
  ) {
    pairs.append(&mut self.clone());
  }
}

impl SerializeMwsParams for () {
  fn serialize_mws_params(
    &self,
    _ctx: &SerializeMwsParamsContext,
    _pairs: &mut Vec<(String, String)>,
  ) {
  }
}

macro_rules! impl_serialize_mws_params_to_string {
  (& $ty:ty) => {
    impl<'a> SerializeMwsParams for &'a $ty {
      fn serialize_mws_params(
        &self,
        ctx: &SerializeMwsParamsContext,
        pairs: &mut Vec<(String, String)>,
      ) {
        let value = self.to_string();
        if !value.is_empty() {
          pairs.push((
            ctx.path.clone().expect("mws param type should be struct"),
            value,
          ))
        }
      }
    }
  };

  ($ty:ty) => {
    impl SerializeMwsParams for $ty {
      fn serialize_mws_params(
        &self,
        ctx: &SerializeMwsParamsContext,
        pairs: &mut Vec<(String, String)>,
      ) {
        pairs.push((
          ctx.path.clone().expect("mws param type should be struct"),
          self.to_string(),
        ))
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
    ctx: &SerializeMwsParamsContext,
    pairs: &mut Vec<(String, String)>,
  ) {
    let path = ctx.path.clone().expect("mws param type should be struct");
    for (i, v) in self.iter().enumerate() {
      let field_path = match ctx.field_config.list_item_type_name.clone() {
        None => format!("{}.{}", path, i + 1),
        Some(item_type_name) => format!("{}.{}.{}", path, item_type_name, i + 1),
      };
      v.serialize_mws_params(
        &SerializeMwsParamsContext {
          path: Some(field_path),
          ..ctx.clone()
        },
        pairs,
      );
    }
  }
}

impl<T> SerializeMwsParams for Option<T>
where
  T: SerializeMwsParams,
{
  fn serialize_mws_params(
    &self,
    ctx: &SerializeMwsParamsContext,
    pairs: &mut Vec<(String, String)>,
  ) {
    if let &Some(ref v) = self {
      v.serialize_mws_params(ctx, pairs);
    }
  }
}

impl<Tz: TimeZone> SerializeMwsParams for DateTime<Tz> {
  fn serialize_mws_params(
    &self,
    ctx: &SerializeMwsParamsContext,
    pairs: &mut Vec<(String, String)>,
  ) {
    let path = ctx.path.clone().expect("mws param type should be struct");
    pairs.push((path.to_string(), self.to_iso8601()))
  }
}

#[derive(Debug, Default)]
pub struct ResponseEnvelope<T: Default> {
  pub payload: T,
  pub request_id: String,
}

#[derive(Debug, Default)]
pub struct ResponseEnvelopeBatch<T: Default> {
  pub payload: Vec<T>,
  pub request_id: String,
}
