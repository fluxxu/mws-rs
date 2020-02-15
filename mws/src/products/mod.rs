//! Amazon MWS Products API - Version 2011-10-01
//!
//! [Reference](http://docs.developer.amazonservices.com/en_US/products/Products_Overview.html)

use client::{Client, Method};
use result::MwsResult;

pub mod types;
pub use self::types::*;

static PATH: &'static str = "/Products/2011-10-01";
static VERSION: &'static str = "2011-10-01";

#[derive(FromXmlStream, Default, Debug)]
#[allow(non_snake_case)]
pub struct GetLowestPricedOffersForSKUResponse {
  pub Identifier: Identifier,
  pub Summary: Summary,
  pub Offers: Vec<Offer>,
}

response_envelope_type!(
  GetLowestPricedOffersForSKUResponseEnvelope<GetLowestPricedOffersForSKUResponse>,
  "GetLowestPricedOffersForSKUResponse",
  "GetLowestPricedOffersForSKUResult"
);

#[allow(non_snake_case)]
#[derive(Debug, Default, Serialize, SerializeMwsParams)]
pub struct GetLowestPricedOffersForSKUParameters {
  pub MarketplaceId: String,
  pub SellerSKU: String,
  pub ItemCondition: ItemCondition,
}

#[allow(non_snake_case)]
pub fn GetLowestPricedOffersForSKU(
  client: &Client,
  params: GetLowestPricedOffersForSKUParameters,
) -> MwsResult<GetLowestPricedOffersForSKUResponse> {
  client
    .request_xml_with_form(
      Method::Post,
      PATH,
      VERSION,
      "GetLowestPricedOffersForSKU",
      params,
    )
    .map(|e: GetLowestPricedOffersForSKUResponseEnvelope| e.into_inner())
    .map_err(|err| err.into())
}

response_envelope_batch_type!(
  GetMyPriceForASINResponseEnvelope<GetMyPriceForASINResult>,
  "GetMyPriceForASINResponse",
  "GetMyPriceForASINResult"
);

#[allow(non_snake_case)]
#[derive(Debug, Default, Serialize, SerializeMwsParams)]
pub struct GetMyPriceForASINParameters {
  pub MarketplaceId: String,
  #[mws_param(list_item_type_name = "ASIN")]
  pub ASINList: Vec<String>,
}

#[derive(FromXmlStream, Default, Debug, PartialEq)]
#[allow(non_snake_case)]
pub struct GetMyPriceForASINResult {
  #[from_xml_stream(from_attr = "ASIN")]
  pub ASIN: String,
  #[from_xml_stream(from_attr = "status")]
  pub Status: String,
  pub Product: product::Product,
}

