//! https://docs.developer.amazonservices.com/en_US/products/Products_Datatypes.html

use chrono::{DateTime, Utc};

str_enum! {
  pub enum ItemCondition {
    New,
    Used,
    Collectible,
    Refurbished,
    Club,
  }
}

str_enum! {
  pub enum AvailabilityType {
    NOW,
    FUTURE_WITHOUT_DATE,
    FUTURE_WITH_DATE,
  }
}

#[allow(non_snake_case)]
#[derive(Debug, Default, PartialEq, Serialize, FromXmlStream)]
pub struct Identifier {
  pub MarketplaceId: String,
  pub SellerSKU: String,
  pub ItemCondition: ItemCondition,
  pub TimeOfOfferChange: Option<DateTime<Utc>>,
}

#[allow(non_snake_case)]
#[derive(Debug, Default, PartialEq, Serialize, FromXmlStream)]
pub struct OfferCount {
  #[from_xml_stream(from_attr = "condition")]
  pub Condition: String,
  #[from_xml_stream(from_attr = "fulfillmentChannel")]
  pub FulfillmentChannel: String,
  #[from_xml_stream(from_content)]
  pub Value: i32,
}

#[allow(non_snake_case)]
#[derive(Debug, Default, PartialEq, Serialize, FromXmlStream)]
pub struct MoneyType {
  pub Amount: String,
  pub CurrencyCode: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Default, PartialEq, Serialize, FromXmlStream)]
pub struct Points {
  pub PointsNumber: i32,
  pub PointsMonetaryValue: MoneyType,
}

#[allow(non_snake_case)]
#[derive(Debug, Default, PartialEq, Serialize, FromXmlStream)]
pub struct LowestPrice {
  #[from_xml_stream(from_attr = "condition")]
  pub Condition: String,
  #[from_xml_stream(from_attr = "fulfillmentChannel")]
  pub FulfillmentChannel: String,
  pub LandedPrice: MoneyType,
  pub ListingPrice: MoneyType,
  pub Shipping: MoneyType,
  pub Points: Option<Points>,
}

#[allow(non_snake_case)]
#[derive(Debug, Default, PartialEq, Serialize, FromXmlStream)]
pub struct BuyBoxPrice {
  #[from_xml_stream(from_attr = "condition")]
  pub Condition: String,
  pub LandedPrice: MoneyType,
  pub ListingPrice: MoneyType,
  pub Shipping: MoneyType,
}

#[allow(non_snake_case)]
#[derive(Debug, Default, PartialEq, Serialize, FromXmlStream)]
pub struct Summary {
  pub TotalOfferCount: i32,
  pub NumberOfOffers: Vec<OfferCount>,
  pub LowestPrices: Vec<LowestPrice>,
  pub BuyBoxPrices: Vec<BuyBoxPrice>,
  pub BuyBoxEligibleOffers: Vec<OfferCount>,
}

#[allow(non_snake_case)]
#[derive(Debug, Default, PartialEq, Serialize, FromXmlStream)]
pub struct SellerFeedbackRating {
  pub SellerPositiveFeedbackRating: Option<String>,
  pub FeedbackCount: i32,
}

#[allow(non_snake_case)]
#[derive(Debug, Default, PartialEq, Serialize, FromXmlStream)]
pub struct ShippingTime {
  #[from_xml_stream(from_attr = "minimumHours")]
  pub MinimumHours: Option<i32>,
  #[from_xml_stream(from_attr = "maximumHours")]
  pub MaximumHours: Option<i32>,
  #[from_xml_stream(from_attr = "availabilityDate")]
  pub AvailableDate: Option<DateTime<Utc>>,
  #[from_xml_stream(from_attr = "availabilityType")]
  pub AvailabilityType: Option<AvailabilityType>,
}

#[allow(non_snake_case)]
#[derive(Debug, Default, PartialEq, Serialize, FromXmlStream)]
pub struct ShipsFrom {
  pub State: String,
  pub Country: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Default, PartialEq, Serialize, FromXmlStream)]
