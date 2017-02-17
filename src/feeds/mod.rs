//! Amazon MWS Feeds API - Version 2010-10-01
//!
//! [Documentation](http://docs.developer.amazonservices.com/en_CA/feeds/Feeds_Overview.html)

use xmlhelper::encode;

mod inventory;

/// Enumerates all the feed types that are available through the Feeds API section.
string_map_enum! {
  pub enum FeedType {
    Product = "_POST_PRODUCT_DATA_",
    Inventory = "_POST_INVENTORY_AVAILABILITY_DATA_",
    Overrides = "_POST_PRODUCT_OVERRIDES_DATA_",
    Pricing = "_POST_PRODUCT_PRICING_DATA_",
    ProductImages = "_POST_PRODUCT_IMAGE_DATA_",
    Relationships = "_POST_PRODUCT_RELATIONSHIP_DATA_",
    FlatFileInventoryLoader = "_POST_FLAT_FILE_INVLOADER_DATA_",
    FlatFileListings = "_POST_FLAT_FILE_LISTINGS_DATA_",
    FlatFileBookLoader = "_POST_FLAT_FILE_BOOKLOADER_DATA_",
    FlatFileMusicLoader = "_POST_FLAT_FILE_CONVERGENCE_LISTINGS_DATA_",
    // FlatFileVideoLoader = "_POST_FLAT_FILE_LISTINGS_DATA_",
    FlatFilePriceAndQuantityUpdate = "_POST_FLAT_FILE_PRICEANDQUANTITYONLY_UPDATE_DATA_",
    UIEEInventory = "_POST_UIEE_BOOKLOADER_DATA_",
    AutomotivePartFinder = "_POST_STD_ACES_DATA_",
  }
}

/// The optional OperationType element can be used to specify the type of operation (Update, Delete
/// or PartialUpdate) to be performed on the data. The OperationType is only applicable to productrelated
/// feeds (Product, Inventory, Price, etc) and will be ignored for non-applicable feeds.
#[derive(Debug, PartialEq)] 
pub enum OperationType {
  /// All specified information overwrites any existing information. Any
  /// unspecified information is erased.
  Update,

  /// All information is removed
  Delete,

  /// For Product feeds only: If you use PartialUpdate for a Product feed, all specified
  /// information overwrites any existing information, but unspecified information is
  /// unaffected. Caution: This operation type is only valid for Product feeds. If this operation
  /// type is used for any other feed type, such as Inventory and Price feeds, unpredictable
  /// data loss can occur.
  PartialUpdate,
}

pub trait Message {
  fn get_message_type() -> &'static str;
}

#[allow(non_snake_case)]
#[derive(Debug, PartialEq)]
pub struct EnvelopeMessage<M: Message> {
  pub data: M,
  pub operation_type: Option<OperationType>,
}

#[allow(non_snake_case)]
#[derive(Debug, PartialEq)]
pub struct Envelope<M: Message> {
  pub MerchantIdentifier: String,
  messages: Vec<EnvelopeMessage<M>>,
}


impl<M: Message> Envelope<M> {
  pub fn new(merchant_identifier: String) -> Self {
    Envelope::<M> {
      MerchantIdentifier: merchant_identifier,
      messages: vec![],
    }
  }

  pub fn add_message(&mut self, m: M, operation_type: Option<OperationType>) -> &mut Self {
    self.messages.push(EnvelopeMessage::<M> {
      data: m,
      operation_type: operation_type,
    });
    self
  }

  pub fn write_envelope_xml<W: encode::XmlEventWriter, F: FnMut(&mut W) -> encode::Result<()>>(&self, w: &mut W, mut f: F) -> encode::Result<()> {
    w.write(
      encode::XmlEvent::start_element("AmazonEnvelope")
        .ns("xsi", "http://www.w3.org/2001/XMLSchema-instance")
        .attr("xsi:noNamespaceSchemaLocation", "amznenvelope.xsd")
        .into()
    )?;

    let mid: &str = self.MerchantIdentifier.as_ref();
    let message_type: &str = M::get_message_type();

    write_xml!(w,
      Header[][
        DocumentVersion[]["1.01"]
        MerchantIdentifier[][mid]
      ]
      MessageType[][message_type]
      Messages[][
        [{ f(w) }]
      ]
    )?;

    w.write(
      encode::XmlEvent::end_element().into()
    )
  }
}

impl<W: encode::XmlEventWriter> encode::XmlWrite<W> for Envelope<inventory::InventoryMessage> {
  fn write_xml(&self, w: &mut W) -> encode::Result<()> {
    self.write_envelope_xml(w, |w: &mut W| {
      for message in self.messages.iter() {
        let sku: &str = message.data.SKU.as_ref();
        let quantity = message.data.Quantity.to_string();
        let fulfillment_latency = message.data.FulfillmentLatency.to_string();
        write_xml!(w,
          Message[][
            SKU[][sku]
            Quantity[][(&quantity)]
            FulfillmentLatency[][(&fulfillment_latency)]
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
  use xmlhelper::encode::{EventWriter, EmitterConfig};
  use xmlhelper::encode::XmlWrite;

  #[test]
  fn test_envelope_write_xml() {
    let mut writer = EventWriter::new_with_config(vec![], EmitterConfig::new().perform_indent(true));

    {
      let w = &mut writer;
      let mut e = Envelope::<inventory::InventoryMessage>::new("1234567890".to_owned());
      e.add_message(inventory::InventoryMessage {
        SKU: "p1".to_owned(),
        Quantity: 100,
        FulfillmentLatency: 0,
      }, Some(OperationType::PartialUpdate))
      .add_message(inventory::InventoryMessage {
        SKU: "p2".to_owned(),
        Quantity: 200,
        FulfillmentLatency: 0,
      }, Some(OperationType::PartialUpdate));
      e.write_xml(w).unwrap();
    }

    let xml = String::from_utf8(writer.into_inner()).unwrap();
    assert_eq!(xml, r#"<?xml version="1.0" encoding="utf-8"?>
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
</AmazonEnvelope>"#);
  }
}