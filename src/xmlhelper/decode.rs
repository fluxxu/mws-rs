//! Utilities to parse a XML event stream
//!

use xml::reader::{EventReader, Events, Result as XmlReaderResult, XmlEvent};
pub use xml::name::OwnedName as Name;
pub use xml::attribute::OwnedAttribute as Attribute;
use std::iter::Peekable;
use std::io::Read;
use std::str::FromStr;
use std::fmt::Display;

error_chain! {
  foreign_links {
    Xml(::xml::reader::Error);
  }

  errors {
    UnexpectedEndOfXml(msg: String) {
      description("unexpected end of xml")
      display("unexpected end of xml: {}", msg)
    }

    UnexpectedXmlEvent(expected: String, found: String) {
      description("unexpected xml event")
      display("unexpected xml event: expected '{}', found '{}'", expected, found)
    }

    ParseString(what: String, message: String) {
      description("parse string error")
      display("parse string error: {} : {}", what, message)
    }
  }
}

/// Get next event from stream, run an optional expr if type matches
macro_rules! try_consume_event {
    (
      $stream:ident,
      $pattern:pat
    ) => {{
      match $stream.next() {
        Some(Ok($pattern)) => (),
        Some(Ok(event)) => {
          return Err(ErrorKind::UnexpectedXmlEvent(
            stringify!($pattern).to_string(), format!("{:?}", event)).into())
        },
        Some(Err(e)) => return Err(e.into()),
        None => {
          return Err(ErrorKind::UnexpectedEndOfXml(
            format!("expected event: {}", stringify!($pattern))).into())
        },
      }
    }};

    (
      $stream:ident,
      $pattern:pat => $expr:expr
    ) => {{
      match $stream.next() {
        Some(Ok($pattern)) => $expr,
        Some(Ok(event)) => {
          return Err(ErrorKind::UnexpectedXmlEvent(
            stringify!($pattern).to_string(), format!("{:?}", event)).into())
        },
        Some(Err(e)) => return Err(e.into()),
        None => {
          return Err(ErrorKind::UnexpectedEndOfXml(
            format!("expected event: {}", stringify!($pattern))).into())
          },
      }
    }};

    (
      $stream:ident,
      $(
        $pattern:pat => $expr:expr
      ),*
    ) => {{
      match $stream.next() {
        $(
          Some(Ok($pattern)) => $expr,
        )*
        Some(Ok(event)) => {
          return Err(ErrorKind::UnexpectedXmlEvent(
            stringify!($pattern).to_string(), format!("{:?}", event)).into())
        },
        Some(Err(e)) => return Err(e.into()),
        None => {
          return Err(ErrorKind::UnexpectedEndOfXml(
            format!("expected events: {}", stringify!($($pattern),*)).into()))
        },
      }
    }};
}

pub trait FromXMLStream<S: XmlEventStream>: Sized + Default {
  fn from_xml(stream: &mut S) -> Result<Self>;
}

pub trait XmlEventStream {
  fn next(&mut self) -> Option<XmlReaderResult<XmlEvent>>;
  fn peek(&mut self) -> Option<&XmlReaderResult<XmlEvent>>;
}

impl<R: Read> From<R> for Stream<R> {
  fn from(source: R) -> Stream<R> {
    Stream::new(source)
  }
}

/// Owned stream
pub struct Stream<R: Read>(Peekable<Events<R>>);

impl<R: Read> Stream<R> {
  /// Construct a stream
  pub fn new(read: R) -> Self {
    let config = ::xml::ParserConfig::new()
      .trim_whitespace(true)
      .ignore_comments(true);
    Stream(
      EventReader::new_with_config(read, config)
        .into_iter()
        .peekable(),
    )
  }
}

impl<R: Read> XmlEventStream for Stream<R> {
  fn next(&mut self) -> Option<XmlReaderResult<XmlEvent>> {
    self.0.next()
  }

  fn peek(&mut self) -> Option<&XmlReaderResult<XmlEvent>> {
    self.0.peek()
  }
}

/// Element Scoped Stream
pub struct ElementScopedStream<'a, S: XmlEventStream + 'a> {
  inner: &'a mut S,
  level: i32,
  elem: XmlElement,
}

