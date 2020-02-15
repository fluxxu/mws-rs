macro_rules! str_enum {
  (
    $(#[doc = $doc:expr])*
    pub enum $name:ident { $($item:tt)* }
  ) => {
    str_enum!(
      $(#[doc = $doc])*
      ~
      $name, $($item)*
    );
  };

  (
    $(#[doc = $doc:expr])*
    ~
    $name:ident, $($item:tt),*,
  ) => {
    str_enum!(
      $(#[doc = $doc])*
      ~
      $name, $($item)*
    );
  };

  (
    $(#[doc = $doc:expr])*
    ~
    $name:ident, $($item:tt)*
  ) => {
    $(#[doc = $doc])*
    #[allow(non_camel_case_types)]
    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub enum $name {
      $(
        $item,
      )*
      UnknownValue(String)
    }

    impl $name {
      pub fn to_string(&self) -> String {
        self.as_ref().to_owned()
      }
    }

    impl AsRef<str> for $name {
      fn as_ref(&self) -> &str {
        match *self {
          $(
            $name::$item => stringify!($item),
          )*
          $name::UnknownValue(ref v) => v.as_ref(),
        }
      }
    }

    impl<'a> From<&'a str> for $name {
      fn from(v: &'a str) -> Self {
        match v {
          $(
            stringify!($item) => $name::$item,
          )*
          _ => $name::UnknownValue(v.to_owned()),
        }
      }
    }

    impl ::std::ops::Deref for $name {
      type Target = str;

      fn deref(&self) -> &str {
        self.as_ref()
      }
    }

    impl<T:AsRef<str>> PartialEq<T> for $name {
      fn eq(&self, other: &T) -> bool {
        self.as_ref() == other.as_ref()
      }
    }

    impl Default for $name {
      fn default() -> Self {
        $name::UnknownValue("".to_string())
      }
    }

    impl ::std::str::FromStr for $name {
      type Err = ::std::io::Error;
      fn from_str(s: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok($name::from(s))
      }
    }

    impl Into<String> for $name {
      fn into(self) -> String {
        match self {
          $(
            $name::$item => stringify!($item).to_string(),
          )*
          $name::UnknownValue(v) => v,
        }
      }
    }

    impl ::SerializeMwsParams for $name {
      fn serialize_mws_params(&self, ctx: &::types::SerializeMwsParamsContext, pairs: &mut Vec<(String, String)>) {
        self.as_ref().serialize_mws_params(ctx, pairs);
      }
    }

    impl<S> ::xmlhelper::decode::FromXmlStream<S> for $name
    where S: ::xmlhelper::decode::XmlEventStream
    {
      fn from_xml(s: &mut S) -> ::result::MwsResult<Self> {
        use xmlhelper::decode::characters;
        characters(s)
      }
    }
  };
}

macro_rules! string_map_enum {
  (
    $(#[doc = $doc:expr])*
    pub enum $name:ident {
      $($variant:ident = $value:expr),+
      $(,)*
    }
  ) => (
    $(#[doc = $doc])*
    #[allow(non_camel_case_types)]
    #[derive(Clone, Debug, Serialize)]
    pub enum $name {
      $($variant,)*
      UnknownValue(String)
    }

    impl ::std::str::FromStr for $name {
      type Err = ::std::io::Error;
      fn from_str(s: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(match s {
          $(
            $value => $name::$variant,
          )*
          _ => $name::UnknownValue(s.to_owned()),
        })
      }
    }

    impl ::std::ops::Deref for $name {
      type Target = str;

      fn deref(&self) -> &str {
        self.as_ref()
      }
    }

    impl AsRef<str> for $name {
      fn as_ref(&self) -> &str {
        match *self {
          $(
            $name::$variant => $value,
          )*
          $name::UnknownValue(ref v) => v.as_ref(),
        }
      }
    }

    impl<T:AsRef<str>> PartialEq<T> for $name {
      fn eq(&self, other: &T) -> bool {
        self.as_ref() == other.as_ref()
      }
    }

    impl Default for $name {
      fn default() -> Self {
        $name::UnknownValue("".to_owned())
      }
    }

    impl ::SerializeMwsParams for $name {
      fn serialize_mws_params(&self, ctx: &::types::SerializeMwsParamsContext, pairs: &mut Vec<(String, String)>) {
        self.as_ref().serialize_mws_params(ctx, pairs);
      }
    }

    impl<S> ::xmlhelper::decode::FromXmlStream<S> for $name
    where S: ::xmlhelper::decode::XmlEventStream
    {
      fn from_xml(s: &mut S) -> ::result::MwsResult<Self> {
        use xmlhelper::decode::characters;
        characters(s)
      }
    }
  )
}

macro_rules! response_envelope_type {
  ($name:ident < $payload_ty:ty > , $response_tag:expr, $result_tag:expr) => {
    #[derive(Default)]
    struct $name(::types::ResponseEnvelope<$payload_ty>);

    impl $name {
      fn into_inner(self) -> $payload_ty {
        self.0.payload
      }
    }

    impl<S> ::xmlhelper::decode::FromXmlStream<S> for $name
    where
      S: ::xmlhelper::decode::XmlEventStream,
    {
      fn from_xml(s: &mut S) -> ::result::MwsResult<Self> {
        use xmlhelper::decode::{characters, element, fold_elements, start_document};
        start_document(s)?;
        element(s, $response_tag, |s| {
          fold_elements(
            s,
            ::types::ResponseEnvelope::<$payload_ty>::default(),
            |s, response| {
              if s.local_name() == $result_tag {
                response.payload = ::xmlhelper::decode::FromXmlStream::from_xml(s)?;
              } else if s.local_name() == "ResponseMetadata" {
                response.request_id = element(s, "RequestId", |s| characters(s))?;
              }
              Ok(())
            },
          )
        })
        .map($name)
      }
    }
  };
}

macro_rules! response_envelope_batch_type {
  ($name:ident < $payload_ty:ty > , $response_tag:expr, $result_tag:expr) => {
    #[derive(Default)]
    struct $name(::types::ResponseEnvelopeBatch<$payload_ty>);

    impl $name {
      fn into_inner(self) -> Vec<$payload_ty> {
        self.0.payload
      }
    }

    impl<S> ::xmlhelper::decode::FromXmlStream<S> for $name
    where
      S: ::xmlhelper::decode::XmlEventStream,
    {
      fn from_xml(s: &mut S) -> ::result::MwsResult<Self> {
        use xmlhelper::decode::{characters, element, fold_elements, start_document};
        start_document(s)?;
        element(s, $response_tag, |s| {
          fold_elements(
            s,
            ::types::ResponseEnvelopeBatch::<$payload_ty>::default(),
            |s, response| {
              if s.local_name() == $result_tag {
                response
                  .payload
                  .push(::xmlhelper::decode::FromXmlStream::from_xml(s)?);
              } else if s.local_name() == "ResponseMetadata" {
                response.request_id = element(s, "RequestId", |s| characters(s))?;
              }
              Ok(())
            },
          )
        })
        .map($name)
      }
    }
  };
}
