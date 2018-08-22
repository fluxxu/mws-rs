use chrono::{DateTime, Utc};
use xmlhelper::decode;

/// The condition of the item.
str_enum! {
  pub enum Condition {
    NewItem,
    NewWithWarranty,
    NewOEM,
    NewOpenBox,
    UsedLikeNew,
    UsedVeryGood,
    UsedGood,
    UsedAcceptable,
    UsedPoor,
    UsedRefurbished,
    CollectibleLikeNew,
    CollectibleVeryGood,
    CollectibleGood,
    CollectibleAcceptable,
    CollectiblePoor,
    RefurbishedWithWarranty,
    Refurbished,
    Club,
  }
}

#[allow(non_snake_case)]
#[derive(Debug, Default, PartialEq, Serialize)]
pub struct InventorySupply {
  /// The Seller SKU of the item.
  pub SellerSKU: String,
  /// The Fulfillment Network SKU (FNSKU) of the item.
  /// The FNSKU is a unique identifier for each inventory item stored in an Amazon fulfillment center.
  pub FNSKU: String,
  /// The Amazon Standard Identification Number (ASIN) of the item.
  pub ASIN: String,
  /// The condition of the item.
  pub Condition: Option<Condition>,
  /// The total item quantity in the Amazon Fulfillment Network supply chain.
  /// This includes item quantity currently in an Amazon fulfillment center,
  /// item quantity currently in an inbound shipment, and item quantity being
  /// transferred between Amazon fulfillment centers in the Amazon Fulfillment Network.
  pub TotalSupplyQuantity: i32,
  /// The item quantity available for fulfillment. This does not
  /// include item quantity currently in an inbound shipment or item
  /// quantity being transferred between Amazon fulfillment centers
  /// in the Amazon Fulfillment Network.
  pub InStockSupplyQuantity: i32,
  /// The earliest date that your inventory is expected to be available for picking.
  pub EarliestAvailability: Timepoint,
  /// Detailed information about the availability of inventory for a specific
  /// item and its current location in the Amazon Fulfillment Network supply chain.
  pub SupplyDetail: Option<Vec<InventorySupplyDetail>>,
}

/// Indicates whether inventory is immediately available for picking,
/// whether inventory availability is unknown, or whether inventory
/// is expected to be available for picking by a specific date.
str_enum! {
  pub enum TimepointType {
    Immediately,
    DateTime,
    Unknown,
  }
}

#[allow(non_snake_case)]
#[derive(Debug, Default, PartialEq, Serialize)]
pub struct Timepoint {
  pub TimepointType: TimepointType,
  /// The date and time by which inventory is expected
  /// to be available for picking, in ISO 8601 date time format.
  pub DateTime: Option<DateTime<Utc>>,
}

impl<S: decode::XmlEventStream> decode::FromXmlStream<S> for Timepoint {
  fn from_xml(s: &mut S) -> decode::Result<Timepoint> {
    use xmlhelper::decode::{characters, fold_elements};
    fold_elements(s, Timepoint::default(), |s, record| {
      match s.local_name() {
        "TimepointType" => record.TimepointType = characters(s)?,
        "DateTime" => record.DateTime = characters(s).map(Some)?,
        _ => {}
      }
      Ok(())
    })
  }
}

/// The current inventory status for a specific item.
str_enum! {
  pub enum SupplyType {
    InStock,
    Inbound,
    Transfer,
  }
}

#[allow(non_snake_case)]
#[derive(Debug, Default, PartialEq, Serialize)]
/// Specific information about the availability of inventory for a single SKU,
/// including the number of units that are in an Amazon fulfillment center, in an inbound shipment,
/// or being transferred between Amazon fulfillment centers.
pub struct InventorySupplyDetail {
  /// The quantity of inventory for a specific item.
  pub Quantity: i32,
  ///	The current inventory status for a specific item.
  pub SupplyType: SupplyType,
  /// The earliest date that your inventory is expected to be available for picking.
  pub EarliestAvailableToPick: Timepoint,
  /// The latest date that your inventory is expected to be available for picking.
  pub LatestAvailableToPick: Timepoint,
}

