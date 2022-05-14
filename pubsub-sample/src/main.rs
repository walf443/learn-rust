use google_api_proto::google::pubsub::v1::{ListTopicsRequest, PublishRequest, Topic};
use google_api_proto::google::pubsub::v1::publisher_client::PublisherClient;
use tonic::{Code, Request, Status};
use tonic::transport::Channel;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("create topic");
    let channel = Channel::from_static("http://localhost:8085").connect().await?;

    let mut pubsub = PublisherClient::new(channel);
    let res = pubsub.create_topic(Request::new(Topic {
        name: "projects/test/topics/test".to_string(),
        labels: Default::default(),
        message_storage_policy: None,
        kms_key_name: "".to_string(),
        schema_settings: None,
        satisfies_pzs: false,
        message_retention_duration: None
    })).await;

    if res.is_err() {

    }
    if let Err(status) = res {
        if status.code() !=  Code::AlreadyExists {
            return Err(status.into());
        }
    } else {
        println!("response = {:#?}", res);
    }

    println!("list topics");

    let res = pubsub.list_topics(Request::new(ListTopicsRequest {
        project: format!("projects/{}", "test"),
        page_size: 10,
        ..Default::default()
    })).await?;

    for topic in &res.get_ref().topics {
        println!("topic = {:#?}", topic);
    }


    Ok(())
}