#[allow(non_snake_case)]
pub fn GetMyPriceForASIN(
  client: &Client,
  params: GetMyPriceForASINParameters,
) -> MwsResult<Vec<GetMyPriceForASINResult>> {
  client
    .request_xml_with_form(Method::Post, PATH, VERSION, "GetMyPriceForASIN", params)
    .map(|e: GetMyPriceForASINResponseEnvelope| e.into_inner())
    .map_err(|err| err.into())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_get_my_price_for_sku_response() {
    test_decode_envelope!(
      GetMyPriceForASINResponseEnvelope,
      r#"
        <GetMyPriceForASINResponse xmlns="http://mws.amazonservices.com/schema/Products/2011-10-01">
          <GetMyPriceForASINResult ASIN="B073000000" status="Success">
            <Product xmlns:ns2="http://mws.amazonservices.com/schema/Products/2011-10-01/default.xsd">
              <Identifiers>
                <MarketplaceASIN>
                  <MarketplaceId>ATVPDKIKX0DER</MarketplaceId>
                  <ASIN>B073000000</ASIN>
                </MarketplaceASIN>
              </Identifiers>
              <Offers>
                <Offer>
                  <BuyingPrice>
                    <LandedPrice>
                      <CurrencyCode>USD</CurrencyCode>
                      <Amount>29.99</Amount>
                    </LandedPrice>
                    <ListingPrice>
                      <CurrencyCode>USD</CurrencyCode>
                      <Amount>29.99</Amount>
                    </ListingPrice>
                    <Shipping>
                      <CurrencyCode>USD</CurrencyCode>
                      <Amount>0.00</Amount>
                    </Shipping>
                  </BuyingPrice>
                  <RegularPrice>
                    <CurrencyCode>USD</CurrencyCode>
                    <Amount>29.99</Amount>
                  </RegularPrice>
                  <FulfillmentChannel>AMAZON</FulfillmentChannel>
                  <ItemCondition>New</ItemCondition>
                  <ItemSubCondition>New</ItemSubCondition>
                  <SellerId>A23AS800000000</SellerId>
                  <SellerSKU>sku</SellerSKU>
                </Offer>
                <Offer>
                  <BuyingPrice>
                    <LandedPrice>
                      <CurrencyCode>USD</CurrencyCode>
                      <Amount>29.99</Amount>
                    </LandedPrice>
                    <ListingPrice>
                      <CurrencyCode>USD</CurrencyCode>
                      <Amount>29.99</Amount>
                    </ListingPrice>
                    <Shipping>
                      <CurrencyCode>USD</CurrencyCode>
                      <Amount>0.00</Amount>
                    </Shipping>
                  </BuyingPrice>
                  <RegularPrice>
                    <CurrencyCode>USD</CurrencyCode>
                    <Amount>29.99</Amount>
                  </RegularPrice>
                  <FulfillmentChannel>MERCHANT</FulfillmentChannel>
                  <ItemCondition>New</ItemCondition>
                  <ItemSubCondition>New</ItemSubCondition>
                  <SellerId>A23AS800000000</SellerId>
                  <SellerSKU>sku-fbm</SellerSKU>
                </Offer>
              </Offers>
            </Product>
          </GetMyPriceForASINResult>
          <GetMyPriceForASINResult ASIN="B073000001" status="Success">
            <Product xmlns:ns2="http://mws.amazonservices.com/schema/Products/2011-10-01/default.xsd">
              <Identifiers>
                <MarketplaceASIN>
                  <MarketplaceId>ATVPDKIKX0DER</MarketplaceId>
                  <ASIN>B073000001</ASIN>
                </MarketplaceASIN>
              </Identifiers>
              <Offers/>
            </Product>
          </GetMyPriceForASINResult>
          <ResponseMetadata>
            <RequestId>3e353f76-2ef6-442e-a714-6bbc26f96626</RequestId>
          </ResponseMetadata>
        </GetMyPriceForASINResponse>
      "#,
      vec![
        GetMyPriceForASINResult {
          ASIN: "B073000000".to_string(),
          Status: "Success".to_string(),
          Product: product::Product {
            Identifiers: product::Identifier {
              MarketplaceASIN: Some(product::MarketplaceASIN {
                MarketplaceId: "ATVPDKIKX0DER".to_string(),
                ASIN: "B073000000".to_string(),
              }),
              ..Default::default()
            },
            Offers: vec![
              product::Offer {
                BuyingPrice: product::Price {
                  LandedPrice: MoneyType {
                    CurrencyCode: "USD".to_string(),
                    Amount: "29.99".to_string(),
                  },
                  ListingPrice: MoneyType {
                    CurrencyCode: "USD".to_string(),
                    Amount: "29.99".to_string(),
                  },
                  Shipping: MoneyType {
                    CurrencyCode: "USD".to_string(),
                    Amount: "0.00".to_string(),
                  },
                },
                RegularPrice: MoneyType {
                  CurrencyCode: "USD".to_string(),
                  Amount: "29.99".to_string(),
                },
                FulfillmentChannel: "AMAZON".to_string(),
                ItemCondition: ItemCondition::New,
                ItemSubCondition: "New".to_string(),
                SellerId: "A23AS800000000".to_string(),
                SellerSKU: "sku".to_string(),
              },
              product::Offer {
                BuyingPrice: product::Price {
                  LandedPrice: MoneyType {
                    CurrencyCode: "USD".to_string(),
                    Amount: "29.99".to_string(),
                  },
                  ListingPrice: MoneyType {
                    CurrencyCode: "USD".to_string(),
                    Amount: "29.99".to_string(),
                  },
                  Shipping: MoneyType {
                    CurrencyCode: "USD".to_string(),
                    Amount: "0.00".to_string(),
                  },
                },
                RegularPrice: MoneyType {
                  CurrencyCode: "USD".to_string(),
                  Amount: "29.99".to_string(),
                },
                FulfillmentChannel: "MERCHANT".to_string(),
                ItemCondition: ItemCondition::New,
                ItemSubCondition: "New".to_string(),
                SellerId: "A23AS800000000".to_string(),
                SellerSKU: "sku-fbm".to_string(),
              }
            ],
          }
        },
        GetMyPriceForASINResult {
          ASIN: "B073000001".to_string(),
          Status: "Success".to_string(),
          Product: product::Product {
            Identifiers: product::Identifier {
              MarketplaceASIN: Some(product::MarketplaceASIN {
                MarketplaceId: "ATVPDKIKX0DER".to_string(),
                ASIN: "B073000001".to_string(),
              }),
              ..Default::default()
            },
            Offers: vec![]
          }
        }
      ]
    );
  }
}
