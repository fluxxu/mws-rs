use chrono::{DateTime, Utc};

#[allow(non_snake_case)]
#[derive(Debug, Default, PartialEq, Serialize, FromXmlStream)]
pub struct ShippingAddress {
  pub StateOrRegion: String,
  pub City: String,
  pub CountryCode: String,
  pub PostalCode: String,
  pub Name: String,
  pub AddressLine1: String,
  pub AddressLine2: String,
  pub Phone: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Debug, Default, PartialEq, Serialize, FromXmlStream)]
pub struct CurrencyAmount {
  pub CurrencyCode: String,
  pub Amount: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Default, PartialEq, Serialize, FromXmlStream)]
pub struct Order {
  /// An Amazon-defined order identifier, in 3-7-7 format.
  pub AmazonOrderId: String,
  /// A seller-defined order identifier.
  pub SellerOrderId: String,
  /// The date when the order was created.
  pub PurchaseDate: Option<DateTime<Utc>>,
  /// The date when the order was last updated.
  pub LastUpdateDate: Option<DateTime<Utc>>,
  /// The current order status.
  pub OrderStatus: OrderStatus,
  /// How the order was fulfilled: by Amazon (AFN) or by the seller (MFN).
  pub FulfillmentChannel: FulfillmentChannel,
  /// The sales channel of the first item in the order.
  pub SalesChannel: String,
  /// The order channel of the first item in the order.
  pub OrderChannel: String,
  /// The shipment service level of the order.
  pub ShipServiceLevel: String,
  /// The shipping address for the order.
  pub ShippingAddress: Option<ShippingAddress>,
  /// The total charge for the order.
  pub OrderTotal: Option<CurrencyAmount>,
  /// The number of items shipped.
  pub NumberOfItemsShipped: i32,
  /// The number of items unshipped.
  pub NumberOfItemsUnshipped: i32,

  // pub PaymentExecutionDetail: ?
  /// The payment method for the order.
  /// This response element is limited to Cash On Delivery (COD)
  /// and Convenience Store (CVS) payment methods.
  /// Unless you need the specific COD payment information provided
  /// by the PaymentExecutionDetailItem element, we recommend using
  /// the PaymentMethodDetails response element to get payment
  /// method information.
  pub PaymentMethod: PaymentMethod,

  // pub PaymentMethodDetails: ?
  /// true if this is a replacement order.
  pub IsReplacementOrder: bool,
  /// The AmazonOrderId value for the order that is being replaced.
  pub ReplacedOrderId: String,
  /// The anonymized identifier for the Marketplace where the order was placed.
  pub MarketplaceId: String,
  /// The anonymized e-mail address of the buyer.
  pub BuyerEmail: String,
  /// The name of the buyer.
  pub BuyerName: String,
  /// The county of the buyer.
  /// This element is used only in the Brazil marketplace.
  pub BuyerCounty: String,

  // pub BuyerTaxInfo: ?
  /// The shipment service level category of the order.
  /// ShipmentServiceLevelCategory values: Expedited, FreeEconomy, NextDay, SameDay, SecondDay, Scheduled, Standard
  pub ShipmentServiceLevelCategory: String,
  /// true if the order was shipped by the Amazon Transportation for Merchants (Amazon TFM) service.
  /// Amazon TFM is available only in the China marketplace.
  pub ShippedByAmazonTFM: bool,
  /// The status of the Amazon TFM order. Returned only if
  /// ShippedByAmazonTFM = True. Note that even if ShippedByAmazonTFM = True,
  /// TFMShipmentStatus will not be returned if you have
  /// not yet created the shipment.
  /// TFMShipmentStatus values: PendingPickUp, LabelCanceled, PickedUp,
  /// AtDestinationFC, Delivered, RejectedByBuyer, Undeliverable,
  /// ReturnedToSeller
  /// Amazon TFM is available only in the China marketplace.
  pub TFMShipmentStatus: String,
  /// The status of the Amazon Easy Ship order.
  /// This element is included only for Amazon Easy Ship orders.
  /// EasyShipShipmentStatus values: PendingPickUp, LabelCanceled,
  /// PickedUp, OutForDelivery, Damaged, Delivered, RejectedByBuyer,
  /// Undeliverable, ReturnedToSeller
  /// Amazon Easy Ship is available only in the India marketplace.
  pub EasyShipShipmentStatus: String,
  /// The type of the order.
  pub OrderType: String,
  /// The start of the time period that you have committed to
  /// ship the order. In ISO 8601 date time format.
  pub EarliestShipDate: Option<DateTime<Utc>>,
  /// The end of the time period that you have committed to ship
  /// the order. In ISO 8601 date time format.
  pub LatestShipDate: Option<DateTime<Utc>>,
  /// The start of the time period that you have commited to fulfill
  /// the order. In ISO 8601 date time format.
  pub EarliestDeliveryDate: Option<DateTime<Utc>>,
  /// The end of the time period that you have commited to fulfill the order.
  /// In ISO 8601 date time format.
  pub LatestDeliveryDate: Option<DateTime<Utc>>,
  /// true if the order is an Amazon Business order.
  /// An Amazon Business order is an order where the buyer is a
  /// Verified Business Buyer and the seller is an Amazon Business Seller.
  /// For more information about the Amazon Business Seller Program,
  /// see the Amazon Business website.
  pub IsBusinessOrder: bool,
  /// The purchase order (PO) number entered by the buyer at checkout.
  pub PurchaseOrderNumber: String,
  /// true if the order is a seller-fulfilled Amazon Prime order.
  pub IsPrime: bool,
  /// true if the order has a Premium Shipping Service Level Agreement.
  /// For more information about Premium Shipping orders,
  /// see "Premium Shipping Options" in the Seller Central Help for
  /// your marketplace.
  pub IsPremiumOrder: bool,
  /// Indicates the date by which the seller must respond to the buyer
  /// with an Estimated Ship Date.
  pub PromiseResponseDueDate: Option<DateTime<Utc>>,
  /// true if the Estimated Ship Date is set for the order.
  pub IsEstimatedShipDateSet: bool,
}

