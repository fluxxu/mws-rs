use chrono::{DateTime, Utc};
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
pub struct CurrencyAmount {
  pub CurrencyCode: String,
  pub Amount: f64,
}

#[allow(non_snake_case)]
#[derive(Debug, Default, PartialEq)]
pub struct Order {
  pub LatestShipDate: Option<DateTime<Utc>>,
  pub OrderType: String,
  pub PurchaseDate: Option<DateTime<Utc>>,
  pub AmazonOrderId: String,
  pub LastUpdateDate: Option<DateTime<Utc>>,
  pub ShipServiceLevel: String,
  pub NumberOfItemsShipped: i32,
  pub OrderStatus: OrderStatus,
  pub SalesChannel: String,
  pub IsBusinessOrder: bool,
  pub NumberOfItemsUnshipped: i32,
  pub IsPremiumOrder: bool,
  pub EarliestShipDate: Option<DateTime<Utc>>,
  pub MarketplaceId: String,
  pub FulfillmentChannel: FulfillmentChannel,
  pub PaymentMethod: PaymentMethod,
  pub IsPrime: bool,
  pub ShipmentServiceLevelCategory: String,
  pub SellerOrderId: String,
  pub BuyerEmail: String,
  pub BuyerName: String,
  pub OrderTotal: Option<CurrencyAmount>,
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
          "OrderTotal" => order.OrderTotal = fold_elements(s, CurrencyAmount::default(), |s, total| {
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

#[allow(non_snake_case)]
#[derive(Debug, Default, PartialEq)]
pub struct OrderItem {
  pub OrderItemId: String,
  pub QuantityOrdered: i32,
  pub Title: String,
  pub ASIN: String,
  pub SellerSKU: String,
  pub QuantityShipped: i32,
  pub ItemPrice: Option<CurrencyAmount>,
  pub ItemTax: Option<CurrencyAmount>,
  pub GiftWrapPrice: Option<CurrencyAmount>,
  pub GiftWrapTax: Option<CurrencyAmount>,
  pub PromotionDiscount: Option<CurrencyAmount>,
  pub ShippingPrice: Option<CurrencyAmount>,
  pub ShippingDiscount: Option<CurrencyAmount>,
  pub ShippingTax: Option<CurrencyAmount>,
}

impl<S: decode::XmlEventStream> decode::FromXMLStream<S> for OrderItem {
  fn from_xml(s: &mut S) -> decode::Result<OrderItem> {
    use xmlhelper::decode::{element, fold_elements, characters};
    element(s, "OrderItem", |s| {
      fold_elements(s, OrderItem::default(), |s, item| {
        match s.local_name() {
          "OrderItemId" => item.OrderItemId = characters(s)?,
          "QuantityOrdered" => item.QuantityOrdered = characters(s)?,
          "Title" => item.Title = characters(s)?,
          "ASIN" => item.ASIN = characters(s)?,
          "SellerSKU" => item.SellerSKU = characters(s)?,
          "QuantityShipped" => item.QuantityShipped = characters(s)?,
          "ItemPrice" => item.ItemPrice = fold_elements(s, CurrencyAmount::default(), |s, amount| {
            match s.local_name() {
              "CurrencyCode" => amount.CurrencyCode = characters(s)?,
              "Amount" => amount.Amount = characters(s)?,
              _ => {},
            }
            Ok(())
          }).map(Some)?,
          "ItemTax" => item.ItemTax = fold_elements(s, CurrencyAmount::default(), |s, amount| {
            match s.local_name() {
              "CurrencyCode" => amount.CurrencyCode = characters(s)?,
              "Amount" => amount.Amount = characters(s)?,
              _ => {},
            }
            Ok(())
          }).map(Some)?,
          "GiftWrapPrice" => item.GiftWrapPrice = fold_elements(s, CurrencyAmount::default(), |s, amount| {
            match s.local_name() {
              "CurrencyCode" => amount.CurrencyCode = characters(s)?,
              "Amount" => amount.Amount = characters(s)?,
              _ => {},
            }
            Ok(())
          }).map(Some)?,
          "GiftWrapTax" => item.GiftWrapTax = fold_elements(s, CurrencyAmount::default(), |s, amount| {
            match s.local_name() {
              "CurrencyCode" => amount.CurrencyCode = characters(s)?,
              "Amount" => amount.Amount = characters(s)?,
              _ => {},
            }
            Ok(())
          }).map(Some)?,
          "PromotionDiscount" => item.PromotionDiscount = fold_elements(s, CurrencyAmount::default(), |s, amount| {
            match s.local_name() {
              "CurrencyCode" => amount.CurrencyCode = characters(s)?,
              "Amount" => amount.Amount = characters(s)?,
              _ => {},
            }
            Ok(())
          }).map(Some)?,
          "ShippingPrice" => item.ShippingPrice = fold_elements(s, CurrencyAmount::default(), |s, amount| {
            match s.local_name() {
              "CurrencyCode" => amount.CurrencyCode = characters(s)?,
              "Amount" => amount.Amount = characters(s)?,
              _ => {},
            }
            Ok(())
          }).map(Some)?,
          "ShippingDiscount" => item.ShippingDiscount = fold_elements(s, CurrencyAmount::default(), |s, amount| {
            match s.local_name() {
              "CurrencyCode" => amount.CurrencyCode = characters(s)?,
              "Amount" => amount.Amount = characters(s)?,
              _ => {},
            }
            Ok(())
          }).map(Some)?,
          "ShippingTax" => item.ShippingTax = fold_elements(s, CurrencyAmount::default(), |s, amount| {
            match s.local_name() {
              "CurrencyCode" => amount.CurrencyCode = characters(s)?,
              "Amount" => amount.Amount = characters(s)?,
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
        OrderTotal: Some(CurrencyAmount {
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

  #[test]
  fn test_decode_orderitem() {
    let mut s = decode::Stream::new(Cursor::new(r#"<OrderItem>
        <QuantityOrdered>1</QuantityOrdered>
        <Title>Edifier R1280T Powered Bookshelf Speakers - 2.0 Active Near Field Monitors - Studio Monitor Speaker - Wooden Enclosure - 42 Watts RMS</Title>
        <PromotionDiscount>
          <CurrencyCode>USD</CurrencyCode>
          <Amount>0.00</Amount>
        </PromotionDiscount>
        <ASIN>B016P9HJIA</ASIN>
        <SellerSKU>edifier-r1280t-fba</SellerSKU>
        <OrderItemId>46510268396154</OrderItemId>
        <QuantityShipped>1</QuantityShipped>
        <ItemPrice>
          <CurrencyCode>USD</CurrencyCode>
          <Amount>99.99</Amount>
        </ItemPrice>
        <ItemTax>
          <CurrencyCode>USD</CurrencyCode>
          <Amount>0.00</Amount>
        </ItemTax>
      </OrderItem>"#));

    decode::start_document(&mut s).expect("start element");
    let item = OrderItem::from_xml(&mut s).expect("decode order item");
    assert_eq!(item, OrderItem {
      OrderItemId: "46510268396154".to_string(),
      QuantityOrdered: 1,
      Title: "Edifier R1280T Powered Bookshelf Speakers - 2.0 Active Near Field Monitors - Studio Monitor Speaker - Wooden Enclosure - 42 Watts RMS".to_string(),
      ASIN: "B016P9HJIA".to_string(),
      SellerSKU: "edifier-r1280t-fba".to_string(),
      QuantityShipped: 1,
      ItemPrice: Some(CurrencyAmount { CurrencyCode: "USD".to_string(), Amount: 99.99 }),
      ItemTax: Some(CurrencyAmount { CurrencyCode: "USD".to_string(), Amount: 0.00 }),
      GiftWrapPrice: None,
      GiftWrapTax: None,
      PromotionDiscount: Some(CurrencyAmount { CurrencyCode: "USD".to_string(), Amount: 0.00 }),
      ShippingPrice: None,
      ShippingDiscount: None,
      ShippingTax: None,
    });
  }
}