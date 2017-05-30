//! Amazon MWS Reports API - Version 2009-01-01
//!
//! [Documentation](http://docs.developer.amazonservices.com/en_US/reports/Reports_Overview.html)

use chrono::{DateTime, UTC};
use client::{Client, Method, Response};
mod types;
pub use self::types::{ReportInfo, ReportType, SettlementReport};
use super::types::ToIso8601;
use xmlhelper::decode;
use std::io::{self, Write};

error_chain! {
  links {
    Client(super::client::Error, super::client::ErrorKind);
    XmlDecode(decode::Error, decode::ErrorKind);
    TdffDecode(super::tdff::Error, super::tdff::ErrorKind);
  }

  foreign_links {
    Io(io::Error);
  }
}

static PATH: &'static str = "/";
static VERSION: &'static str = "2009-01-01";

/// Parameters for `GetReportList`
#[derive(Debug, Default)]
pub struct GetReportListParameters {
  pub max_count: Option<i32>,
  pub report_type_list: Option<Vec<ReportType>>,
  pub acknowledged: Option<bool>,
  pub available_from_date: Option<DateTime<UTC>>,
  pub available_to_date: Option<DateTime<UTC>>,
  pub report_request_id_list: Option<Vec<String>>,
}

impl Into<Vec<(String, String)>> for GetReportListParameters {
  fn into(self) -> Vec<(String, String)> {
    let mut result = vec![];

    if let Some(v) = self.max_count {
      result.push(("MaxCount".to_string(), v.to_string()));
    }

    if let Some(list) = self.report_type_list {
      for (i, ty) in list.into_iter().enumerate() {
        result.push((format!("ReportTypeList.Type.{}", i + 1), ty.into()));
      }
    }

    if let Some(v) = self.acknowledged {
      result.push(("Acknowledged".to_string(), v.to_string()));
    }

    if let Some(date) = self.available_from_date {
      result.push(("AvailableFromDate".to_string(), date.to_iso8601()));
    }

    if let Some(date) = self.available_to_date {
      result.push(("AvailableToDate".to_string(), date.to_iso8601()));
    }

    if let Some(list) = self.report_request_id_list {
      for (i, id) in list.into_iter().enumerate() {
        result.push((format!("ReportRequestIdList.Id.{}", i + 1), id));
      }
    }

    result
  }
}

#[derive(Debug, Default)]
pub struct GetReportListResponse {
  pub request_id: String,
  pub reports: Vec<ReportInfo>,
  pub next_token: Option<String>,
  pub has_next: bool,
}

impl<S: decode::XmlEventStream> decode::FromXMLStream<S> for GetReportListResponse {
  fn from_xml(s: &mut S) -> decode::Result<GetReportListResponse> {
    use self::decode::{start_document, element, fold_elements, characters};
    start_document(s)?;
    element(s, "GetReportListResponse", |s| {
      fold_elements(s, GetReportListResponse::default(), |s, response| {
        match s.local_name() {
          "GetReportListResult" => {
            fold_elements(s, (), |s, _| {
              match s.local_name() {
                "ReportInfo" => {
                  let item = fold_elements(s, ReportInfo::default(), |s, info| {
                    match s.local_name() {
                      "ReportType" => {
                        info.report_type = characters(s)?;
                      },
                      "Acknowledged" => {
                        info.acknowledged = characters(s)?;
                      },
                      "AcknowledgedDate" => {
                        info.acknowledged_date = characters(s).map(Some)?;
                      },
                      "ReportId" => {
                        info.report_id = characters(s)?;
                      },
                      "AvailableDate" => {
                        info.available_date = characters(s).map(Some)?;                        
                      },
                      "ReportRequestId" => {
                        info.report_request_id = characters(s)?;
                      },
                      _ => {},
                    }
                    Ok(())
                  })?;
                  response.reports.push(item);
                },
                "HasNext" => {
                  response.has_next = characters(s)?;
                },
                "NextToken" => {
                  response.next_token = Some(characters(s)?);
                },
                _ => {},
              }
              Ok(())
            })
          },
          "ResponseMetadata" => {
            response.request_id = element(s, "RequestId", |s| {
              characters(s)
            })?;
            Ok(())
          },
          _ => { Ok(()) }
        }
      })
    })
  }
}

/// Returns a list of reports that were created in the previous 90 days.
#[allow(non_snake_case)]
pub fn GetReportList(client: &Client, params: GetReportListParameters) -> Result<Response<GetReportListResponse>> {
  client.request_xml(Method::Post, PATH, VERSION, "GetReportList", params).map_err(|err| err.into())
}