str_enum! {
  /// A list of OrderStatus values. Used to select orders with a current status that matches
  /// one of the status values that you specify.
  ///
  /// Unshipped and PartiallyShipped must be used together in this version of the Orders API
  /// section. Using one and not the other returns an error.
  ///
  /// [Reference](http://docs.developer.amazonservices.com/en_CA/orders-2013-09-01/Orders_ListOrders.html)
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

str_enum! {
  ///	A list that indicates how an order was fulfilled.
  ///
  /// [Reference](http://docs.developer.amazonservices.com/en_CA/orders-2013-09-01/Orders_ListOrders.html)
  pub enum FulfillmentChannel {
    AFN,
    MFN,
  }
}

str_enum! {
  /// A list of PaymentMethod values. Used to select orders paid for with the payment methods
  /// that you specify.
  ///
  /// Note: COD and CVS values are valid only in Japan (JP).
  pub enum PaymentMethod {
    COD,
    CVS,
    Other,
  }
}

str_enum! {
  /// A list of TFMShipmentStatus values. Used to select Amazon Transportation for Merchants (TFM)
  /// orders with a current status that matches one of the status values that you specify.
  /// If TFMShipmentStatus is specified, only TFM orders are returned.
  ///
  /// Note: The TFMShipmentStatus request parameter is available only in China (CN).
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
#[derive(Debug, Default, PartialEq, Serialize, FromXmlStream)]
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
  pub IsTransparency: bool,
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_decode_order() {
    test_decode!(
      Order,
      r#"
        <LatestShipDate>2018-12-18T07:59:59Z</LatestShipDate>
        <OrderType>StandardOrder</OrderType>
        <SellerOrderId>666</SellerOrderId>
        <PurchaseDate>2018-12-16T18:53:45.856Z</PurchaseDate>
        <AmazonOrderId>111-3081581-6666666</AmazonOrderId>
        <BuyerEmail>666@marketplace.amazon.com</BuyerEmail>
        <IsReplacementOrder>false</IsReplacementOrder>
        <LastUpdateDate>2018-12-17T16:50:11.820Z</LastUpdateDate>
        <NumberOfItemsShipped>1</NumberOfItemsShipped>
        <ShipServiceLevel>Std US D2D Dom</ShipServiceLevel>
        <OrderStatus>Shipped</OrderStatus>
        <SalesChannel>Amazon.com</SalesChannel>
        <ShippedByAmazonTFM>false</ShippedByAmazonTFM>
        <IsBusinessOrder>false</IsBusinessOrder>
        <LatestDeliveryDate>2018-12-27T07:59:59Z</LatestDeliveryDate>
        <NumberOfItemsUnshipped>0</NumberOfItemsUnshipped>
        <PaymentMethodDetails>
          <PaymentMethodDetail>Standard</PaymentMethodDetail>
        </PaymentMethodDetails>
        <BuyerName>First Last</BuyerName>
        <EarliestDeliveryDate>2018-12-20T08:00:00Z</EarliestDeliveryDate>
        <OrderTotal>
          <CurrencyCode>USD</CurrencyCode>
          <Amount>666.66</Amount>
        </OrderTotal>
        <IsPremiumOrder>false</IsPremiumOrder>
        <EarliestShipDate>2018-12-17T08:00:00Z</EarliestShipDate>
        <MarketplaceId>ATVPDKIKX0DER</MarketplaceId>
        <FulfillmentChannel>MFN</FulfillmentChannel>
        <PaymentMethod>Other</PaymentMethod>
        <ShippingAddress>
          <City>SAN MATEO</City>
          <AddressType>Residential</AddressType>
          <PostalCode>92683-4617</PostalCode>
          <isAddressSharingConfidential>false</isAddressSharingConfidential>
          <StateOrRegion>CA</StateOrRegion>
          <CountryCode>US</CountryCode>
          <Name>First Last</Name>
          <AddressLine1>8888 J AVE</AddressLine1>
        </ShippingAddress>
        <IsPrime>true</IsPrime>
        <ShipmentServiceLevelCategory>Standard</ShipmentServiceLevelCategory>"#,
      Order {
        LatestShipDate: Some(
          "2018-12-18T07:59:59Z"
            .parse()
            .expect("parse LatestShipDate",)
        ),
        OrderType: "StandardOrder".to_string(),
        PurchaseDate: Some(
          "2018-12-16T18:53:45.856Z"
            .parse()
            .expect("parse PurchaseDate")
        ),
        AmazonOrderId: "111-3081581-6666666".to_string(),
        LastUpdateDate: Some(
          "2018-12-17T16:50:11.820Z"
            .parse()
            .expect("parse LastUpdateDate",)
        ),
        ShipServiceLevel: "Std US D2D Dom".to_string(),
        NumberOfItemsShipped: 1,
        OrderStatus: OrderStatus::Shipped,
        SalesChannel: "Amazon.com".to_string(),
        IsBusinessOrder: false,
        NumberOfItemsUnshipped: 0,
        IsPremiumOrder: false,
        EarliestShipDate: Some(
          "2018-12-17T08:00:00Z"
            .parse()
            .expect("parse EarliestShipDate",)
        ),
        MarketplaceId: "ATVPDKIKX0DER".to_string(),
        FulfillmentChannel: FulfillmentChannel::MFN,
        PaymentMethod: PaymentMethod::Other,
        IsPrime: true,
        ShipmentServiceLevelCategory: "Standard".to_string(),
        SellerOrderId: "666".to_string(),
        BuyerEmail: "666@marketplace.amazon.com".to_string(),
        BuyerName: "First Last".to_string(),
        OrderTotal: Some(CurrencyAmount {
          CurrencyCode: "USD".to_string(),
          Amount: "666.66".to_owned(),
        }),
        ShippingAddress: Some(ShippingAddress {
          Phone: None,
          StateOrRegion: "CA".to_string(),
          City: "SAN MATEO".to_string(),
          CountryCode: "US".to_string(),
          PostalCode: "92683-4617".to_string(),
          Name: "First Last".to_string(),
          AddressLine1: "8888 J AVE".to_string(),
          AddressLine2: "".to_string(),
        }),
        BuyerCounty: "".to_string(),
        EarliestDeliveryDate: Some("2018-12-20T08:00:00Z".parse().unwrap()),
        LatestDeliveryDate: Some("2018-12-27T07:59:59Z".parse().unwrap()),
        EasyShipShipmentStatus: "".to_owned(),
        IsEstimatedShipDateSet: false,
        IsReplacementOrder: false,
        ReplacedOrderId: "".to_owned(),
        OrderChannel: "".to_owned(),
        PromiseResponseDueDate: None,
        PurchaseOrderNumber: "".to_owned(),
        ShippedByAmazonTFM: false,
        TFMShipmentStatus: "".to_owned(),
      }
    );
  }

  #[test]
  fn test_decode_orderitem() {
    test_decode!(
      OrderItem,
      r#"<QuantityOrdered>1</QuantityOrdered>
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
        </ItemTax>"#,
      OrderItem {
        OrderItemId: "46510268396154".to_string(),
        QuantityOrdered: 1,
        Title: "Edifier R1280T Powered Bookshelf Speakers - 2.0 Active Near Field Monitors - Studio Monitor Speaker - Wooden Enclosure - 42 Watts RMS".to_string(),
        ASIN: "B016P9HJIA".to_string(),
        SellerSKU: "edifier-r1280t-fba".to_string(),
        QuantityShipped: 1,
        ItemPrice: Some(CurrencyAmount { CurrencyCode: "USD".to_string(), Amount: "99.99".to_owned() }),
        ItemTax: Some(CurrencyAmount { CurrencyCode: "USD".to_string(), Amount: "0.00".to_owned() }),
        GiftWrapPrice: None,
        GiftWrapTax: None,
        PromotionDiscount: Some(CurrencyAmount { CurrencyCode: "USD".to_string(), Amount: "0.00".to_owned() }),
        ShippingPrice: None,
        ShippingDiscount: None,
        ShippingTax: None,
      }
    );
  }
}