impl<'a, S: XmlEventStream + 'a> ElementScopedStream<'a, S> {
  fn new(inner: &mut S) -> Result<ElementScopedStream<S>> {
    let elem = try_consume_event!(inner,
      XmlEvent::StartElement { name, attributes, .. } => XmlElement {
        name: name,
        attributes: XmlAttributeList(attributes),
      });
    Ok(ElementScopedStream {
      inner: inner,
      level: 1,
      elem: elem,
    })
  }

  pub fn elem(&self) -> &XmlElement {
    &self.elem
  }

  pub fn local_name(&self) -> &str {
    self.elem.name.local_name.as_ref()
  }

  pub fn has_next(&mut self) -> Result<bool> {
    match self.peek() {
      Some(&Ok(_)) => Ok(true),
      Some(&Err(ref err)) => Err(err.clone().into()),
      None => Ok(false),
    }
  }

  fn consume_remaining(&mut self) -> Result<()> {
    loop {
      match self.inner.next() {
        None => return Ok(()),
        Some(Ok(XmlEvent::EndElement { .. })) => {
          self.level = self.level - 1;
          if self.level < 1 {
            return Ok(());
          }
        }
        Some(Ok(_)) => {}
        Some(Err(err)) => return Err(err.into()),
      }
    }
  }
}

impl<'a, S: XmlEventStream + 'a> XmlEventStream for ElementScopedStream<'a, S> {
  fn next(&mut self) -> Option<XmlReaderResult<XmlEvent>> {
    if self.level < 1 {
      return None;
    }

    let event = self.inner.next();
    match event {
      Some(Ok(XmlEvent::StartElement { .. })) => {
        self.level = self.level + 1;
      }
      Some(Ok(XmlEvent::EndElement { .. })) => {
        self.level = self.level - 1;
        if self.level < 1 {
          return None;
        }
      }
      _ => {}
    }

    event
  }

  fn peek(&mut self) -> Option<&XmlReaderResult<XmlEvent>> {
    if self.level < 1 {
      return None;
    }

    match self.inner.peek() {
      Some(&Ok(XmlEvent::EndElement { .. })) if self.level == 1 => None,
      Some(result) => Some(result),
      None => None,
    }
  }
}

#[derive(Debug)]
pub struct XmlAttributeList(Vec<Attribute>);
impl XmlAttributeList {
  pub fn find_name<K: AsRef<str>>(&self, name: K) -> Option<&Attribute> {
    let name = name.as_ref();
    self.0.iter().find(|a| a.name.local_name == name)
  }

  pub fn value<K: AsRef<str>>(&self, name: K) -> Option<String> {
    self.find_name(name.as_ref()).map(|a| a.value.clone())
  }

  pub fn value_or<K: AsRef<str>, V: Into<String>>(&self, name: K, default: V) -> String {
    self
      .find_name(name.as_ref())
      .map_or_else(|| default.into(), |a| a.value.clone())
  }
}

#[derive(Debug)]
pub struct XmlElement {
  pub name: Name,
  pub attributes: XmlAttributeList,
}

/// Consume a `StartDocument` event
pub fn start_document<S: XmlEventStream>(stream: &mut S) -> Result<()> {
  Ok(try_consume_event!(stream, XmlEvent::StartDocument { .. }))
}

/// Consume a `EndDocument` event
pub fn end_document<S: XmlEventStream>(stream: &mut S) -> Result<()> {
  Ok(try_consume_event!(stream, XmlEvent::EndDocument))
}

/// Consume a `StartElement` event
pub fn start_element<S: XmlEventStream, N: AsRef<str>>(
  stream: &mut S,
  expected_name: N,
) -> Result<XmlElement> {
  Ok(
    try_consume_event!(stream, XmlEvent::StartElement { name, attributes, .. } => {
    if name.local_name != expected_name.as_ref() {
      return Err(format!("unexpected element: expected '{}', found: '{}'",
        expected_name.as_ref(), name.local_name).into()
      )
    }
    XmlElement {
      name: name,
      attributes: XmlAttributeList(attributes),
    }
  }),
  )
}

