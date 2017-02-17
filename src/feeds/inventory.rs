use super::{Message};

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