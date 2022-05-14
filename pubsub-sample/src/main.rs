use google_api_proto::google::pubsub::v1::publisher_client::PublisherClient;
use google_api_proto::google::pubsub::v1::{ListTopicsRequest, Topic};
use tonic::transport::Channel;
use tonic::{Code, Request};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let channel = Channel::from_static("http://localhost:8085")
        .connect()
        .await?;

    let mut pubsub = PublisherClient::new(channel);
    create_topic(&mut pubsub).await?;

    list_topic(&mut pubsub).await?;

    Ok(())
}

async fn create_topic(pubsub: &mut PublisherClient<Channel>) -> anyhow::Result<()> {
    println!("create topic");

    let res = pubsub
        .create_topic(Request::new(Topic {
            name: "projects/test/topics/test".to_string(),
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

async fn list_topic(pubsub: &mut PublisherClient<Channel>) -> anyhow::Result<()> {
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
