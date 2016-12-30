//! XML Decode

use std::default::Default;
use std::io::Read;
use std::rc::Rc;
use xml::reader::{EventReader, XmlEvent, Events};
use xml::attribute::OwnedAttribute;

pub use xml::name::OwnedName as Name;

error_chain! {
  foreign_links {
    XML(::xml::reader::Error);
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

  fn update(&mut self, event: XMLDecodeEvent, context: Option<XMLDecoderContext<Self::R>>) -> Result<()>;
  fn finalize(self) -> Result<Self> {
    Ok(self)
  }
}

pub struct XMLDecoderContext<R: Read> {
  level: i32,
  end_level: i32,
  events: Rc<Events<R>>,
}

impl<R: Read> XMLDecoderContext<R> {
  fn new(events: Rc<Events<R>>) -> XMLDecoderContext<R> {
    XMLDecoderContext {
      level: 0,
      end_level: 0,
      events: events,
    }
  }

  fn fork(&mut self) -> XMLDecoderContext<R> {
    XMLDecoderContext {
      level: self.level,
      end_level: self.level,
      events: self.events.clone(),
    }
  }

  fn decode<D: XMLDecode<R = R>>(&mut self) -> Result<D> {
    let mut r = D::default();
    loop {
      let evt = Rc::get_mut(&mut self.events).unwrap().next()
          .ok_or(ErrorKind::UnexpectedEndOfXML)??;
      match evt {
        XmlEvent::StartElement { name, attributes, .. } => {
          self.level = self.level + 1;
          let ctx = self.fork();
          r.update(XMLDecodeEvent::StartElement(name, XMLAttributeList(attributes)), Some(ctx))?
        },
        XmlEvent::Characters(value) => {
          r.update(XMLDecodeEvent::Characters(value), None)?
        },
        XmlEvent::EndElement { name, .. } => {
          r.update(XMLDecodeEvent::EndElement(name), None)?;
          self.level = self.level - 1;
          if self.level == self.end_level {
            return r.finalize()
          }
        },
        XmlEvent::EndDocument => {
          return Err(ErrorKind::UnexpectedEndOfXML.into())
        },
        _ => {}
      }
    }
  }
}

pub fn decode<R: Read, D: XMLDecode<R = R>>(source: R) -> Result<D> {
  let reader = EventReader::new(source);
  XMLDecoderContext::<R>::new(Rc::new(reader.into_iter())).decode::<D>()
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

      fn update(&mut self, event: XMLDecodeEvent, context: Option<XMLDecoderContext<Self::R>>) -> Result<()> {
        match event {
          XMLDecodeEvent::StartElement(_, attrs) => {
            self.name = attrs.value("name").expect("name attr");
          },
          XMLDecodeEvent::Characters(value) => {
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
}