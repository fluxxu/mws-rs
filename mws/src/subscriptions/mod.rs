//! Amazon MWS Subscriptions API - Version 2013-07-01
//!
//! [Reference](http://docs.developer.amazonservices.com/en_US/subscriptions/Subscriptions_Overview.html)

use client::{Client, Method};
use result::MwsResult;

pub mod types;
pub use self::types::*;
pub mod notification;

static PATH: &'static str = "/Subscriptions/2013-07-01";
static VERSION: &'static str = "2013-07-01";

#[allow(non_snake_case)]
#[derive(Debug, Default, Serialize, SerializeMwsParams)]
pub struct RegisterDestinationParameters {
  pub MarketplaceId: String,
  pub Destination: Destination,
}

/// Specifies a new destination where you want to receive notifications.
#[allow(non_snake_case)]
pub fn RegisterDestination(
  client: &Client,
  params: RegisterDestinationParameters,
) -> MwsResult<()> {
  client
    .request_xml(Method::Post, PATH, VERSION, "RegisterDestination", params)
    .map_err(|err| err.into())
}

/// Removes an existing destination from the list of registered destinations.
#[allow(non_snake_case)]
pub fn DeregisterDestination(
  client: &Client,
  params: RegisterDestinationParameters,
) -> MwsResult<()> {
  client
    .request_xml(Method::Post, PATH, VERSION, "DeregisterDestination", params)
    .map_err(|err| err.into())
}

#[derive(FromXmlStream, Default, Debug)]
#[allow(non_snake_case)]
pub struct ListRegisteredDestinationsResponse {
  pub DestinationList: Vec<Destination>,
}

response_envelope_type!(
  ListRegisteredDestinationsResponseEnvelope<ListRegisteredDestinationsResponse>,
  "ListRegisteredDestinationsResponse",
  "ListRegisteredDestinationsResult"
);

/// Removes an existing destination from the list of registered destinations.
#[allow(non_snake_case)]
pub fn ListRegisteredDestinations(
  client: &Client,
  marketplace_id: String,
) -> MwsResult<ListRegisteredDestinationsResponse> {
  client
    .request_xml(
      Method::Post,
      PATH,
      VERSION,
      "ListRegisteredDestinations",
      vec![("MarketplaceId".to_string(), marketplace_id)],
    )
    .map(|e: ListRegisteredDestinationsResponseEnvelope| e.into_inner())
    .map_err(|err| err.into())
}

/// Sends a test notification to an existing destination.
#[allow(non_snake_case)]
pub fn SendTestNotificationToDestination(
  client: &Client,
  params: RegisterDestinationParameters,
) -> MwsResult<()> {
  client
    .request_xml(
      Method::Post,
      PATH,
      VERSION,
      "SendTestNotificationToDestination",
      params,
    )
    .map_err(|err| err.into())
}

#[allow(non_snake_case)]
#[derive(Debug, Default, Serialize, SerializeMwsParams)]
pub struct CreateSubscriptionParameters {
  pub MarketplaceId: String,
  pub Subscription: Subscription,
}

/// Creates a new subscription for the specified notification type and destination.
#[allow(non_snake_case)]
pub fn CreateSubscription(client: &Client, params: CreateSubscriptionParameters) -> MwsResult<()> {
  client
    .request_xml(Method::Post, PATH, VERSION, "CreateSubscription", params)
    .map_err(|err| err.into())
}

#[allow(non_snake_case)]
#[derive(Debug, Default, Serialize, SerializeMwsParams)]
pub struct GetSubscriptionParameters {
  pub MarketplaceId: String,
  pub NotificationType: NotificationType,
  pub Destination: Destination,
}

#[derive(FromXmlStream, Default, Debug)]
#[allow(non_snake_case)]
pub struct GetSubscriptionResponse {
  pub Subscription: Subscription,
}

response_envelope_type!(
  GetSubscriptionResponseEnvelope<GetSubscriptionResponse>,
  "GetSubscriptionResponse",
  "GetSubscriptionResult"
);

/// Gets the subscription for the specified notification type and destination.
#[allow(non_snake_case)]
pub fn GetSubscription(
  client: &Client,
  params: GetSubscriptionParameters,
) -> MwsResult<GetSubscriptionResponse> {
  client
    .request_xml(Method::Post, PATH, VERSION, "GetSubscription", params)
    .map(|e: GetSubscriptionResponseEnvelope| e.into_inner())
    .map_err(|err| err.into())
}

#[derive(FromXmlStream, Default, Debug)]
#[allow(non_snake_case)]
pub struct ListSubscriptionsResponse {
  pub SubscriptionList: Vec<Subscription>,
}

response_envelope_type!(
  ListSubscriptionsResponseEnvelope<ListSubscriptionsResponse>,
  "ListSubscriptionsResponse",
  "ListSubscriptionsResult"
);

/// Returns a list of all your current subscriptions.
#[allow(non_snake_case)]
pub fn ListSubscriptions(
  client: &Client,
  marketplace_id: String,
) -> MwsResult<ListSubscriptionsResponse> {
  client
    .request_xml(
      Method::Post,
      PATH,
      VERSION,
      "ListSubscriptions",
      vec![("MarketplaceId".to_string(), marketplace_id)],
    )
    .map(|e: ListSubscriptionsResponseEnvelope| e.into_inner())
    .map_err(|err| err.into())
}

/// Updates the subscription for the specified notification type and destination.
#[allow(non_snake_case)]
pub fn UpdateSubscription(client: &Client, params: CreateSubscriptionParameters) -> MwsResult<()> {
  client
    .request_xml(Method::Post, PATH, VERSION, "UpdateSubscription", params)
    .map_err(|err| err.into())
}
