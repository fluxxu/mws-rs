use std::io::Read;
use chrono::{DateTime, UTC};
use tdff;

#[derive(Debug, Default, PartialEq)]
pub struct ReportInfo {
  pub report_type: String,
  pub acknowledged: bool,
  pub acknowledged_date: Option<DateTime<UTC>>,
  pub report_id: String,
  pub available_date: Option<DateTime<UTC>>,
  pub report_request_id: String,
}

str_enum! {
  pub enum ReportProcessingStatus {
    _SUBMITTED_,
    _IN_PROGRESS_,
    _CANCELLED_,
    _DONE_,
    _DONE_NO_DATA_,
  }
}

#[derive(Debug, Default, PartialEq)]
pub struct ReportRequestInfo {
  pub report_request_id: String,
  pub report_type: String,
  pub start_date: Option<DateTime<UTC>>,
  pub end_date: Option<DateTime<UTC>>,
  pub scheduled: bool,
  pub submitted_date: Option<DateTime<UTC>>,
  pub report_processing_status: String,
  pub generated_report_id: Option<String>,
  pub started_processing_date: Option<DateTime<UTC>>,
  pub completed_date: Option<DateTime<UTC>>,
}

/// FlatFileSettlementReport
#[allow(non_snake_case)]
#[derive(Debug, Default, PartialEq)]
pub struct SettlementReport {
  pub SettlementId: String,
  pub SettlementStartDate: Option<DateTime<UTC>>,
  pub SettlementEndDate: Option<DateTime<UTC>>,
  pub DepositDate: Option<DateTime<UTC>>,
  pub TotalAmount: f64,
  pub Currency: String,
  pub Items: Vec<SettlementReportItem>,
}

macro_rules! parse_field {
    ($var:ident, $field:ident, $value:ident) => ({
      let trimmed = $value.trim();
      if !trimmed.is_empty() {
        $var.$field = trimmed.parse()
          .map_err(|err| tdff::ErrorKind::ParseString(stringify!($field).to_string(), format!("{}: '{}'", err, $value)))?;
      }
    })
}

macro_rules! parse_option_field {
    ($var:ident, $field:ident, $value:ident) => ({
      let trimmed = $value.trim();
      if !trimmed.is_empty() {      
        $var.$field = trimmed.parse()
          .map(Some)
          .map_err(|err| tdff::ErrorKind::ParseString(stringify!($field).to_string(), format!("{}: '{}'", err, $value)))?;
      }
    })
}

impl<R: Read> tdff::FromTdff<R> for SettlementReport {
  fn from_tdff(source: R) -> tdff::Result<SettlementReport> {
    let mut scanner = tdff::TdffScanner::new(source)?;
    let mut report = SettlementReport::default();
    scanner.for_each_row(|i, row| {
      if i == 0 {
        for (key, value) in row {
          match key {
            "settlement-id" => report.SettlementId = value,
            "settlement-start-date" => parse_option_field!(report, SettlementStartDate, value),
            "settlement-end-date" => parse_option_field!(report, SettlementEndDate, value),
            "deposit-date" => parse_option_field!(report, DepositDate, value),
            "total-amount" => parse_field!(report, TotalAmount, value),
            "currency" => parse_field!(report, Currency, value),
            _ => {},
          }
        }
      } else {
        let mut item = SettlementReportItem::default();
        for (key, value) in row {
          match key {
            "transaction-type" => item.TransactionType = value,
            "order-id" => item.OrderId = value,
            "merchant-order-id" => item.MerchantOrderId = value,
            "adjustment-id" => item.AdjustmentId = value,
            "shipment-id" => item.ShipmentId = value,
            "marketplace-name" => item.MarketplaceName = value,
            "shipment-fee-type" => item.ShipmentFeeType = value,
            "shipment-fee-amount" => parse_field!(item, ShipmentFeeAmount, value),
            "order-fee-type" => item.OrderFeeType = value,
            "order-fee-amount" => parse_field!(item, OrderFeeAmount, value),
            "fulfillment-id" => item.FulfillmentId = value,
            "posted-date" => parse_option_field!(item, PostedDate, value),
            "order-item-code" => item.OrderItemCode = value,
            "merchant-order-item-id" => item.MerchantOrderItemId = value,
            "merchant-adjustment-item-id" => item.MerchantAdjustmentItemId = value,
            "sku" => item.Sku = value,
            "quantity-purchased" => parse_field!(item, QuantityPurchased, value),
            "price-type" => item.PriceType = value,
            "price-amount" => parse_field!(item, PriceAmount, value),
            "item-related-fee-type" => item.ItemRelatedFeeType = value,
            "item-related-fee-amount" => parse_field!(item, ItemRelatedFeeAmount, value),
            "misc-fee-amount" => parse_field!(item, MiscFeeAmount, value),
            "other-fee-amount" => parse_field!(item, OtherFeeAmount, value),
            "other-fee-reason-description" => item.OtherFeeReasonDescription = value,
            "promotion-id" => item.PromotionId = value,
            "promotion-type" => item.PromotionType = value,
            "promotion-amount" => parse_field!(item, PromotionAmount, value),
            "direct-payment-type" => item.DirectPaymentType = value,
            "direct-payment-amount" => parse_field!(item, DirectPaymentAmount, value),
            "other-amount" => parse_field!(item, OtherAmount, value),
            _ => {},
          }
        }
        report.Items.push(item);
      }
      Ok(())
    })?;
    Ok(report)
  }
}

#[allow(non_snake_case)]
#[derive(Debug, Default, PartialEq)]
pub struct SettlementReportItem {
  pub TransactionType: String,
  pub OrderId: String,
  pub MerchantOrderId: String,
  pub AdjustmentId: String,
  pub ShipmentId: String,
  pub MarketplaceName: String,
  pub ShipmentFeeType: String,
  pub ShipmentFeeAmount: f64,
  pub OrderFeeType: String,
  pub OrderFeeAmount: f64,
  pub FulfillmentId: String,
  pub PostedDate: Option<DateTime<UTC>>,
  pub OrderItemCode: String,
  pub MerchantOrderItemId: String,
  pub MerchantAdjustmentItemId: String,	
  pub Sku: String,	
  pub QuantityPurchased: i32,	
  pub PriceType: String,
  pub PriceAmount: f64,	
  pub ItemRelatedFeeType: String,	
  pub ItemRelatedFeeAmount: f64,	
  pub MiscFeeAmount: f64,	
  pub OtherFeeAmount: f64,
  pub OtherFeeReasonDescription: String,	
  pub PromotionId: String,	
  pub PromotionType: String,	
  pub PromotionAmount: f64,	
  pub DirectPaymentType: String,
  pub DirectPaymentAmount: f64,	
  pub OtherAmount: f64,
}
