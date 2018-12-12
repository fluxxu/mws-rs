use chrono::{DateTime, Utc};
use feeds::Envelope;
use feeds::Message;
use types::ToIso8601;
use xmlhelper::encode;

#[derive(Debug, Serialize)]
pub struct OrderFulfillmentMessage {
  pub message_id: String,
  pub amazon_order_id: String,
  pub fulfillment_date: DateTime<Utc>,
  pub carrier_name: String,
  pub shipping_method: String,
  pub shipper_tracking_number: String,
  pub items: Vec<OrderFulfillmentItem>,
}

#[derive(Debug, Serialize)]
pub struct OrderFulfillmentItem {
  pub amazon_order_item_code: String,
  pub quantity: i32,
}

impl Message for OrderFulfillmentMessage {
  fn get_message_type() -> &'static str {
    "OrderFulfillment"
  }
}

impl<W: encode::XmlEventWriter> encode::XmlWrite<W> for Envelope<OrderFulfillmentMessage> {
  fn write_xml(&self, w: &mut W) -> encode::Result<()> {
    self.write_envelope_xml(w, |w: &mut W| {
      for message in self.messages.iter() {
        let fulfillment_date = message.data.fulfillment_date.to_iso8601();
        write_xml!(w,
          Message[][
            MessageID[][
              (&message.data.message_id)
            ]
            OperationType[][
              "Update"
            ]
            OrderFulfillment[][
              AmazonOrderID[][
                (&message.data.amazon_order_id)
              ]
              FulfillmentDate[][
                (&fulfillment_date)
              ]
              FulfillmentData[][
                CarrierName[][
                  (&message.data.carrier_name)
                ]
                ShippingMethod[][
                  (&message.data.shipping_method)
                ]
                ShipperTrackingNumber[][
                  (&message.data.shipper_tracking_number)
                ]
              ]
              [{
                for item in &message.data.items {
                  let quantity = item.quantity.to_string();
                  write_xml!(w,
                    Item[][
                      AmazonOrderItemCode[][
                        (&item.amazon_order_item_code)
                      ]
                      Quantity[][
                        (&quantity)
                      ]
                    ]
                  )?;
                }
                Ok(())
              }]
            ]
          ]
        )?;
      }
      Ok(())
    })
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use chrono::FixedOffset;
  use feeds::OperationType;
  use xmlhelper::encode::XmlWrite;
  use xmlhelper::encode::{EmitterConfig, EventWriter};

  #[test]
  fn test_order_fulfillment_feed() {
    let mut writer =
      EventWriter::new_with_config(vec![], EmitterConfig::new().perform_indent(true));

    {
      let w = &mut writer;
      let mut e = Envelope::<OrderFulfillmentMessage>::new("1234567890".to_owned());
      e.add_message(
        OrderFulfillmentMessage {
          message_id: "1".to_string(),
          amazon_order_id: "112-3739032-8075461".to_string(),
          fulfillment_date: DateTime::<FixedOffset>::parse_from_rfc3339(
            "2018-12-06T17:08:31.760132Z",
          )
          .unwrap()
          .with_timezone(&Utc),
          carrier_name: "UPS".to_string(),
          shipping_method: "Standard".to_string(),
          shipper_tracking_number: "1Z71178X0261236762".to_string(),
          items: vec![OrderFulfillmentItem {
            amazon_order_item_code: "56323517235162".to_string(),
            quantity: 1,
          }],
        },
        Some(OperationType::Update),
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
  <MessageType>OrderFulfillment</MessageType>
  <Message>
    <MessageID>1</MessageID>
    <OperationType>Update</OperationType>
    <OrderFulfillment>
      <AmazonOrderID>112-3739032-8075461</AmazonOrderID>
      <FulfillmentDate>2018-12-06T17:08:31Z</FulfillmentDate>
      <FulfillmentData>
        <CarrierName>UPS</CarrierName>
        <ShippingMethod>Standard</ShippingMethod>
        <ShipperTrackingNumber>1Z71178X0261236762</ShipperTrackingNumber>
      </FulfillmentData>
      <Item>
        <AmazonOrderItemCode>56323517235162</AmazonOrderItemCode>
        <Quantity>1</Quantity>
      </Item>
    </OrderFulfillment>
  </Message>
</AmazonEnvelope>"#
    );
  }
}
