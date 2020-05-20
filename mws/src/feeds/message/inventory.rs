use feeds::Envelope;
use feeds::Message;
use xmlhelper::encode;

#[allow(non_snake_case)]
#[derive(Debug, PartialEq, Serialize)]
pub struct InventoryMessage {
  pub message_id: String,
  /// Used to identify an individual product. Each product must have a SKU, and each SKU must
  /// be unique.
  pub sku: String,

  /// Indicates whether or not an item is available (any positive number = available; 0 = not
  /// available). Every time a quantity is sent for an item, the existing quantity is replaced by the
  /// new quantity in the feed.
  pub quantity: i32,

  /// The number of days between the order date and the ship date (a whole number between 1
  /// and 30)
  pub fulfillment_latency: i32,
}

impl Message for InventoryMessage {
  fn get_message_type() -> &'static str {
    "Inventory"
  }
}

impl<W: encode::XmlEventWriter> encode::XmlWrite<W> for Envelope<InventoryMessage> {
  fn write_xml(&self, w: &mut W) -> encode::Result<()> {
    self.write_envelope_xml(w, |w: &mut W| {
      write_xml!(w,
        Messages[][
          [{
            for message in self.messages.iter() {
              let sku: &str = message.data.sku.as_ref();
              let quantity = message.data.quantity.to_string();
              let fulfillment_latency = message.data.fulfillment_latency.to_string();
              write_xml!(w,
                Message[][
                  MessageID[][
                    (&message.data.message_id)
                  ]
                  SKU[][sku]
                  Quantity[][(&quantity)]
                  FulfillmentLatency[][(&fulfillment_latency)]
                ]
              )?;
            }
            Ok(())
          }]
        ]
      )?;
      Ok(())
    })
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use feeds::OperationType;
  use xmlhelper::encode::XmlWrite;
  use xmlhelper::encode::{EmitterConfig, EventWriter};

  #[test]
  fn test_inventory_feed() {
    let mut writer =
      EventWriter::new_with_config(vec![], EmitterConfig::new().perform_indent(true));

    {
      let w = &mut writer;
      let mut e = Envelope::<InventoryMessage>::new("1234567890".to_owned());
      e.add_message(
        InventoryMessage {
          SKU: "p1".to_owned(),
          Quantity: 100,
          FulfillmentLatency: 0,
        },
        Some(OperationType::PartialUpdate),
      )
      .add_message(
        InventoryMessage {
          SKU: "p2".to_owned(),
          Quantity: 200,
          FulfillmentLatency: 0,
        },
        Some(OperationType::PartialUpdate),
      );
      e.write_xml(w).unwrap();
    }

    let xml = String::from_utf8(writer.into_inner()).unwrap();
    assert_eq!(
      xml,
      r#"<?xml version="1.0" encoding="utf-8"?>
<AmazonEnvelope xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:noNamespaceSchemaLocation="amznenvelope.xsd">
  <Header>
    <DocumentVersion>1.01</DocumentVersion>
    <MerchantIdentifier>1234567890</MerchantIdentifier>
  </Header>
  <MessageType>Inventory</MessageType>
  <Messages>
    <Message>
      <SKU>p1</SKU>
      <Quantity>100</Quantity>
      <FulfillmentLatency>0</FulfillmentLatency>
    </Message>
    <Message>
      <SKU>p2</SKU>
      <Quantity>200</Quantity>
      <FulfillmentLatency>0</FulfillmentLatency>
    </Message>
  </Messages>
</AmazonEnvelope>"#
    );
  }
}
