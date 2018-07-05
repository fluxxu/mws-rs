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
  };
}

macro_rules! string_map_enum {
  (
    pub enum $name:ident { 
      $($variant:ident = $value:expr),+
      $(,)*
    }
  ) => (
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
  )
}
