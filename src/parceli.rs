use futures::future::join_all;
use std::collections::HashMap;

use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE},
    Client,
};

use crate::{
    error::ParceliError,
    models::{api, local},
};

use super::ARGS;

#[derive(Debug)]
pub struct Parceli {
    client: Client,
}

impl Parceli {
    pub fn new() -> Parceli {
        let mut header_map = HeaderMap::new();
        header_map.append(
            AUTHORIZATION,
            HeaderValue::from_str(format!("Bearer {}", ARGS.api_key).as_str()).unwrap(),
        );
        header_map.append(
            CONTENT_TYPE,
            HeaderValue::from_static("application/json; charset=utf-8"),
        );

        let client = Client::builder()
            .default_headers(header_map)
            .build()
            .expect("Unable to create reqwest client");

        Parceli { client }
    }

    pub async fn track(&self) -> Result<Vec<local::Parcel>, ParceliError> {
        let mut tasks = vec![];

        for shared_id in &ARGS.parcel_ids {
            let client = self.client.clone();
            let id = shared_id.clone();

            tasks.push(tokio::spawn(async move {
                let mut body = HashMap::new();
                body.insert("trackingNumber", id.as_str());

                let response = client
                    .post("https://api.ship24.com/public/v1/tracking/search")
                    .json(&body)
                    .send()
                    .await?;

                let api_response_data = response.json::<api::TrackingsResponse>().await?;

                let local_response_data: Vec<local::Parcel> = api_response_data.into();

                if let Some(i) = local_response_data.get(0) {
                    Ok(i.clone())
                } else {
                    Err(ParceliError::NoData)
                }
            }));
        }

        let join_results = join_all(tasks).await;
        let mut results = vec![];

        for res in join_results {
            results.push(match res.unwrap() {
                Ok(parcel) => parcel,
                Err(e) => return Err(e),
            });
        }

        Ok(results)
    }
}
