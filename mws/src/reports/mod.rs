//! Amazon MWS Reports API - Version 2009-01-01
//!
//! [Documentation](http://docs.developer.amazonservices.com/en_US/reports/Reports_Overview.html)

use chrono::{DateTime, Utc};
use client::{Client, Method};
mod types;
pub use self::types::{ReportInfo, ReportProcessingStatus, ReportRequestInfo};
use result::{MwsError, MwsResult};
use std::io::{self, Write};

static PATH: &'static str = "/";
static VERSION: &'static str = "2009-01-01";

/// Parameters for `GetReportList`
#[allow(non_snake_case)]
#[derive(Debug, Default, Serialize, SerializeMwsParams)]
pub struct GetReportListParameters {
  pub MaxCount: Option<i32>,
  #[mws_param(list_item_type_name = "Type")]
  pub ReportTypeList: Option<Vec<String>>,
  pub Acknowledged: Option<bool>,
  pub AvailableFromDate: Option<DateTime<Utc>>,
  pub AvailableToDate: Option<DateTime<Utc>>,
  #[mws_param(list_item_type_name = "Id")]
  pub ReportRequestIdList: Option<Vec<String>>,
}

#[derive(Debug, Default, Serialize, FromXmlStream)]
#[allow(non_snake_case)]
pub struct GetReportListResponse {
  #[from_xml_stream(no_list_wrapper)]
  pub ReportInfo: Vec<ReportInfo>,
  pub NextToken: Option<String>,
  pub HasNext: bool,
}

response_envelope_type!(
  GetReportListEnvelope<GetReportListResponse>,
  "GetReportListResponse",
  "GetReportListResult"
);

response_envelope_type!(
  GetReportListByNextTokenEnvelope<GetReportListResponse>,
  "GetReportListByNextTokenResponse",
  "GetReportListByNextTokenResult"
);

/// Returns a list of reports that were created in the previous 90 days.
#[allow(non_snake_case)]
pub fn GetReportList(
  client: &Client,
  params: GetReportListParameters,
) -> MwsResult<GetReportListResponse> {
  client
    .request_xml(Method::Post, PATH, VERSION, "GetReportList", params)
    .map(|e: GetReportListEnvelope| e.into_inner())
    .map_err(|err| err.into())
}

/// Returns a list of reports using the NextToken, which was supplied by a previous request to either
/// GetReportListByNextToken or GetReportList, where the value of HasNext was true in the previous call.
#[allow(non_snake_case)]
pub fn GetReportListByNextToken(
  client: &Client,
  next_token: String,
) -> MwsResult<GetReportListResponse> {
  let params = vec![("NextToken".to_string(), next_token)];
  client
    .request_xml(
      Method::Post,
      PATH,
      VERSION,
      "GetReportListByNextToken",
      params,
    )
    .map(|e: GetReportListByNextTokenEnvelope| e.into_inner())
    .map_err(|err| err.into())
}

/// Returns the contents of a report and the Content-MD5 header for the returned report body.
#[allow(non_snake_case)]
pub fn GetReport<W: Write>(
  client: &Client,
  report_id: String,
  out: &mut W,
) -> MwsResult<(u64, String)> {
  let params = vec![("ReportId".to_string(), report_id)];
  let mut resp = client.request(Method::Post, PATH, VERSION, "GetReport", params)?;
  let content_md5 = resp
    .headers()
    .get_raw("content-md5")
    .ok_or_else(|| MwsError::ContentMD5HeaderMissing)
    .and_then(|data| ::std::str::from_utf8(&data[0]).map_err(Into::into))?
    .to_owned();
  let size = io::copy(&mut resp, out)?;
  Ok((size, content_md5))
}

/// Parameters for `GetReportRequestList`
#[allow(non_snake_case)]
#[derive(Debug, Default, Serialize, SerializeMwsParams)]
pub struct GetReportRequestListParameters {
  pub MaxCount: Option<i32>,
  #[mws_param(list_item_type_name = "Type")]
  pub ReportTypeList: Option<Vec<String>>,
  pub RequestedFromDate: Option<DateTime<Utc>>,
  pub RequestedToDate: Option<DateTime<Utc>>,
  #[mws_param(list_item_type_name = "Id")]
  pub ReportRequestIdList: Option<Vec<String>>,
  #[mws_param(list_item_type_name = "Status")]
  pub ReportProcessingStatusList: Option<Vec<ReportProcessingStatus>>,
}

