pub use crate::products::{
  AvailabilityType, BuyBoxPrice, LowestPrice, MoneyType, OfferCount, SellerFeedbackRating,
  ShippingTime, ShipsFrom,
};
use chrono::{DateTime, Utc};

#[allow(non_snake_case)]
#[derive(Debug, Default, PartialEq, Serialize, FromXmlStream)]
pub struct AnyOfferChangedNotification {
  pub OfferChangeTrigger: OfferChangeTrigger,
  pub Summary: Summary,
  pub Offers: Vec<Offer>,
}

str_enum! {
  pub enum OfferChangeType {
    External,
    Internal,
  }
}

#[allow(non_snake_case)]
#[derive(Debug, Default, PartialEq, Serialize, FromXmlStream)]
pub struct OfferChangeTrigger {
  pub MarketplaceId: String,
  pub ASIN: String,
  pub ItemCondition: String,
  pub TimeOfOfferChange: Option<DateTime<Utc>>,
  pub OfferChangeType: OfferChangeType,
}

#[allow(non_snake_case)]
#[derive(Debug, Default, PartialEq, Serialize, FromXmlStream)]
pub struct SalesRank {
  pub ProductCategoryId: String,
  pub Rank: i32,
}

#[allow(non_snake_case)]
#[derive(Debug, Default, PartialEq, Serialize, FromXmlStream)]
pub struct Points {
  pub PointsNumber: i32,
}

#[allow(non_snake_case)]
#[derive(Debug, Default, PartialEq, Serialize, FromXmlStream)]
pub struct PrimeInformation {
  pub IsNationalPrime: bool,
  pub IsPrime: bool,
}

#[allow(non_snake_case)]
#[derive(Debug, Default, PartialEq, Serialize, FromXmlStream)]
pub struct Offer {
  pub SellerId: String,
  pub SubCondition: String,
  pub SellerFeedbackRating: SellerFeedbackRating,
  pub ShippingTime: ShippingTime,
  pub ListingPrice: MoneyType,
  pub Points: Option<Points>,
  pub Shipping: MoneyType,
  pub ShipsFrom: Option<ShipsFrom>,
  pub IsFulfilledByAmazon: bool,
  pub IsBuyBoxWinner: Option<bool>,
  pub ConditionNotes: Option<String>,
  pub PrimeInformation: Option<PrimeInformation>,
  pub IsExpeditedShippingAvailable: Option<bool>,
  pub IsFeaturedMerchant: Option<bool>,
  pub ShipsDomestically: Option<bool>,
}

#[allow(non_snake_case)]
#[derive(Debug, Default, PartialEq, Serialize, FromXmlStream)]
pub struct Summary {
  pub NumberOfOffers: Vec<OfferCount>,
  pub LowestPrices: Vec<LowestPrice>,
  pub BuyBoxPrices: Vec<BuyBoxPrice>,
  pub ListPrice: Option<MoneyType>,
  pub SuggestedLowerPricePlusShipping: Option<MoneyType>,
  pub SalesRankings: Vec<SalesRank>,
  pub BuyBoxEligibleOffers: Vec<OfferCount>,
  pub CompetitivePriceThreshold: Option<MoneyType>,
}

#[cfg(test)]
mod tests {
  use super::*;
  use chrono::{TimeZone, Utc};

