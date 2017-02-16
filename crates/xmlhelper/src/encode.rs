use xml::writer::{XmlEvent, EventWriter, Result};
use std::io::Write;

pub trait XmlEventWriter {
  fn write(&mut self, event: XmlEvent) -> Result<()>;
}

impl<W: Write> XmlEventWriter for EventWriter<W> {
  fn write(&mut self, event: XmlEvent) -> Result<()> {
    self.write(event)
  }
}

pub trait XmlWrite<W: XmlEventWriter> {
  fn write_xml(&self, w: &mut W) -> Result<()>;
}

#[macro_export]
macro_rules! write_xml {
  ($w:expr, ) => (Ok(()));

  ($w:expr, [ $b:block ] $($rest:tt)*) => (
    $b.and_then(|_| write_xml!($w, $($rest)*))
  );

  ($w:expr, $e:tt) => (
    $w.write($e)
  );

  ($w:expr, 
    $tag_name:ident[$($attr_name:ident=$attr_value:expr),*][
      $($inner:tt)*
    ] $($rest:tt)*
  ) => ({
    let event = ::xml::writer::XmlEvent::start_element(stringify!($tag_name));
    $(
      let event = event.attr(stringify!($attr_name), $attr_value);
    )*
    let event: ::xml::writer::XmlEvent = event.into();
    $w.write(event)
      .and_then(|_| {
        write_xml!($w, $($inner)*)
      })
      .and_then(|_| {
        $w.write(::xml::writer::XmlEvent::EndElement { name: None })
      })
      .and_then(|_| {
        write_xml!($w, $($rest)*)
      })
  });
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_write_xml() {
    use xml::EmitterConfig;
    let mut writer = EventWriter::new_with_config(vec![], EmitterConfig::new().perform_indent(true));
    let context = vec!["context-value-0", "context-value-1"];

    {
      let w = &mut writer;
      write_xml!(w,
        Enevolop[Static="static-value", FromContext=context[0] ][
          Header[FirstAttr="1", SecondAttr="2" ][]
          Body[][
            "Body Content"
          ]
          [{
            write_xml!(w,
              ContextItems[][
                [{
                  for (i, v) in context.iter().enumerate() {
                    let idx = i.to_string();
                    write_xml!(w, 
                      Item[Index=(&idx)] [
                        (*v)
                      ]
                    ).unwrap();
                  }
                  Ok(())
                }]
              ]
            )
          }]
          Footer[][
            (context[1])
          ]
        ]
      ).unwrap();
    }
    let xml = String::from_utf8(writer.into_inner()).unwrap();
    assert_eq!(xml, r#"<?xml version="1.0" encoding="utf-8"?>
<Enevolop Static="static-value" FromContext="context-value-0">
  <Header FirstAttr="1" SecondAttr="2" />
  <Body>Body Content</Body>
  <ContextItems>
    <Item Index="0">context-value-0</Item>
    <Item Index="1">context-value-1</Item>
  </ContextItems>
  <Footer>context-value-1</Footer>
</Enevolop>"#);
  }
}