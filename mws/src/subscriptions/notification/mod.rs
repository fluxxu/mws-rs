use chrono::{DateTime, Utc};

pub mod any_offer_changed;

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

#[allow(non_snake_case)]
#[derive(Debug, Default, PartialEq, Serialize)]
pub struct NotificationEnvelope<T> {
  pub NotificationPayload: T,
  pub NotificationMetaData: NotificationMetaData,
}

#[cfg(test)]
mod tests {
  use super::*;
  use chrono::{TimeZone, Utc};

  #[derive(Debug, FromXmlStream, Default, PartialEq)]
  pub struct TestPayload {
    pub value: i32,
  }

  notification_envelope_type!(TestEnvelope<TestPayload>);

  #[test]
  fn test_notification_envelope() {
    test_decode_envelope!(
      TestEnvelope,
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
              <TestPayload>
                <value>42</value>
              </TestPayload>
          </NotificationPayload>
        </Notification>
      "#,
      NotificationEnvelope::<TestPayload> {
        NotificationMetaData: NotificationMetaData {
          NotificationType: "Test".to_string(),
          PayloadVersion: "1.0".to_string(),
          UniqueId: "db05ce25-a2d6-49f7-bb39-5419b1a17a26".to_string(),
          PublishTime: Some(Utc.ymd(2020, 2, 3).and_hms(21, 50, 30)),
          SellerId: "A23AS8PFN4IRUQ".to_string(),
          MarketplaceId: "ATVPDKIKX0DER".to_string(),
        },
        NotificationPayload: TestPayload { value: 42 }
      }
    )
  }
}
