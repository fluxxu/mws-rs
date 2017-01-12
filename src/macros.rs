macro_rules! str_enum {
  (
    pub enum $name:ident { $($item:tt)* } 
  ) => {
    str_enum!($name, $($item)*);
  };

  (
    $name:ident, $($item:tt),*,
  ) => {
    str_enum!($name, $($item)*);
  };

  (
    $name:ident, $($item:tt)*
  ) => {
    #[allow(non_camel_case_types)]
    #[derive(Debug, PartialEq)]
    pub enum $name {
      $(
        $item,
      )*
      Unknown(String)
    }

    impl AsRef<str> for $name {
      fn as_ref(&self) -> &str {
        match *self {
          $(
            $name::$item => stringify!($item),
          )*
          $name::Unknown(ref v) => v.as_ref(),
        }
      }
    }

    impl<'a> From<&'a str> for $name {
      fn from(v: &'a str) -> Self {
        match v {
          $(
            stringify!($item) => $name::$item,
          )*
          _ => $name::Unknown(v.to_owned()),
        }
      }
    }

    impl Default for $name {
      fn default() -> Self {
        $name::Unknown("".to_string())
      }
    }

    impl ::std::str::FromStr for $name {
      type Err = ::std::io::Error;
      fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok($name::from(s))
      }
    }

    impl Into<String> for $name {
      fn into(self) -> String {
        match self {
          $(
            $name::$item => stringify!($item).to_string(),
          )*
          $name::Unknown(v) => v,
        }
      }
    }
  };
}
