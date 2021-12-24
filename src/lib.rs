mod app;
mod types;

use std::{collections::HashMap, sync::Arc};

pub use app::App;
use futures::stream::FuturesUnordered;
use tokio::sync::Semaphore;
pub use types::SearchResult;

pub struct Tokped {
    client: reqwest::Client,
}

impl Tokped {
    pub fn new(client: reqwest::Client) -> Self {
        Self { client }
    }

    pub async fn search<F>(&self, app: Arc<App>, callback: F)
    where
        F: FnMut(SearchResult) + Clone + Send + 'static,
    {
        const BASE_URL: &str = "https://ace.tokopedia.com/search/product/v3";
        const MAX_RESULTS_PER_PAGE: usize = 200;
        const MAX_CONCURRENT_REQUESTS: usize = 10;

        let semaphore = Arc::new(Semaphore::new(MAX_CONCURRENT_REQUESTS));

        let page_data = {
            let max_page = (app.max_results as f32 / MAX_RESULTS_PER_PAGE as f32).ceil() as usize;

            (0..max_page)
                .map(|i| {
                    (
                        { i * MAX_RESULTS_PER_PAGE }.to_string(),
                        if i == max_page - 1 {
                            app.max_results - i * MAX_RESULTS_PER_PAGE
                        } else {
                            MAX_RESULTS_PER_PAGE
                        }
                        .to_string(),
                    )
                })
                .collect::<Vec<_>>()
        };

        let futures = FuturesUnordered::new();

        for (offset, limit) in page_data {
            let app = app.clone();
            let client = self.client.clone();
            let semaphore = semaphore.clone();
            let mut callback = callback.clone();

            futures.push(tokio::spawn(async move {
                let mut params = HashMap::from([
                    ("device", String::from("desktop")),
                    ("full_domain", String::from("www.tokopedia.com")),
                    ("q", app.query.clone()),
                    ("rows", limit),
                    ("scheme", String::from("https")),
                    ("source", String::from("shop_product")),
                    ("st", String::from("product")),
                    ("start", offset),
                ]);

                if let Some(min_price) = app.min_price {
                    params.insert("pmin", min_price.to_string());
                }

                if let Some(max_price) = app.max_price {
                    params.insert("pmax", max_price.to_string());
                }

                let sem = semaphore.acquire().await.unwrap();
                let resp = client.get(BASE_URL).query(&params).send().await;
                drop(sem);

                match resp {
                    Ok(resp) => match resp.json::<SearchResult>().await {
                        Ok(data) => {
                            tokio::spawn(async move {
                                callback(data);
                            })
                            .await
                            .expect("Failed to spawn callback!");
                        }
                        Err(e) => {
                            eprintln!("[!] Error: {}", e);
                        }
                    },
                    Err(e) => {
                        eprintln!("[!] Error: {}", e);
                    }
                }
            }));
        }

        futures::future::join_all(futures).await;
    }
}