#[allow(non_snake_case)]
#[derive(Debug, Default, Serialize, FromXmlStream)]
pub struct GetReportRequestListResponse {
  #[from_xml_stream(no_list_wrapper)]
  pub ReportRequestInfo: Vec<ReportRequestInfo>,
  pub NextToken: Option<String>,
  pub HasNext: bool,
}

response_envelope_type!(
  GetReportRequestListEnvelope<GetReportRequestListResponse>,
  "GetReportRequestListResponse",
  "GetReportRequestListResult"
);

response_envelope_type!(
  GetReportRequestListByNextTokenEnvelope<GetReportRequestListResponse>,
  "GetReportRequestListByNextTokenResponse",
  "GetReportRequestListByNextTokenResult"
);

/// Returns a list of report requests that you can use to get the ReportRequestId for a report.
#[allow(non_snake_case)]
pub fn GetReportRequestList(
  client: &Client,
  params: GetReportRequestListParameters,
) -> MwsResult<GetReportRequestListResponse> {
  client
    .request_xml(Method::Post, PATH, VERSION, "GetReportRequestList", params)
    .map(|e: GetReportRequestListEnvelope| e.into_inner())
    .map_err(|err| err.into())
}

/// Returns a list of reports using the NextToken, which was supplied by a previous request to either
/// GetReportListByNextToken or GetReportList, where the value of HasNext was true in the previous call.
#[allow(non_snake_case)]
pub fn GetReportRequestListByNextToken(
  client: &Client,
  next_token: String,
) -> MwsResult<GetReportRequestListResponse> {
  let params = vec![("NextToken".to_string(), next_token)];
  client
    .request_xml(
      Method::Post,
      PATH,
      VERSION,
      "GetReportRequestListByNextToken",
      params,
    )
    .map(|e: GetReportRequestListByNextTokenEnvelope| e.into_inner())
    .map_err(|err| err.into())
}

/// Parameters for `RequestReport`
#[allow(non_snake_case)]
#[derive(Debug, Default, Serialize, SerializeMwsParams)]
pub struct RequestReportParameters {
  pub ReportType: String,
  pub StartDate: Option<DateTime<Utc>>,
  pub EndDate: Option<DateTime<Utc>>,
  pub ReportOptions: Option<String>,
  #[mws_param(list_item_type_name = "Id")]
  pub MarketplaceIdList: Option<Vec<String>>,
}

#[allow(non_snake_case)]
#[derive(Debug, Default, Serialize, FromXmlStream)]
pub struct RequestReportResponse {
  pub ReportRequestInfo: ReportRequestInfo,
}

response_envelope_type!(
  RequestReportEnvelope<RequestReportResponse>,
  "RequestReportResponse",
  "RequestReportResult"
);

#[allow(non_snake_case)]
pub fn RequestReport(
  client: &Client,
  params: RequestReportParameters,
) -> MwsResult<RequestReportResponse> {
  client
    .request_xml(Method::Post, PATH, VERSION, "RequestReport", params)
    .map(|e: RequestReportEnvelope| e.into_inner())
    .map_err(|err| err.into())
}

#[cfg(test)]
mod tests {
  use super::super::client::get_test_client;
  use super::*;
  use dotenv::dotenv;

  #[test]
  #[ignore]
  fn test_get_report_list() {
    dotenv().ok();
    let c = get_test_client();
    let mut params = GetReportListParameters::default();
    params.ReportTypeList = Some(vec!["_GET_V2_SETTLEMENT_REPORT_DATA_FLAT_FILE_".to_owned()]);
    let res = GetReportList(&c, params).expect("GetReportList");
    println!("{:?}", res);
  }

  // #[test]
  // #[ignore]
  // fn test_settlement_report() {
  //   dotenv().ok();
  //   let c = get_test_client();
  //   let res = GetFlatFileSettlementReport(&c, "3915548544017177".to_string())
  //     .expect("GetFlatFileSettlementReport");
  //   println!("{:?}", res);
  // }

  #[test]
  #[ignore]
  fn test_get_report_request_list() {
    dotenv().ok();
    let c = get_test_client();
    let mut params = GetReportRequestListParameters::default();
    params.ReportTypeList = Some(vec!["_GET_AFN_INVENTORY_DATA_".to_owned()]);
    let res = GetReportRequestList(&c, params).expect("GetReportRequestList");
    println!("{:?}", res);

    let next_token = res.NextToken.unwrap();
    let res =
      GetReportRequestListByNextToken(&c, next_token).expect("GetReportRequestListByNextToken");
    println!("{:?}", res);
  }
}
