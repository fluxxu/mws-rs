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
  pub OrderStatus: OrderStatus,
  pub SalesChannel: String,
  pub IsBusinessOrder: bool,
  pub NumberOfItemsUnshipped: i32,
  pub IsPremiumOrder: bool,
  pub EarliestShipDate: Option<DateTime<UTC>>,
  pub MarketplaceId: String,
  pub FulfillmentChannel: FulfillmentChannel,
  pub PaymentMethod: PaymentMethod,
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

/// A list of OrderStatus values. Used to select orders with a current status that matches 
/// one of the status values that you specify.
///
/// Unshipped and PartiallyShipped must be used together in this version of the Orders API 
/// section. Using one and not the other returns an error.
///
/// [Reference](http://docs.developer.amazonservices.com/en_CA/orders-2013-09-01/Orders_ListOrders.html)
str_enum! {
  pub enum OrderStatus {
    PendingAvailability,
    Pending,
    Unshipped,
    PartiallyShipped,
    Shipped,
    InvoiceUnconfirmed,
    Canceled,
    Unfulfillable,
  }
}

///	A list that indicates how an order was fulfilled.
/// 
/// [Reference](http://docs.developer.amazonservices.com/en_CA/orders-2013-09-01/Orders_ListOrders.html)
str_enum! {
  pub enum FulfillmentChannel {
    AFN,
    MFN,
  }
}

/// A list of PaymentMethod values. Used to select orders paid for with the payment methods 
/// that you specify.
/// 
/// Note: COD and CVS values are valid only in Japan (JP).
str_enum! {
  pub enum PaymentMethod {
    COD,
    CVS,
    Other,
  }
}

/// A list of TFMShipmentStatus values. Used to select Amazon Transportation for Merchants (TFM) 
/// orders with a current status that matches one of the status values that you specify. 
/// If TFMShipmentStatus is specified, only TFM orders are returned.
/// 
/// Note: The TFMShipmentStatus request parameter is available only in China (CN).
str_enum! {
  pub enum TFMShipmentStatus {
    PendingPickUp,
    LabelCanceled,
    PickedUp,
    AtDestinationFC,
    Delivered,
    RejectedByBuyer,
    Undeliverable,
    ReturnedToSeller,
    Lost,
  }
}