  #[test]
  fn test_any_offer_changed_notification() {
    test_decode!(
      AnyOfferChangedNotification,
      r#"
      <OfferChangeTrigger>
          <MarketplaceId>ATVPDKIKX0DER</MarketplaceId>
          <ASIN>B0000C000V</ASIN>
          <ItemCondition>new</ItemCondition>
          <TimeOfOfferChange>2020-02-03T21:50:09.000Z</TimeOfOfferChange>
          <OfferChangeType>External</OfferChangeType>
      </OfferChangeTrigger>
      <Summary>
          <NumberOfOffers>
              <OfferCount condition="new" fulfillmentChannel="Merchant">1</OfferCount>
              <OfferCount condition="new" fulfillmentChannel="Amazon">1</OfferCount>
              <OfferCount condition="used" fulfillmentChannel="Amazon">5</OfferCount>
          </NumberOfOffers>
          <LowestPrices>
              <LowestPrice condition="new" fulfillmentChannel="Merchant">
                  <LandedPrice>
                      <Amount>129.99</Amount>
                      <CurrencyCode>USD</CurrencyCode>
                  </LandedPrice>
                  <ListingPrice>
                      <Amount>129.99</Amount>
                      <CurrencyCode>USD</CurrencyCode>
                  </ListingPrice>
                  <Shipping>
                      <Amount>0.00</Amount>
                      <CurrencyCode>USD</CurrencyCode>
                  </Shipping>
              </LowestPrice>
              <LowestPrice condition="new" fulfillmentChannel="Amazon">
                  <LandedPrice>
                      <Amount>129.99</Amount>
                      <CurrencyCode>USD</CurrencyCode>
                  </LandedPrice>
                  <ListingPrice>
                      <Amount>129.99</Amount>
                      <CurrencyCode>USD</CurrencyCode>
                  </ListingPrice>
                  <Shipping>
                      <Amount>0.00</Amount>
                      <CurrencyCode>USD</CurrencyCode>
                  </Shipping>
              </LowestPrice>
              <LowestPrice condition="used" fulfillmentChannel="Amazon">
                  <LandedPrice>
                      <Amount>119.59</Amount>
                      <CurrencyCode>USD</CurrencyCode>
                  </LandedPrice>
                  <ListingPrice>
                      <Amount>119.59</Amount>
                      <CurrencyCode>USD</CurrencyCode>
                  </ListingPrice>
                  <Shipping>
                      <Amount>0.00</Amount>
                      <CurrencyCode>USD</CurrencyCode>
                  </Shipping>
              </LowestPrice>
          </LowestPrices>
          <BuyBoxPrices>
              <BuyBoxPrice condition="new">
                  <LandedPrice>
                      <Amount>129.99</Amount>
                      <CurrencyCode>USD</CurrencyCode>
                  </LandedPrice>
                  <ListingPrice>
                      <Amount>129.99</Amount>
                      <CurrencyCode>USD</CurrencyCode>
                  </ListingPrice>
                  <Shipping>
                      <Amount>0.00</Amount>
                      <CurrencyCode>USD</CurrencyCode>
                  </Shipping>
              </BuyBoxPrice>
              <BuyBoxPrice condition="Used">
                  <LandedPrice>
                      <Amount>125.58</Amount>
                      <CurrencyCode>USD</CurrencyCode>
                  </LandedPrice>
                  <ListingPrice>
                      <Amount>119.59</Amount>
                      <CurrencyCode>USD</CurrencyCode>
                  </ListingPrice>
                  <Shipping>
                      <Amount>5.99</Amount>
                      <CurrencyCode>USD</CurrencyCode>
                  </Shipping>
              </BuyBoxPrice>
          </BuyBoxPrices>



          <SalesRankings>
              <SalesRank>
                  <ProductCategoryId>ce_display_on_website</ProductCategoryId>
                  <Rank>1346</Rank>
              </SalesRank>
              <SalesRank>
                  <ProductCategoryId>3236451011</ProductCategoryId>
                  <Rank>6</Rank>
              </SalesRank>
          </SalesRankings>

          <BuyBoxEligibleOffers>
              <OfferCount condition="new" fulfillmentChannel="Merchant">1</OfferCount>
              <OfferCount condition="new" fulfillmentChannel="Amazon">1</OfferCount>
              <OfferCount condition="used" fulfillmentChannel="Amazon">5</OfferCount>
          </BuyBoxEligibleOffers>
          <CompetitivePriceThreshold>
              <Amount>129.99</Amount>
              <CurrencyCode>USD</CurrencyCode>
          </CompetitivePriceThreshold>
      </Summary>
      <Offers>
          <Offer>
              <SellerId>A00AS0PFN0IRUQ</SellerId>
              <SubCondition>new</SubCondition>
              <SellerFeedbackRating>
                  <SellerPositiveFeedbackRating>99</SellerPositiveFeedbackRating>
                  <FeedbackCount>10536</FeedbackCount>
              </SellerFeedbackRating>
              <ShippingTime minimumHours="24" maximumHours="24" availabilityType="NOW"/>
              <ListingPrice>
                  <Amount>129.99</Amount>
                  <CurrencyCode>USD</CurrencyCode>
              </ListingPrice>
              <Shipping>
                  <Amount>0.00</Amount>
                  <CurrencyCode>USD</CurrencyCode>
              </Shipping>
              <ShipsFrom>
                  <Country>US</Country>
                  <State></State>
              </ShipsFrom>
              <IsFulfilledByAmazon>false</IsFulfilledByAmazon>
              <IsBuyBoxWinner>false</IsBuyBoxWinner>
              <PrimeInformation>
                  <IsPrime>false</IsPrime>
                  <IsNationalPrime>false</IsNationalPrime>
              </PrimeInformation>
              <IsFeaturedMerchant>true</IsFeaturedMerchant>
              <ShipsDomestically>true</ShipsDomestically>
          </Offer>
          <Offer>
              <SellerId>A00AS0PFN0IRUQ</SellerId>
              <SubCondition>new</SubCondition>
              <SellerFeedbackRating>
                  <SellerPositiveFeedbackRating>99</SellerPositiveFeedbackRating>
                  <FeedbackCount>10536</FeedbackCount>
              </SellerFeedbackRating>
              <ShippingTime minimumHours="0" maximumHours="0" availabilityType="NOW"/>
              <ListingPrice>
                  <Amount>129.99</Amount>
                  <CurrencyCode>USD</CurrencyCode>
              </ListingPrice>
              <Shipping>
                  <Amount>0.00</Amount>
                  <CurrencyCode>USD</CurrencyCode>
              </Shipping>
              <IsFulfilledByAmazon>true</IsFulfilledByAmazon>
              <IsBuyBoxWinner>true</IsBuyBoxWinner>
              <PrimeInformation>
                  <IsPrime>true</IsPrime>
                  <IsNationalPrime>true</IsNationalPrime>
              </PrimeInformation>
              <IsFeaturedMerchant>true</IsFeaturedMerchant>
              <ShipsDomestically>true</ShipsDomestically>
          </Offer>
      </Offers>
      "#,
      AnyOfferChangedNotification {
        OfferChangeTrigger: OfferChangeTrigger {
          MarketplaceId: "ATVPDKIKX0DER".to_string(),
          ASIN: "B0000C000V".to_string(),
          ItemCondition: "new".to_string(),
          TimeOfOfferChange: Some(Utc.ymd(2020, 2, 3).and_hms(21, 50, 9)),
          OfferChangeType: OfferChangeType::External,
        },
        Summary: Summary {
          NumberOfOffers: vec![
            OfferCount {
              Condition: "new".to_string(),
              FulfillmentChannel: "Merchant".to_string(),
              Value: 1,
            },
            OfferCount {
              Condition: "new".to_string(),
              FulfillmentChannel: "Amazon".to_string(),
              Value: 1,
            },
            OfferCount {
              Condition: "used".to_string(),
              FulfillmentChannel: "Amazon".to_string(),
              Value: 5,
            }
          ],
          LowestPrices: vec![
            LowestPrice {
              Condition: "new".to_string(),
              FulfillmentChannel: "Merchant".to_string(),
              LandedPrice: MoneyType {
                CurrencyCode: "USD".to_string(),
                Amount: "129.99".to_string(),
              },
              ListingPrice: MoneyType {
                CurrencyCode: "USD".to_string(),
                Amount: "129.99".to_string(),
              },
              Shipping: MoneyType {
                CurrencyCode: "USD".to_string(),
                Amount: "0.00".to_string(),
              },
              Points: None,
            },
            LowestPrice {
              Condition: "new".to_string(),
              FulfillmentChannel: "Amazon".to_string(),
              LandedPrice: MoneyType {
                CurrencyCode: "USD".to_string(),
                Amount: "129.99".to_string(),
              },
              ListingPrice: MoneyType {
                CurrencyCode: "USD".to_string(),
                Amount: "129.99".to_string(),
              },
              Shipping: MoneyType {
                CurrencyCode: "USD".to_string(),
                Amount: "0.00".to_string(),
              },
              Points: None,
            },
            LowestPrice {
              Condition: "used".to_string(),
              FulfillmentChannel: "Amazon".to_string(),
              LandedPrice: MoneyType {
                CurrencyCode: "USD".to_string(),
                Amount: "119.59".to_string(),
              },
              ListingPrice: MoneyType {
                CurrencyCode: "USD".to_string(),
                Amount: "119.59".to_string(),
              },
              Shipping: MoneyType {
                CurrencyCode: "USD".to_string(),
                Amount: "0.00".to_string(),
              },
              Points: None,
            },
          ],
          BuyBoxPrices: vec![
            BuyBoxPrice {
              Condition: "new".to_string(),
              LandedPrice: MoneyType {
                CurrencyCode: "USD".to_string(),
                Amount: "129.99".to_string(),
              },
              ListingPrice: MoneyType {
                CurrencyCode: "USD".to_string(),
                Amount: "129.99".to_string(),
              },
              Shipping: MoneyType {
                CurrencyCode: "USD".to_string(),
                Amount: "0.00".to_string(),
              },
            },
            BuyBoxPrice {
              Condition: "Used".to_string(),
              LandedPrice: MoneyType {
                CurrencyCode: "USD".to_string(),
                Amount: "125.58".to_string(),
              },
              ListingPrice: MoneyType {
                CurrencyCode: "USD".to_string(),
                Amount: "119.59".to_string(),
              },
              Shipping: MoneyType {
                CurrencyCode: "USD".to_string(),
                Amount: "5.99".to_string(),
              },
            }
          ],
          SalesRankings: vec![
            SalesRank {
              ProductCategoryId: "ce_display_on_website".to_string(),
              Rank: 1346
            },
            SalesRank {
              ProductCategoryId: "3236451011".to_string(),
              Rank: 6
            }
          ],
          BuyBoxEligibleOffers: vec![
            OfferCount {
              Condition: "new".to_string(),
              FulfillmentChannel: "Merchant".to_string(),
              Value: 1,
            },
            OfferCount {
              Condition: "new".to_string(),
              FulfillmentChannel: "Amazon".to_string(),
              Value: 1,
            },
            OfferCount {
              Condition: "used".to_string(),
              FulfillmentChannel: "Amazon".to_string(),
              Value: 5,
            },
          ],
          ListPrice: None,
          SuggestedLowerPricePlusShipping: None,
          CompetitivePriceThreshold: Some(MoneyType {
            CurrencyCode: "USD".to_string(),
            Amount: "129.99".to_string(),
          }),
        },
        Offers: vec![
          Offer {
            SellerId: "A00AS0PFN0IRUQ".to_string(),
            SubCondition: "new".to_string(),
            SellerFeedbackRating: SellerFeedbackRating {
              SellerPositiveFeedbackRating: Some("99".to_string()),
              FeedbackCount: 10536
            },
            ShippingTime: ShippingTime {
              MinimumHours: Some(24),
              MaximumHours: Some(24),
              AvailableDate: None,
              AvailabilityType: Some(AvailabilityType::NOW),
            },
            ListingPrice: MoneyType {
              CurrencyCode: "USD".to_string(),
              Amount: "129.99".to_string(),
            },
            Points: None,
            Shipping: MoneyType {
              CurrencyCode: "USD".to_string(),
              Amount: "0.00".to_string(),
            },
            ShipsFrom: Some(ShipsFrom {
              Country: "US".to_string(),
              State: "".to_string(),
            }),
            IsFulfilledByAmazon: false,
            IsBuyBoxWinner: Some(false),
            ConditionNotes: None,
            PrimeInformation: Some(PrimeInformation {
              IsPrime: false,
              IsNationalPrime: false,
            }),
            IsExpeditedShippingAvailable: None,
            IsFeaturedMerchant: Some(true),
            ShipsDomestically: Some(true),
          },
          Offer {
            SellerId: "A00AS0PFN0IRUQ".to_string(),
            SubCondition: "new".to_string(),
            SellerFeedbackRating: SellerFeedbackRating {
              SellerPositiveFeedbackRating: Some("99".to_string()),
              FeedbackCount: 10536
            },
            ShippingTime: ShippingTime {
              MinimumHours: Some(0),
              MaximumHours: Some(0),
              AvailableDate: None,
              AvailabilityType: Some(AvailabilityType::NOW),
            },
            ListingPrice: MoneyType {
              CurrencyCode: "USD".to_string(),
              Amount: "129.99".to_string(),
            },
            Points: None,
            Shipping: MoneyType {
              CurrencyCode: "USD".to_string(),
              Amount: "0.00".to_string(),
            },
            ShipsFrom: None,
            IsFulfilledByAmazon: true,
            IsBuyBoxWinner: Some(true),
            ConditionNotes: None,
            PrimeInformation: Some(PrimeInformation {
              IsPrime: true,
              IsNationalPrime: true,
            }),
            IsExpeditedShippingAvailable: None,
            IsFeaturedMerchant: Some(true),
            ShipsDomestically: Some(true),
          }
        ],
      }
    );
  }
}
