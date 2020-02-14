use crate::result::MwsResult;
use crate::xmlhelper::decode::{FromXmlStream, XmlEventStream};
use chrono::{DateTime, Utc};

pub mod any_offer_changed;
use self::any_offer_changed::AnyOfferChangedNotification;

#[allow(non_snake_case)]
#[derive(Debug, Default, PartialEq, Serialize, FromXmlStream)]
pub struct NotificationMetaData {
  pub NotificationType: String,
  pub PayloadVersion: String,
  pub UniqueId: String,
  pub PublishTime: Option<DateTime<Utc>>,
  pub SellerId: String,
  pub MarketplaceId: String,
}

#[derive(Debug, Serialize, PartialEq)]
pub enum NotificationPayload {
  AnyOfferChanged(AnyOfferChangedNotification),
  Unknown,
}

impl Default for NotificationPayload {
  fn default() -> Self {
    NotificationPayload::Unknown
  }
}

impl<S> FromXmlStream<S> for NotificationPayload
where
  S: XmlEventStream,
{
  fn from_xml(s: &mut S) -> MwsResult<Self> {
    use crate::xmlhelper::decode::{element, AnyElementName};
    element(s, AnyElementName, |s| match s.local_name() {
      "AnyOfferChangedNotification" => Ok(NotificationPayload::AnyOfferChanged(
        AnyOfferChangedNotification::from_xml(s)?,
      )),
      _ => Ok(NotificationPayload::Unknown),
    })
  }
}

#[allow(non_snake_case)]
#[derive(Debug, Default, PartialEq, Serialize, FromXmlStream)]
pub struct Notification {
  pub NotificationPayload: NotificationPayload,
  pub NotificationMetaData: NotificationMetaData,
}

#[cfg(test)]
mod tests {
  use super::any_offer_changed::*;
  use super::*;
  use crate::xmlhelper::decode::parse_xml_string;
  use chrono::{TimeZone, Utc};

  #[test]
  fn test_notification() {
    let v: Notification = parse_xml_string(
      r#"
        <Notification>
          <NotificationMetaData>
              <NotificationType>Test</NotificationType>
              <PayloadVersion>1.0</PayloadVersion>
              <UniqueId>db05ce25-a2d6-49f7-bb39-5419b1a17a26</UniqueId>
              <PublishTime>2020-02-03T21:50:30.000Z</PublishTime>
              <SellerId>A23AS8PFN4IRUQ</SellerId>
              <MarketplaceId>ATVPDKIKX0DER</MarketplaceId>
          </NotificationMetaData>
          <NotificationPayload>
              <AnyOfferChangedNotification>

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


              </AnyOfferChangedNotification>
          </NotificationPayload>
        </Notification>
      "#,
      "Notification",
    )
    .unwrap();
    let expected = Notification {
      NotificationMetaData: NotificationMetaData {
        NotificationType: "Test".to_string(),
        PayloadVersion: "1.0".to_string(),
        UniqueId: "db05ce25-a2d6-49f7-bb39-5419b1a17a26".to_string(),
        PublishTime: Some(Utc.ymd(2020, 2, 3).and_hms(21, 50, 30)),
        SellerId: "A23AS8PFN4IRUQ".to_string(),
        MarketplaceId: "ATVPDKIKX0DER".to_string(),
      },
      NotificationPayload: NotificationPayload::AnyOfferChanged(AnyOfferChangedNotification {
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
            },
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
            },
          ],
          SalesRankings: vec![
            SalesRank {
              ProductCategoryId: "ce_display_on_website".to_string(),
              Rank: 1346,
            },
            SalesRank {
              ProductCategoryId: "3236451011".to_string(),
              Rank: 6,
            },
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
              FeedbackCount: 10536,
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
              FeedbackCount: 10536,
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
          },
        ],
      }),
    };

    assert_eq!(v, expected)
  }
}