#[derive(Debug, Default)]
pub struct GetReportListByNextTokenResponse {
  pub request_id: String,
  pub reports: Vec<ReportInfo>,
  pub next_token: Option<String>,
  pub has_next: bool,
}

impl<S: decode::XmlEventStream> decode::FromXMLStream<S> for GetReportListByNextTokenResponse {
  fn from_xml(s: &mut S) -> decode::Result<GetReportListByNextTokenResponse> {
    use self::decode::{start_document, element, fold_elements, characters};
    start_document(s)?;
    element(s, "GetReportListByNextTokenResponse", |s| {
      fold_elements(s, GetReportListByNextTokenResponse::default(), |s, response| {
        match s.local_name() {
          "GetReportListByNextTokenResult" => {
            fold_elements(s, (), |s, _| {
              match s.local_name() {
                "ReportInfo" => {
                  let item = fold_elements(s, ReportInfo::default(), |s, info| {
                    match s.local_name() {
                      "ReportType" => {
                        info.report_type = characters(s)?;
                      },
                      "Acknowledged" => {
                        info.acknowledged = characters(s)?;
                      },
                      "AcknowledgedDate" => {
                        info.acknowledged_date = characters(s).map(Some)?;
                      },
                      "ReportId" => {
                        info.report_id = characters(s)?;
                      },
                      "AvailableDate" => {
                        info.available_date = characters(s).map(Some)?;                        
                      },
                      "ReportRequestId" => {
                        info.report_request_id = characters(s)?;
                      },
                      _ => {},
                    }
                    Ok(())
                  })?;
                  response.reports.push(item);
                },
                "HasNext" => {
                  response.has_next = characters(s)?;
                },
                "NextToken" => {
                  response.next_token = Some(characters(s)?);
                },
                _ => {},
              }
              Ok(())
            })
          },
          "ResponseMetadata" => {
            response.request_id = element(s, "RequestId", |s| {
              characters(s)
            })?;
            Ok(())
          },
          _ => { Ok(()) }
        }
      })
    })
  }
}

/// Returns a list of reports using the NextToken, which was supplied by a previous request to either 
/// GetReportListByNextToken or GetReportList, where the value of HasNext was true in the previous call.
#[allow(non_snake_case)]
pub fn GetReportListByNextToken(client: &Client, next_token: String) -> Result<Response<GetReportListByNextTokenResponse>> {
  let params = vec![("NextToken".to_string(), next_token)];  
  client.request_xml(Method::Post, PATH, VERSION, "GetReportListByNextToken", params).map_err(|err| err.into())
}

/// Returns the contents of a report and the Content-MD5 header for the returned report body.
#[allow(non_snake_case)]
pub fn GetReport<W: Write>(client: &Client, report_id: String, out: &mut W) -> Result<u64> {
  let params = vec![("ReportId".to_string(), report_id)];
  let mut resp = client.request(Method::Post, PATH, VERSION, "GetReport", params)?;
  let size = io::copy(&mut resp, out)?;
  Ok(size)
} 

#[deprecated]
#[allow(non_snake_case)]
pub fn GetFlatFileSettlementReport(client: &Client, report_id: String) -> Result<Response<SettlementReport>> {
  let params = vec![("ReportId".to_string(), report_id)];
  client.request_tdff(Method::Post, PATH, VERSION, "GetReport", params).map_err(|err| err.into())
}

#[cfg(test)]
mod tests {
  // use dotenv::dotenv;
  // use super::*;
  // use super::super::client::get_test_client;

  // #[test]
  // fn test_get_report_list() {
  //   dotenv().ok();
  //   let c = get_test_client();
  //   let mut params = GetReportListParameters::default();
  //   params.report_type_list = Some(vec![ReportType::_GET_V2_SETTLEMENT_REPORT_DATA_FLAT_FILE_]);
  //   let res = GetReportList(&c, params).expect("GetReportList");
  //   match res {
  //     Response::Error(e) => panic!("request error: {:?}", e),
  //     Response::Success(res) => {
  //       println!("{:?}", res);
  //     },
  //   }
  // }

  // #[test]
  // fn test_settlement_report() {
  //   dotenv().ok();
  //   let c = get_test_client();
  //   let res = GetFlatFileSettlementReport(&c, "3915548544017177".to_string()).expect("GetFlatFileSettlementReport");
  //   match res {
  //     Response::Error(e) => panic!("request error: {:?}", e),
  //     Response::Success(res) => {
  //       println!("{:?}", res);
  //     },
  //   }
  // }
}