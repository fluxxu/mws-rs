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
