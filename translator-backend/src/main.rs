extern crate dotenv;

use dotenv::dotenv;
use std::env;
use error_chain::error_chain;
use reqwest::header::{HeaderValue};

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}


#[tokio::main]
async fn main() -> Result<()>{
    dotenv().ok();
    let translate_id = env::var("TRANSLATE_API_ID").unwrap();
    let client = reqwest::Client::new();
    let params = [
        ("sl", "en"),
        ("tl", "ja"),
        ("q", "Inoue became inspired to make Slam Dunk as he liked basketball since his high school years.  After Inoue started Slam Dunk, he was surprised when he began receiving letters from readers that said they started playing the sport due to the manga.  His editor even told him \"basketball was a taboo in this world.\" Due to these letters, Inoue decided he wanted to draw better basketball games in the series."),
    ];
    let url = format!("{}{}", "https://translate.google.com/translate_a/single?client=at&dt=t&dt=ld&dt=qca&dt=rm&dt=bd&dj=1&hl=ja&ie=UTF-8&oe=UTF-8&inputm=2&otf=2&iid=", translate_id.as_str());
    let res = client.post(url)
        // .headers(headers)
        .header(reqwest::header::CONTENT_TYPE,  HeaderValue::from_static("application/x-www-form-urlencoded;charset=utf-8"))
        .header(reqwest::header::USER_AGENT, HeaderValue::from_static("AndroidTranslate/5.3.0.RC02.130475354-53000263 5.1 phone TRANSLATE_OPM5_TEST_1"))
        .form(&params)
        .send()
        .await?;

    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());
    println!("Body:\n{}", res.text().await?);

    Ok(())
}
