use crate::engines::LeecherModule;
use crate::error::Error;
use crate::processor::process_content;
use futures::{stream, StreamExt};
use std::collections::HashSet;

pub async fn leech_module<M: LeecherModule>(module: M) -> Result<Vec<String>, Error> {
    let targets = module.urls().await?;

    let mut results: HashSet<String> = HashSet::new();

    async fn get_content(client: &reqwest::Client, url: String) -> Result<Vec<String>, Error> {
        let body = client.get(url).send().await?.text().await?;
        Ok(process_content(&body))
    }

    let client = reqwest::Client::new();

    stream::iter(targets)
        .map(|target| {
            let client = &client;
            async move { get_content(client, target).await }
        })
        .buffer_unordered(10)
        .filter_map(|c| futures::future::ready(c.ok()))
        .for_each(|res| {
            results.extend(res);
            futures::future::ready(())
        })
        .await;

    Ok(results.into_iter().collect())
}
