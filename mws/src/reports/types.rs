use chrono::{DateTime, Utc};

#[allow(non_snake_case)]
#[derive(Debug, Default, PartialEq, Serialize, FromXmlStream)]
pub struct ReportInfo {
  pub ReportType: String,
  pub Acknowledged: bool,
  pub AcknowledgedDate: Option<DateTime<Utc>>,
  pub ReportId: String,
  pub AvailableDate: Option<DateTime<Utc>>,
  pub ReportRequestId: String,
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

#[allow(non_snake_case)]
#[derive(Debug, Default, PartialEq, Serialize, FromXmlStream)]
pub struct ReportRequestInfo {
  pub ReportRequestId: String,
  pub ReportType: String,
  pub StartDate: Option<DateTime<Utc>>,
  pub EndDate: Option<DateTime<Utc>>,
  pub Scheduled: bool,
  pub SubmittedDate: Option<DateTime<Utc>>,
  pub ReportProcessingStatus: ReportProcessingStatus,
  pub GeneratedReportId: Option<String>,
  pub StartedProcessingDate: Option<DateTime<Utc>>,
  pub CompletedDate: Option<DateTime<Utc>>,
}
