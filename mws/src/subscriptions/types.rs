//! https://docs.developer.amazonservices.com/en_UK/subscriptions/Subscriptions_Datatypes.html#Destination

str_enum! {
  pub enum DeliveryChannel {
    SQS,
  }
}

str_enum! {
  pub enum AttributeKey {
    sqsQueueUrl,
  }
}

str_enum! {
  pub enum NotificationType {
    AnyOfferChanged,
    FeedProcessingFinished,
    FBAOutboundShipmentStatus,
    FeePromotion,
    FulfillmentOrderStatus,
    ReportProcessingFinished,
  }
}

#[allow(non_snake_case)]
#[derive(Debug, Default, PartialEq, Serialize, FromXmlStream, SerializeMwsParams)]
pub struct AttributeKeyValue {
  pub Key: AttributeKey,
  pub Value: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Default, PartialEq, Serialize, SerializeMwsParams, FromXmlStream)]
pub struct Destination {
  pub DeliveryChannel: DeliveryChannel,
  pub AttributeList: Vec<AttributeKeyValue>,
}

#[allow(non_snake_case)]
#[derive(Debug, Default, PartialEq, Serialize, SerializeMwsParams, FromXmlStream)]
pub struct Subscription {
  pub NotificationType: NotificationType,
  pub Destination: Destination,
  pub IsEnabled: bool,
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_destination() {
    test_decode!(
      Destination,
      r#"
        <DeliveryChannel>SQS</DeliveryChannel>
        <AttributeList>
          <member>
            <Value>https://sqs.us-east-1.amazonaws.com/111111111/test</Value>
            <Key>sqsQueueUrl</Key>
          </member>
        </AttributeList>
      "#,
      Destination {
        DeliveryChannel: DeliveryChannel::SQS,
        AttributeList: vec![AttributeKeyValue {
          Key: AttributeKey::sqsQueueUrl,
          Value: "https://sqs.us-east-1.amazonaws.com/111111111/test".to_string(),
        }],
      }
    );
  }

  #[test]
  fn test_subscription() {
    test_decode!(
      Subscription,
      r#"
        <Destination>
          <DeliveryChannel>SQS</DeliveryChannel>
          <AttributeList>
            <member>
              <Value>https://sqs.us-east-1.amazonaws.com/111111111/test</Value>
              <Key>sqsQueueUrl</Key>
            </member>
          </AttributeList>
        </Destination>
        <NotificationType>AnyOfferChanged</NotificationType>
        <IsEnabled>false</IsEnabled>
      "#,
      Subscription {
        NotificationType: NotificationType::AnyOfferChanged,
        IsEnabled: false,
        Destination: Destination {
          DeliveryChannel: DeliveryChannel::SQS,
          AttributeList: vec![AttributeKeyValue {
            Key: AttributeKey::sqsQueueUrl,
            Value: "https://sqs.us-east-1.amazonaws.com/111111111/test".to_string(),
          }],
        }
      }
    );
  }
}