/// Consume a `EndElement` event
pub fn end_element<S: XmlEventStream>(stream: &mut S) -> Result<Name> {
  Ok(
    try_consume_event!(stream, XmlEvent::EndElement { name } => name),
  )
}

/// Consume a `Characters` event and parse it
pub fn characters<S: XmlEventStream, E, T: FromStr<Err = E>>(stream: &mut S) -> Result<T>
where
  E: ::std::error::Error + Display,
{
  if let None = stream.peek() {
    return "".parse().map_err(|err| {
      ErrorKind::ParseString("".to_owned(), format!("{}", err)).into()
    });
  }

  let content = try_consume_event!(stream, XmlEvent::Characters(value) => value);
  content.parse().map_err(|err| {
    ErrorKind::ParseString(content, format!("{}", err)).into()
  })
}

/// Consume an element and its children
pub fn skip_element<S: XmlEventStream>(stream: &mut S) -> Result<()> {
  let mut depth = 0;
  loop {
    match stream.next() {
      Some(Ok(XmlEvent::StartElement { .. })) => depth = depth + 1,
      Some(Ok(XmlEvent::EndElement { .. })) => depth = depth - 1,
      Some(Ok(_)) => {}
      Some(Err(err)) => return Err(err.into()),
      None => {
        return Err(
          ErrorKind::UnexpectedEndOfXml("expected end of element".to_string()).into(),
        )
      }
    }

    if depth < 1 {
      break;
    }
  }

  Ok(())
}

pub trait ElementNameSet: ::std::fmt::Debug {
  fn contains_element_name(&self, value: &str) -> bool;
}

impl ElementNameSet for &'static str {
  fn contains_element_name(&self, value: &str) -> bool {
    *self == value
  }
}

impl ElementNameSet for Vec<&'static str> {
  fn contains_element_name(&self, value: &str) -> bool {
    self.contains(&value)
  }
}

/// Consume an element, apply a function to an element scoped stream, return the function result
pub fn element<S: XmlEventStream, N: ElementNameSet, F, T>(
  stream: &mut S,
  expected_name: N,
  mut f: F,
) -> Result<T>
where
  F: FnMut(&mut ElementScopedStream<S>) -> Result<T>,
{
  let mut ss = ElementScopedStream::new(stream)?;
  if !expected_name.contains_element_name(&ss.elem().name.local_name) {
    return Err(
      format!("unexpected element: expected '{:?}', found: '{}'",
      expected_name, ss.elem().name.local_name).into(),
    );
  }
  let result = f(&mut ss)?;
  ss.consume_remaining()?;
  Ok(result)
}

/// Consume all events of a stream by calling a function repeatly
pub fn all<S: XmlEventStream, F, T>(stream: &mut S, mut f: F) -> Result<Vec<T>>
where
  F: FnMut(&mut S) -> Result<T>,
{
  let mut result = vec![];
  loop {
    match stream.peek() {
      Some(&Ok(_)) => {}
      Some(&Err(ref err)) => return Err(err.clone().into()),
      None => break,
    }
    result.push(f(stream)?);
  }
  Ok(result)
}

/// Consume all elements of a stream, fold them to a value
pub fn fold_elements<S: XmlEventStream, State, F>(
  stream: &mut S,
  state: State,
  mut f: F,
) -> Result<State>
where
  F: FnMut(&mut ElementScopedStream<S>, &mut State) -> Result<()>,
{
  let mut state = state;
  {
    let state_ref = &mut state;
    loop {
      let skip = match stream.peek() {
        Some(&Ok(XmlEvent::StartElement { .. })) => false,
        Some(&Err(ref err)) => return Err(err.clone().into()),
        None => break,
        _ => true,
      };

      if skip {
        stream.next();
      } else {
        let mut ss = ElementScopedStream::new(stream)?;
        f(&mut ss, state_ref)?;
        ss.consume_remaining()?;
      }
    }
  }
  Ok(state)
}

