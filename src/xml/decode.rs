//! XML Decode

use std::default::Default;
use std::io::Read;
use std::rc::Rc;
use std::cell::RefCell;
use xml_rs::reader::{EventReader, XmlEvent, Events, Result as ReaderResult};
use xml_rs::attribute::OwnedAttribute;

pub use xml_rs::name::OwnedName as Name;

error_chain! {
  foreign_links {
    XML(::xml_rs::reader::Error);
  }

  errors {
    UnexpectedEndOfXML
  }
}

#[derive(Debug)]
pub struct XMLAttributeList (Vec<OwnedAttribute>);
impl XMLAttributeList {
  pub fn find_name<K: AsRef<str>>(&self, name: K) -> Option<&OwnedAttribute> {
    let name = name.as_ref();
    self.0.iter()
      .find(|a| a.name.local_name == name)
  }

  pub fn value<K: AsRef<str>>(&self, name: K) -> Option<String> {
    self.find_name(name.as_ref()).map(|a| a.value.clone())
  }

  pub fn value_or<K: AsRef<str>, V: Into<String>>(&self, name: K, default: V) -> String {
    self.find_name(name.as_ref())
      .map_or_else(|| default.into(), |a| a.value.clone())
  }
}

#[derive(Debug)]
pub enum XMLDecodeEvent {
  StartElement(Name, XMLAttributeList),
  Characters(String),
  EndElement(Name),
}

pub trait XMLDecode: Default + Sized {
  type R: Read;
  type State: Default;

  fn update(&mut self, state: &mut Self::State, event: XMLDecodeEvent, context: Option<XMLDecoderContext<Self::R>>) -> Result<()>;
  fn finalize(&mut self, state: &Self::State) -> Result<()> {
    Ok(())
  }
}

pub struct XMLDecoderContext<R: Read> {
  level: i32,
  end_level: i32,
  events: Rc<RefCell<Events<R>>>,
  queued_event: Option<XmlEvent>,
}

impl<R: Read> XMLDecoderContext<R> {
  fn new(events: Events<R>) -> XMLDecoderContext<R> {
    XMLDecoderContext {
      level: 0,
      end_level: 1,
      events: Rc::new(RefCell::new(events)),
      queued_event: None,
    }
  }

  fn fork(&mut self, current_event: XmlEvent) -> XMLDecoderContext<R> {
    XMLDecoderContext {
      level: self.level,
      end_level: self.level,
      events: self.events.clone(),
      queued_event: Some(current_event),
    }
  }

  fn dispatch<D: XMLDecode<R = R>>(&mut self, event: XmlEvent, target: &mut D, state: &mut D::State) -> Result<bool> {
    match event {
      e @ XmlEvent::StartElement { .. } => {
        self.level = self.level + 1;
        let ctx = self.fork(e.clone()); // unnecessary copy?
        if let XmlEvent::StartElement { name, attributes, .. } = e {
          target.update(state, XMLDecodeEvent::StartElement(name, XMLAttributeList(attributes)), Some(ctx))?;
        }
      },
      XmlEvent::Characters(value) => {
        target.update(state, XMLDecodeEvent::Characters(value), None)?;
      },
      XmlEvent::EndElement { name, .. } => {
        target.update(state, XMLDecodeEvent::EndElement(name), None)?;
        self.level = self.level - 1;
        if self.level == self.end_level {
          target.finalize(state)?;
          return Ok(true)
        }
      },
      XmlEvent::EndDocument => {
        return Err(ErrorKind::UnexpectedEndOfXML.into())
      },
      _ => {}
    }
    Ok(false)
  }

  fn decode<D: XMLDecode<R = R>>(&mut self) -> Result<D> {
    let mut target = D::default();
    let mut state = D::State::default();
    loop {
      let evt = if let Some(e) = self.queued_event.take() {
        e
      } else {
        (*self.events.as_ref().borrow_mut())
          .next()
          .ok_or(ErrorKind::UnexpectedEndOfXML)??
      };

      if self.dispatch(evt, &mut target, &mut state)? {
        return Ok(target)
      }
    }
  }
}

pub fn decode<R: Read, D: XMLDecode<R = R>>(source: R) -> Result<D> {
  let reader = EventReader::new(source);
  XMLDecoderContext::<R>::new(reader.into_iter()).decode::<D>()
}

pub trait XMLSource {
  type R: Read;

  fn xml_decode<D: XMLDecode<R = Self::R>>(self) -> Result<D>;
}

impl<T: Read> XMLSource for T {
  type R = T;

  fn xml_decode<D: XMLDecode<R = Self::R>>(self) -> Result<D> {
    decode::<Self::R, D>(self)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::io::Cursor;
  
  type Reader = Cursor<&'static str>;

  #[test]
  fn single_element() {
    #[derive(Debug, Default, PartialEq)]
    struct Test {
      pub name: String,
      pub value: i32,
    }

    impl XMLDecode for Test {
      type R = Reader;
      type State = ();

      fn update(&mut self, state: &mut Self::State, event: XMLDecodeEvent, context: Option<XMLDecoderContext<Self::R>>) -> Result<()> {
        match event {
          XMLDecodeEvent::StartElement(ref name, ref attrs) => {
            match name.local_name.as_ref() {
              "test" => self.name = attrs.value("name").expect("name attr"),
              _ => {}
            }
          },
          XMLDecodeEvent::Characters(ref value) => {
            self.value = value.parse().unwrap();
          },
          _ => {}
        }

        Ok(())
      }
    }

    let decoded = Cursor::new(r#"<test name="single">888</test>"#).xml_decode::<Test>().unwrap();
    assert_eq!(decoded, Test {
      name: "single".to_string(),
      value: 888,
    });
  }

  #[test]
  fn nested_element () {
    #[derive(Debug, Default, PartialEq)]
    struct Test {
      pub name: String,
      pub value: i32,
      pub inner: TestInner,
    }

    impl XMLDecode for Test {
      type R = Reader;
      type State = ();

      fn update(&mut self, state: &mut Self::State, event: XMLDecodeEvent, context: Option<XMLDecoderContext<Self::R>>) -> Result<()> {
        match event {
          XMLDecodeEvent::StartElement(ref name, ref attrs) => {
            match name.local_name.as_ref() {
              "test" => self.name = attrs.value("name").expect("name attr"),
              "test-inner" => {
                if let Some(mut ctx) = context {
                  self.inner = ctx.decode()?;
                }
              }
              _ => {}
            }
          },
          XMLDecodeEvent::Characters(ref value) => {
            self.value = value.parse().unwrap();
          },
          _ => {}
        }

        Ok(())
      }
    }

    #[derive(Debug, Default, PartialEq)]
    struct TestInner {
      pub value: i32
    }

    impl XMLDecode for TestInner {
      type R = Reader;
      type State = ();

      fn update(&mut self, state: &mut Self::State, event: XMLDecodeEvent, context: Option<XMLDecoderContext<Self::R>>) -> Result<()> {
        match event {
          XMLDecodeEvent::Characters(ref value) => {
            self.value = value.parse().unwrap();
          },
          _ => {}
        }

        Ok(())
      }
    }

    let decoded = Cursor::new(r#"<test name="single">888<test-inner>999</test-inner></test>"#).xml_decode::<Test>().unwrap();
    assert_eq!(decoded, Test {
      name: "single".to_string(),
      value: 888,
      inner: TestInner {
        value: 999,
      }
    });
  }
}