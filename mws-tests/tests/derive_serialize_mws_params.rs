#[macro_use]
extern crate mws_derive;
extern crate mws;

use mws::{SerializeMwsParams, SerializeMwsParamsContext};

#[test]
fn derive_struct() {
  #[derive(SerializeMwsParams)]
  struct S {
    a: String,
    b: i32,
  }

  let pairs = S {
    a: "value of a".to_string(),
    b: 888,
  }.into_mws_params();

  assert_eq!(
    pairs,
    vec![
      ("a".to_owned(), "value of a".to_owned()),
      ("b".to_owned(), "888".to_owned()),
    ]
  )
}

#[test]
fn derive_option_field() {
  #[derive(SerializeMwsParams)]
  struct S {
    v: Option<i32>,
  }

  assert_eq!(S { v: None }.into_mws_params(), vec![]);
  assert_eq!(
    S { v: Some(1) }.into_mws_params(),
    vec![("v".to_string(), "1".to_string())]
  );
}

#[test]
fn derive_nested_struct() {
  #[derive(SerializeMwsParams)]
  struct S {
    a: String,
    b: i32,
    inner: SS,
  }

  #[derive(SerializeMwsParams)]
  struct SS {
    c: String,
    d: i32,
  }

  let pairs = S {
    a: "value of a".to_string(),
    b: 888,
    inner: SS {
      c: "value of c".to_string(),
      d: 999,
    },
  }.into_mws_params();

  assert_eq!(
    pairs,
    vec![
      ("a".to_owned(), "value of a".to_owned()),
      ("b".to_owned(), "888".to_owned()),
      ("inner.c".to_owned(), "value of c".to_owned()),
      ("inner.d".to_owned(), "999".to_owned()),
    ]
  )
}

#[test]
fn derive_vec() {
  #[derive(SerializeMwsParams)]
  struct S {
    v: i32,
    items: Vec<SS>,
  }

  #[derive(SerializeMwsParams)]
  struct SS {
    v: i32,
  }

  let pairs = S {
    v: 1,
    items: vec![SS { v: 111 }, SS { v: 222 }, SS { v: 333 }, SS { v: 444 }],
  }.into_mws_params();

  assert_eq!(
    pairs,
    vec![
      ("v".to_owned(), "1".to_owned()),
      ("items.member.1.v".to_owned(), "111".to_owned()),
      ("items.member.2.v".to_owned(), "222".to_owned()),
      ("items.member.3.v".to_owned(), "333".to_owned()),
      ("items.member.4.v".to_owned(), "444".to_owned()),
    ]
  )
}

#[test]
fn derive_vec_config() {
  #[derive(SerializeMwsParams)]
  struct S {
    v: i32,
    #[mws_param(list_item_type_name = "Item")]
    items: Vec<SS>,
  }

  #[derive(SerializeMwsParams)]
  struct SS {
    #[mws_param(list_item_type_name = "Inner")]
    vv: Vec<i32>,
  }

  let pairs = S {
    v: 1,
    items: vec![SS { vv: vec![111] }],
  }.into_mws_params();

  assert_eq!(
    pairs,
    vec![
      ("v".to_owned(), "1".to_owned()),
      ("items.Item.1.vv.Inner.1".to_owned(), "111".to_owned()),
    ]
  )
}