#[macro_export]
macro_rules! test_decode {
    (
      $decoder:ident, $xml:expr, $result:expr
    ) => {
      {
        let mut s = $crate::decode::Stream::new(::std::io::Cursor::new($xml));
        let result = <$decoder as $crate::decode::FromXMLStream<_>>::from_xml(&mut s)
          .expect("decode");
        assert_eq!(result, $result);
      }
    };
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::io::Cursor;

  #[test]
  fn test_start_element() {
    let mut s = Stream::new(Cursor::new(r#"<test name="single">888</test>"#));

    start_document(&mut s).expect("start document");
    let elem = start_element(&mut s, "test").expect("expected an element");
    let name = elem.attributes.value("name").expect("name attr");
    assert_eq!(name, "single");

    let chars: String = characters(&mut s).expect("characters");
    assert_eq!(chars, "888");
  }

  #[test]
  fn test_element_scope() {
    let mut s = Stream::new(Cursor::new(r#"<test>content</test>"#));

    start_document(&mut s).expect("start document");
    let (name, content): (String, String) = element(&mut s, "test", |s| {
      Ok((s.elem().name.local_name.clone(), characters(s)?))
    }).expect("element");
    assert_eq!(name, "test");
    assert_eq!(content, "content");
  }

  #[test]
  fn test_fold_elements() {
    let s = &mut Stream::new(Cursor::new(r#"<test>1</test><test>2</test><test>3</test>"#));
    start_document(s).expect("start document");
    let mut contents = String::new();
    fold_elements(s, &mut contents, |ss, contents| {
      let content: String = characters(ss)?;
      contents.push_str(&content);
      Ok(())
    }).expect("fold_elements");
    assert_eq!(contents, "123");
  }

  #[test]
  fn test_element_list() {
    let mut s = Stream::new(Cursor::new(
      r#"      <Order>
        <LatestShipDate>2017-01-07T10:00:00Z</LatestShipDate>
        <OrderType>StandardOrder</OrderType>
        <PurchaseDate>2017-01-06T05:05:04Z</PurchaseDate>
        <AmazonOrderId>102-6272421-6433852</AmazonOrderId>
        <LastUpdateDate>2017-01-06T05:05:05Z</LastUpdateDate>
        <ShipServiceLevel>SecondDay</ShipServiceLevel>
        <NumberOfItemsShipped>0</NumberOfItemsShipped>
        <OrderStatus>Pending</OrderStatus>
        <SalesChannel>Amazon.com</SalesChannel>
        <IsBusinessOrder>false</IsBusinessOrder>
        <NumberOfItemsUnshipped>1</NumberOfItemsUnshipped>
        <IsPremiumOrder>false</IsPremiumOrder>
        <EarliestShipDate>2017-01-07T10:00:00Z</EarliestShipDate>
        <MarketplaceId>ATVPDKIKX0DER</MarketplaceId>
        <FulfillmentChannel>AFN</FulfillmentChannel>
        <PaymentMethod>Other</PaymentMethod>
        <IsPrime>false</IsPrime>
        <ShipmentServiceLevelCategory>SecondDay</ShipmentServiceLevelCategory>
        <SellerOrderId>102-6272421-6433852</SellerOrderId>
      </Order>"#,
    ));

    #[derive(Default, Debug, PartialEq)]
    struct Order {
      amazon_order_id: String,
      order_type: String,
    }

    start_document(&mut s).expect("start document");
    let order = element(&mut s, "Order", |ss| {
      let order = fold_elements(ss, Order::default(), |ss, state| {
        match ss.elem().name.local_name.as_ref() {
          "AmazonOrderId" => state.amazon_order_id = characters(ss)?,
          "OrderType" => state.order_type = characters(ss)?,
          _ => {}
        }
        Ok(())
      })?;
      Ok(order)
    }).expect("order");

    assert_eq!(order, Order {
        amazon_order_id: "102-6272421-6433852".to_string(),
        order_type: "StandardOrder".to_string(),
      });
  }

  #[test]
  fn test_empty_element() {
    let mut s = Stream::new(Cursor::new(r#"<Empty></Empty>"#));

    start_document(&mut s).expect("start document");
    element(&mut s, "Empty", |s| {
      assert_eq!(s.local_name(), "Empty");
      let v: String = characters(s)?;
      assert_eq!(v, "");
      Ok(())
    }).expect("element");
  }

}
