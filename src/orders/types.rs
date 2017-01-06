use chrono::{DateTime, UTC};
use xmlhelper::decode;

#[allow(non_snake_case)]
#[derive(Debug, Default, PartialEq)]
pub struct Order {
  pub LatestShipDate: Option<DateTime<UTC>>,
  pub OrderType: String,
  pub PurchaseDate: Option<DateTime<UTC>>,
  pub AmazonOrderId: String,
  pub LastUpdateDate: Option<DateTime<UTC>>,
  pub ShipServiceLevel: String,
  pub NumberOfItemsShipped: i32,
  pub OrderStatus: String,
  pub SalesChannel: String,
  pub IsBusinessOrder: bool,
  pub NumberOfItemsUnshipped: i32,
  pub IsPremiumOrder: bool,
  pub EarliestShipDate: Option<DateTime<UTC>>,
  pub MarketplaceId: String,
  pub FulfillmentChannel: String,
  pub PaymentMethod: String,
  pub IsPrime: bool,
  pub ShipmentServiceLevelCategory: String,
  pub SellerOrderId: String,
}

impl<S: decode::XmlEventStream> decode::FromXMLStream<S> for Order {
  fn from_xml(s: &mut S) -> decode::Result<Order> {
    use xmlhelper::decode::{element, fold_elements, characters};
    element(s, "Order", |s| {
      fold_elements(s, Order::default(), |s, order| {
        match s.elem().name.local_name.as_ref() {
          "LatestShipDate" => order.LatestShipDate = characters(s).map(Some)?,
          "OrderType" => order.OrderType = characters(s)?,
          "PurchaseDate" => order.PurchaseDate = characters(s).map(Some)?,
          "AmazonOrderId" => order.AmazonOrderId = characters(s)?,
          "LastUpdateDate" => order.LastUpdateDate = characters(s).map(Some)?,
          "ShipServiceLevel" => order.ShipServiceLevel = characters(s)?,
          "NumberOfItemsShipped" => order.NumberOfItemsShipped = characters(s)?,
          "OrderStatus" => order.OrderStatus = characters(s)?,
          "SalesChannel" => order.SalesChannel = characters(s)?,
          "IsBusinessOrder" => order.IsBusinessOrder = characters(s)?,
          "NumberOfItemsUnshipped" => order.NumberOfItemsUnshipped = characters(s)?,
          "IsPremiumOrder" => order.IsPremiumOrder = characters(s)?,
          "EarliestShipDate" => order.EarliestShipDate = characters(s).map(Some)?,
          "MarketplaceId" => order.MarketplaceId = characters(s)?,
          "FulfillmentChannel" => order.FulfillmentChannel = characters(s)?,
          "PaymentMethod" => order.PaymentMethod = characters(s)?,
          "IsPrime" => order.IsPrime = characters(s)?,
          "ShipmentServiceLevelCategory" => order.ShipmentServiceLevelCategory = characters(s)?,
          "SellerOrderId" => order.SellerOrderId = characters(s)?,
          _ => {},
        }
        Ok(())
      })
    })
  }
}