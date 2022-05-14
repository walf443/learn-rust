use google_api_proto::google::pubsub::v1::publisher_client::PublisherClient;
use google_api_proto::google::pubsub::v1::{
    ListTopicsRequest, PublishRequest, PubsubMessage, Topic,
};
use tonic::codegen::Bytes;
use tonic::transport::Channel;
use tonic::{Code, Request};

type PubSub = PublisherClient<Channel>;

struct TopicName(String);

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let channel = Channel::from_static("http://localhost:8085")
        .connect()
        .await?;

    let mut pubsub = PublisherClient::new(channel);
    let topic_name = TopicName("projects/test/topics/test".to_string());
    create_topic(&mut pubsub, &topic_name).await?;

    list_topic(&mut pubsub).await?;

    publish_topic(&mut pubsub, &topic_name).await?;

    Ok(())
}

async fn create_topic(pubsub: &mut PubSub, topic_name: &TopicName) -> anyhow::Result<()> {
    println!("create topic");

    let res = pubsub
        .create_topic(Request::new(Topic {
            name: topic_name.0.clone(),
            labels: Default::default(),
            message_storage_policy: None,
            kms_key_name: "".to_string(),
            schema_settings: None,
            satisfies_pzs: false,
            message_retention_duration: None,
        }))
        .await;

    if let Err(status) = res {
        if status.code() != Code::AlreadyExists {
            return Err(status.into());
        }
    } else {
        println!("response = {:#?}", res);
    }

    Ok(())
}

async fn list_topic(pubsub: &mut PubSub) -> anyhow::Result<()> {
    println!("list topics");

    let res = pubsub
        .list_topics(Request::new(ListTopicsRequest {
            project: format!("projects/{}", "test"),
            page_size: 10,
            ..Default::default()
        }))
        .await?;

    for topic in &res.get_ref().topics {
        println!("topic = {:#?}", topic);
    }

    Ok(())
}

async fn publish_topic(pubsub: &mut PubSub, topic_name: &TopicName) -> anyhow::Result<()> {
    println!("publish topic");

    let res = pubsub
        .publish(PublishRequest {
            topic: topic_name.0.clone(),
            messages: vec![PubsubMessage {
                data: Bytes::from_static(b"this is test"),
                attributes: Default::default(),
                message_id: "1".to_string(),
                publish_time: None,
                ordering_key: "1".to_string(),
            }],
        })
        .await?;

    println!("response = {:#?}", res);

    Ok(())
}