impl<S: decode::XmlEventStream> decode::FromXmlStream<S> for InventorySupply {
  fn from_xml(s: &mut S) -> decode::Result<InventorySupply> {
    use xmlhelper::decode::{characters, fold_elements};
    fold_elements(s, InventorySupply::default(), |s, record| {
      match s.local_name() {
        "Condition" => record.Condition = characters(s).map(Some)?,
        "SupplyDetail" => {
          if s.has_next()? {
            record.SupplyDetail = fold_elements(s, vec![], |s, list| {
              let item = fold_elements(s, InventorySupplyDetail::default(), |s, detail| {
                match s.local_name() {
                  "Quantity" => detail.Quantity = characters(s)?,
                  "SupplyType" => detail.SupplyType = characters(s)?,
                  "EarliestAvailableToPick" => {
                    detail.EarliestAvailableToPick = Timepoint::from_xml(s)?
                  }
                  "LatestAvailableToPick" => detail.LatestAvailableToPick = Timepoint::from_xml(s)?,
                  _ => {}
                };
                Ok(())
              })?;
              list.push(item);
              Ok(())
            }).map(Some)?;
          } else {
            // if an item has an empty <SupplyDetail />
            // it's still a valid item
            record.SupplyDetail = Some(vec![]);
          }
        }
        "TotalSupplyQuantity" => record.TotalSupplyQuantity = characters(s)?,
        "EarliestAvailability" => record.EarliestAvailability = Timepoint::from_xml(s)?,
        "FNSKU" => record.FNSKU = characters(s)?,
        "InStockSupplyQuantity" => record.InStockSupplyQuantity = characters(s)?,
        "ASIN" => record.ASIN = characters(s)?,
        "SellerSKU" => record.SellerSKU = characters(s)?,
        _ => {}
      }
      Ok(())
    })
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::io::Cursor;
  use xmlhelper::decode;
  use xmlhelper::decode::FromXmlStream;

  #[test]
  fn test_decode_inventory_supply() {
    let mut s = decode::Stream::new(Cursor::new(
      r#"
        <Condition>NewItem</Condition>
        <SupplyDetail/>
        <TotalSupplyQuantity>127</TotalSupplyQuantity>
        <EarliestAvailability>
          <TimepointType>Immediately</TimepointType>
        </EarliestAvailability>
        <FNSKU>B013JG71CG</FNSKU>
        <InStockSupplyQuantity>127</InStockSupplyQuantity>
        <ASIN>B013JG71CG</ASIN>
        <SellerSKU>edifier-k815-white</SellerSKU>
      "#,
    ));

    decode::start_document(&mut s).expect("start element");
    let record = InventorySupply::from_xml(&mut s).expect("decode inventory supply");
    assert_eq!(
      record,
      InventorySupply {
        SellerSKU: "edifier-k815-white".to_owned(),
        FNSKU: "B013JG71CG".to_owned(),
        ASIN: "B013JG71CG".to_owned(),
        Condition: Some(Condition::NewItem),
        TotalSupplyQuantity: 127,
        InStockSupplyQuantity: 127,
        EarliestAvailability: Timepoint {
          TimepointType: TimepointType::Immediately,
          DateTime: None,
        },
        SupplyDetail: None,
      }
    );
  }

  #[test]
  fn test_decode_inventory_supply_detail() {
    let mut s = decode::Stream::new(Cursor::new(
      r#"
        <Condition>NewItem</Condition>
        <SupplyDetail>
          <member>
            <LatestAvailableToPick>
              <TimepointType>Immediately</TimepointType>
            </LatestAvailableToPick>
            <EarliestAvailableToPick>
              <TimepointType>Immediately</TimepointType>
            </EarliestAvailableToPick>
            <Quantity>2</Quantity>
            <SupplyType>InStock</SupplyType>
          </member>
          <member>
            <LatestAvailableToPick>
              <TimepointType>Immediately</TimepointType>
            </LatestAvailableToPick>
            <EarliestAvailableToPick>
              <TimepointType>Immediately</TimepointType>
            </EarliestAvailableToPick>
            <Quantity>15</Quantity>
            <SupplyType>InStock</SupplyType>
          </member>
          <member>
            <LatestAvailableToPick>
              <TimepointType>Immediately</TimepointType>
            </LatestAvailableToPick>
            <EarliestAvailableToPick>
              <TimepointType>Immediately</TimepointType>
            </EarliestAvailableToPick>
            <Quantity>58</Quantity>
            <SupplyType>InStock</SupplyType>
          </member>          
        </SupplyDetail>
        <TotalSupplyQuantity>127</TotalSupplyQuantity>
        <EarliestAvailability>
          <TimepointType>Immediately</TimepointType>
        </EarliestAvailability>
        <FNSKU>B013JG71CG</FNSKU>
        <InStockSupplyQuantity>127</InStockSupplyQuantity>
        <ASIN>B013JG71CG</ASIN>
        <SellerSKU>edifier-k815-white</SellerSKU>
      "#,
    ));

    decode::start_document(&mut s).expect("start element");
    let record = InventorySupply::from_xml(&mut s).expect("decode inventory supply");
    assert_eq!(
      record,
      InventorySupply {
        SellerSKU: "edifier-k815-white".to_owned(),
        FNSKU: "B013JG71CG".to_owned(),
        ASIN: "B013JG71CG".to_owned(),
        Condition: Some(Condition::NewItem),
        TotalSupplyQuantity: 127,
        InStockSupplyQuantity: 127,
        EarliestAvailability: Timepoint {
          TimepointType: TimepointType::Immediately,
          DateTime: None,
        },
        SupplyDetail: Some(vec![
          InventorySupplyDetail {
            LatestAvailableToPick: Timepoint {
              TimepointType: TimepointType::Immediately,
              DateTime: None,
            },
            EarliestAvailableToPick: Timepoint {
              TimepointType: TimepointType::Immediately,
              DateTime: None,
            },
            Quantity: 2,
            SupplyType: SupplyType::InStock,
          },
          InventorySupplyDetail {
            LatestAvailableToPick: Timepoint {
              TimepointType: TimepointType::Immediately,
              DateTime: None,
            },
            EarliestAvailableToPick: Timepoint {
              TimepointType: TimepointType::Immediately,
              DateTime: None,
            },
            Quantity: 15,
            SupplyType: SupplyType::InStock,
          },
          InventorySupplyDetail {
            LatestAvailableToPick: Timepoint {
              TimepointType: TimepointType::Immediately,
              DateTime: None,
            },
            EarliestAvailableToPick: Timepoint {
              TimepointType: TimepointType::Immediately,
              DateTime: None,
            },
            Quantity: 58,
            SupplyType: SupplyType::InStock,
          },
        ]),
      }
    );
  }
}
