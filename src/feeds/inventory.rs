use super::{Message};

/*
<?xml version="1.0" encoding="utf-8" ?>
<AmazonEnvelope xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:noNamespaceSchemaLocation="amznenvelope.xsd">
<Header>
<DocumentVersion>1.01</DocumentVersion>
<MerchantIdentifier>M_SELLER_354577</MerchantIdentifier>
</Header>
<MessageType>Inventory</MessageType>
<Message>
<MessageID>1</MessageID>
<OperationType>Update</OperationType>
<Inventory>
<SKU>ASUSVNA1</SKU>
<Quantity>8</Quantity>
<FulfillmentLatency>1</FulfillmentLatency>
</Inventory>
</Message>
<Message>
<MessageID>2</MessageID>
<OperationType>Update</OperationType>
<Inventory>
<SKU>ASUS8VM</SKU>
<Quantity>6</Quantity>
<FulfillmentLatency>1</FulfillmentLatency>
</Inventory>
</Message>
</AmazonEnvelope>
*/


#[allow(non_snake_case)]
#[derive(Debug, PartialEq)]
pub struct InventoryMessage {
  /// Used to identify an individual product. Each product must have a SKU, and each SKU must
  /// be unique.
  pub SKU: String,

  /// Indicates whether or not an item is available (any positive number = available; 0 = not
  /// available). Every time a quantity is sent for an item, the existing quantity is replaced by the
  /// new quantity in the feed.
  pub Quantity: i32,

  /// The number of days between the order date and the ship date (a whole number between 1
  /// and 30)
  pub FulfillmentLatency: i32,
}

impl Message for InventoryMessage {
  fn get_message_type() -> &'static str {
    "Inventory"
  }
}