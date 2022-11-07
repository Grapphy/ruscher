use crate::engines::LeecherModule;
use crate::error::Error;
use crate::proxies::LuminatioRotator;

use async_trait::async_trait;
use lazy_static::lazy_static;
use regex::Regex;

use std::collections::HashSet;

#[derive(Debug)]
pub struct GoogleSearch {
    pub client: reqwest::Client,
    pub lumproxy: Option<LuminatioRotator>,
}

#[derive(Debug)]
pub struct GoogleSearchBuilder {
    pub lumproxy: Option<LuminatioRotator>,
}

impl GoogleSearchBuilder {
    pub fn new() -> Self {
        GoogleSearchBuilder { lumproxy: None }
    }

    pub fn luminatio(mut self, lumproxy: LuminatioRotator) -> Self {
        self.lumproxy = Some(lumproxy);
        self
    }

    pub fn build(&self) -> Result<GoogleSearch, Error> {
        let mut client = reqwest::Client::builder();
        if let Some(lumproxy) = &self.lumproxy {
            client = client.proxy(reqwest::Proxy::all(lumproxy.proxy())?)
        }

        let client = client.build()?;
        Ok(GoogleSearch::new(client.clone(), self.lumproxy.clone()))
    }
}

impl GoogleSearch {
    pub fn new(client: reqwest::Client, lumproxy: Option<LuminatioRotator>) -> Self {
        GoogleSearch {
            client: client,
            lumproxy: lumproxy,
        }
    }

    pub fn builder() -> GoogleSearchBuilder {
        GoogleSearchBuilder::new()
    }

    pub fn process_content(&self, content: &str) -> Vec<String> {
        lazy_static! {
            static ref REGEX_CMP: Regex =
                Regex::new("(https://pastebin.com/[a-zA-Z1-9]*)").unwrap();
        }
        REGEX_CMP
            .find_iter(content)
            .into_iter()
            .filter_map(|val| val.as_str().parse().ok())
            .collect()
    }

    pub async fn search(&self, query: String) -> Result<Vec<String>, Error> {
        let mut results: HashSet<String> = HashSet::new();
        let mut searching = true;
        let mut page = 1;

        while searching {
            let url = format!(
                "https://www.google.com/search?q={}&gws_rd=cr,ssl&client=ubuntu&ie=UTF-8&start={}0",
                query, page
            );

            let text = self
                .client
                .get(url)
                .header(
                    "User-Agent",
                    "Mozilla/5.0 (Windows NT 10.0; rv:91.0) Gecko/20100101 Firefox/91.0",
                )
                .send()
                .await?
                .text()
                .await?;
            println!("{}", text);
            let matches = self.process_content(&text);
            if matches.is_empty() {
                searching = false;
            } else {
                page += 1;
                results.extend(matches);
            }
        }

        Ok(results.into_iter().collect())
    }
}