pub struct Offer {
  pub MyOffer: bool,
  pub SubCondition: String,
  pub SellerFeedbackRating: SellerFeedbackRating,
  pub ShippingTime: ShippingTime,
  pub ListingPrice: MoneyType,
  pub Shipping: MoneyType,
  pub ShipsFrom: Option<ShipsFrom>,
  pub IsFulfilledByAmazon: bool,
  pub IsBuyBoxWinner: bool,
  pub IsFeaturedMerchant: bool,
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_summary() {
    test_decode!(
      Summary,
      r#"
        <TotalOfferCount>9</TotalOfferCount>
        <NumberOfOffers>
            <OfferCount condition="new" fulfillmentChannel="Amazon">3</OfferCount>
            <OfferCount condition="new" fulfillmentChannel="Merchant">6</OfferCount>
        </NumberOfOffers>
        <LowestPrices>
            <LowestPrice condition="new" fulfillmentChannel="Amazon">
                <LandedPrice>
                    <CurrencyCode>GBP</CurrencyCode>
                    <Amount>239.99</Amount>
                </LandedPrice>
                <ListingPrice>
                    <CurrencyCode>GBP</CurrencyCode>
                    <Amount>239.99</Amount>
                </ListingPrice>
                <Shipping>
                    <CurrencyCode>GBP</CurrencyCode>
                    <Amount>0.00</Amount>
                </Shipping>
            </LowestPrice>
            <LowestPrice condition="new" fulfillmentChannel="Merchant">
                <LandedPrice>
                    <CurrencyCode>GBP</CurrencyCode>
                    <Amount>239.95</Amount>
                </LandedPrice>
                <ListingPrice>
                    <CurrencyCode>GBP</CurrencyCode>
                    <Amount>239.95</Amount>
                </ListingPrice>
                <Shipping>
                    <CurrencyCode>GBP</CurrencyCode>
                    <Amount>0.00</Amount>
                </Shipping>
            </LowestPrice>
        </LowestPrices>
        <BuyBoxPrices>
            <BuyBoxPrice condition="New">
                <LandedPrice>
                    <CurrencyCode>GBP</CurrencyCode>
                    <Amount>239.99</Amount>
                </LandedPrice>
                <ListingPrice>
                    <CurrencyCode>GBP</CurrencyCode>
                    <Amount>239.99</Amount>
                </ListingPrice>
                <Shipping>
                    <CurrencyCode>GBP</CurrencyCode>
                    <Amount>0.00</Amount>
                </Shipping>
            </BuyBoxPrice>
        </BuyBoxPrices>
        <BuyBoxEligibleOffers>
            <OfferCount condition="new" fulfillmentChannel="Amazon">3</OfferCount>
            <OfferCount condition="new" fulfillmentChannel="Merchant">3</OfferCount>
        </BuyBoxEligibleOffers>
      "#,
      Summary {
        TotalOfferCount: 9,
        NumberOfOffers: vec![
          OfferCount {
            Condition: "new".to_string(),
            FulfillmentChannel: "Amazon".to_string(),
            Value: 3,
          },
          OfferCount {
            Condition: "new".to_string(),
            FulfillmentChannel: "Merchant".to_string(),
            Value: 6,
          },
        ],
        LowestPrices: vec![
          LowestPrice {
            Condition: "new".to_string(),
            FulfillmentChannel: "Amazon".to_string(),
            LandedPrice: MoneyType {
              CurrencyCode: "GBP".to_string(),
              Amount: "239.99".to_string(),
            },
            ListingPrice: MoneyType {
              CurrencyCode: "GBP".to_string(),
              Amount: "239.99".to_string(),
            },
            Shipping: MoneyType {
              CurrencyCode: "GBP".to_string(),
              Amount: "0.00".to_string(),
            },
            Points: None,
          },
          LowestPrice {
            Condition: "new".to_string(),
            FulfillmentChannel: "Merchant".to_string(),
            LandedPrice: MoneyType {
              CurrencyCode: "GBP".to_string(),
              Amount: "239.95".to_string(),
            },
            ListingPrice: MoneyType {
              CurrencyCode: "GBP".to_string(),
              Amount: "239.95".to_string(),
            },
            Shipping: MoneyType {
              CurrencyCode: "GBP".to_string(),
              Amount: "0.00".to_string(),
            },
            Points: None,
          }
        ],
        BuyBoxPrices: vec![BuyBoxPrice {
          Condition: "New".to_string(),
          LandedPrice: MoneyType {
            CurrencyCode: "GBP".to_string(),
            Amount: "239.99".to_string(),
          },
          ListingPrice: MoneyType {
            CurrencyCode: "GBP".to_string(),
            Amount: "239.99".to_string(),
          },
          Shipping: MoneyType {
            CurrencyCode: "GBP".to_string(),
            Amount: "0.00".to_string(),
          },
        },],
        BuyBoxEligibleOffers: vec![
          OfferCount {
            Condition: "new".to_string(),
            FulfillmentChannel: "Amazon".to_string(),
            Value: 3,
          },
          OfferCount {
            Condition: "new".to_string(),
            FulfillmentChannel: "Merchant".to_string(),
            Value: 3,
          },
        ],
      }
    );
  }

  #[test]
  fn test_offer() {
    test_decode!(
      Offer,
      r#"
        <MyOffer>false</MyOffer>
        <SubCondition>new</SubCondition>
        <SellerFeedbackRating>
            <SellerPositiveFeedbackRating>95.0</SellerPositiveFeedbackRating>
            <FeedbackCount>618</FeedbackCount>
        </SellerFeedbackRating>
        <ShippingTime minimumHours="24" maximumHours="24" availabilityType="NOW"/>
        <ListingPrice>
            <CurrencyCode>GBP</CurrencyCode>
            <Amount>239.95</Amount>
        </ListingPrice>
        <Shipping>
            <CurrencyCode>GBP</CurrencyCode>
            <Amount>0.00</Amount>
        </Shipping>
        <ShipsFrom>
            <Country>GB</Country>
        </ShipsFrom>
        <IsFulfilledByAmazon>false</IsFulfilledByAmazon>
        <IsBuyBoxWinner>false</IsBuyBoxWinner>
        <IsFeaturedMerchant>true</IsFeaturedMerchant>
      "#,
      Offer {
        MyOffer: false,
        SubCondition: "new".to_string(),
        SellerFeedbackRating: SellerFeedbackRating {
          SellerPositiveFeedbackRating: Some("95.0".to_string()),
          FeedbackCount: 618,
        },
        ShippingTime: ShippingTime {
          MinimumHours: Some(24),
          MaximumHours: Some(24),
          AvailableDate: None,
          AvailabilityType: Some(AvailabilityType::NOW),
        },
        ListingPrice: MoneyType {
          CurrencyCode: "GBP".to_string(),
          Amount: "239.95".to_string(),
        },
        Shipping: MoneyType {
          CurrencyCode: "GBP".to_string(),
          Amount: "0.00".to_string(),
        },
        ShipsFrom: Some(ShipsFrom {
          Country: "GB".to_string(),
          ..Default::default()
        }),
        IsFulfilledByAmazon: false,
        IsBuyBoxWinner: false,
        IsFeaturedMerchant: true,
      }
    );
  }
}
