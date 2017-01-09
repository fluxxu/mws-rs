use chrono::{DateTime, UTC};
use xmlhelper::decode;

#[allow(non_snake_case)]
#[derive(Debug, Default, PartialEq)]
pub struct ShippingAddress {
  pub StateOrRegion: String,
  pub City: String,
  pub CountryCode: String,
  pub PostalCode: String,
  pub Name: String,
  pub AddressLine1: String,
  pub AddressLine2: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Default, PartialEq)]
pub struct OrderTotal {
  pub CurrencyCode: String,
  pub Amount: f64,
}

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
  pub BuyerEmail: String,
  pub BuyerName: String,
  pub OrderTotal: Option<OrderTotal>,
  pub ShippingAddress: Option<ShippingAddress>,
}

impl<S: decode::XmlEventStream> decode::FromXMLStream<S> for Order {
  fn from_xml(s: &mut S) -> decode::Result<Order> {
    use xmlhelper::decode::{element, fold_elements, characters};
    element(s, "Order", |s| {
      fold_elements(s, Order::default(), |s, order| {
        match s.local_name() {
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
          "BuyerEmail" => order.BuyerEmail = characters(s)?,
          "BuyerName" => order.BuyerName = characters(s)?,
          "OrderTotal" => order.OrderTotal = fold_elements(s, OrderTotal::default(), |s, total| {
            match s.local_name() {
              "CurrencyCode" => total.CurrencyCode = characters(s)?,
              "Amount" => total.Amount = characters(s)?,
              _ => {},
            }
            Ok(())
          }).map(Some)?,
          "ShippingAddress" => order.ShippingAddress = fold_elements(s, ShippingAddress::default(), |s, addr| {
            match s.local_name() {
              "StateOrRegion" => addr.StateOrRegion = characters(s)?,
              "City" => addr.City = characters(s)?,
              "CountryCode" => addr.CountryCode = characters(s)?,
              "PostalCode" => addr.PostalCode = characters(s)?,
              "Name" => addr.Name = characters(s)?,
              "AddressLine1" => addr.AddressLine1 = characters(s)?,
              "AddressLine2" => addr.AddressLine2 = characters(s)?,
              _ => {},
            }
            Ok(())
          }).map(Some)?,
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

#[cfg(test)]
mod tests {
  use super::*;
  use xmlhelper::decode;
  use xmlhelper::decode::FromXMLStream;
  use std::io::Cursor;

  #[test]
  fn test_decode_order() {
      let mut s = decode::Stream::new(Cursor::new(r#"      <Order>
        <LatestShipDate>2016-11-03T00:09:40Z</LatestShipDate>
        <OrderType>StandardOrder</OrderType>
        <PurchaseDate>2016-11-01T05:01:22Z</PurchaseDate>
        <BuyerEmail>test@marketplace.amazon.com</BuyerEmail>
        <AmazonOrderId>104-8343004-0000000</AmazonOrderId>
        <LastUpdateDate>2016-11-03T00:12:39Z</LastUpdateDate>
        <ShipServiceLevel>SecondDay</ShipServiceLevel>
        <NumberOfItemsShipped>1</NumberOfItemsShipped>
        <OrderStatus>Shipped</OrderStatus>
        <SalesChannel>Amazon.com</SalesChannel>
        <IsBusinessOrder>false</IsBusinessOrder>
        <NumberOfItemsUnshipped>0</NumberOfItemsUnshipped>
        <BuyerName>First Last</BuyerName>
        <OrderTotal>
          <CurrencyCode>USD</CurrencyCode>
          <Amount>249.99</Amount>
        </OrderTotal>
        <IsPremiumOrder>false</IsPremiumOrder>
        <EarliestShipDate>2016-11-03T00:09:40Z</EarliestShipDate>
        <MarketplaceId>MMMMMMMMMMMMM</MarketplaceId>
        <FulfillmentChannel>AFN</FulfillmentChannel>
        <PaymentMethod>Other</PaymentMethod>
        <ShippingAddress>
          <StateOrRegion>CA</StateOrRegion>
          <City>SAN MATEO</City>
          <CountryCode>US</CountryCode>
          <PostalCode>88888-7777</PostalCode>
          <Name>First Last</Name>
          <AddressLine1>2 RANDOM ROAD APT 001</AddressLine1>
        </ShippingAddress>
        <IsPrime>false</IsPrime>
        <ShipmentServiceLevelCategory>SecondDay</ShipmentServiceLevelCategory>
        <SellerOrderId>104-8343004-0000000</SellerOrderId>
      </Order>"#));

      decode::start_document(&mut s).expect("start element");
      let order = Order::from_xml(&mut s).expect("decode order");
      assert_eq!(order, Order {
        LatestShipDate: Some("2016-11-03T00:09:40Z".parse().expect("parse LatestShipDate")),
        OrderType: "StandardOrder".to_string(),
        PurchaseDate: Some("2016-11-01T05:01:22Z".parse().expect("parse PurchaseDate")),
        AmazonOrderId: "104-8343004-0000000".to_string(),
        LastUpdateDate: Some("2016-11-03T00:12:39Z".parse().expect("parse LastUpdateDate")),
        ShipServiceLevel: "SecondDay".to_string(),
        NumberOfItemsShipped: 1,
        OrderStatus: OrderStatus::Shipped,
        SalesChannel: "Amazon.com".to_string(),
        IsBusinessOrder: false,
        NumberOfItemsUnshipped: 0,
        IsPremiumOrder: false,
        EarliestShipDate: Some("2016-11-03T00:09:40Z".parse().expect("parse EarliestShipDate")),
        MarketplaceId: "MMMMMMMMMMMMM".to_string(),
        FulfillmentChannel: FulfillmentChannel::AFN,
        PaymentMethod: PaymentMethod::Other,
        IsPrime: false,
        ShipmentServiceLevelCategory: "SecondDay".to_string(),
        SellerOrderId: "104-8343004-0000000".to_string(),
        BuyerEmail: "test@marketplace.amazon.com".to_string(),
        BuyerName: "First Last".to_string(),
        OrderTotal: Some(OrderTotal {
          CurrencyCode: "USD".to_string(),
          Amount: 249.99,
        }),
        ShippingAddress: Some(ShippingAddress {
          StateOrRegion: "CA".to_string(),
          City: "SAN MATEO".to_string(),
          CountryCode: "US".to_string(),
          PostalCode: "88888-7777".to_string(),
          Name: "First Last".to_string(),
          AddressLine1: "2 RANDOM ROAD APT 001".to_string(),
          AddressLine2: "".to_string(),
        }),
      });
  }
}