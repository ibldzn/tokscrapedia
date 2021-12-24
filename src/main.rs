use std::{
    fs::File,
    io::Write,
    sync::{Arc, Mutex},
};

use anyhow::Result;
use tokscrapedia::{App, Tokped};

#[tokio::main]
async fn main() -> Result<()> {
    let app = Arc::new(App::parse_from_args());

    let file = Arc::new(Mutex::new(File::create(&app.output)?));
    file.lock()
        .unwrap()
        .write_all(b"name,price,priceint,url\n")?;

    Tokped::new(reqwest::Client::new())
        .search(app, move |res| {
            for product in res.data.products {
                let line = format!(
                    "\"{}\",{},{},{}\n",
                    product.name.replace("\"", ""),
                    product.price,
                    product.price_int,
                    product.url
                );

                file.lock()
                    .unwrap()
                    .write_all(line.as_bytes())
                    .expect("Failed to write to file!");
            }
        })
        .await;

    Ok(())
}
