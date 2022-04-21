use std::{env, io::{Read, self, Write}, fs::File};

use clap::Parser;
use dotenv::dotenv;
use reqwest::header::HeaderValue;

#[derive(Parser, Debug)]
struct CliArgs {
    #[clap(short = 'f', long = "from-lang")]
    from_lang: String,
    #[clap(short = 't', long = "to-lang")]
    to_lang: String,
    from_path: std::path::PathBuf,
    to_path: std::path::PathBuf,
}

// https://rust-cli.github.io/book/tutorial/cli-args.html
#[tokio::main]
async fn main() -> io::Result<()>{
    let args = CliArgs::parse();
    println!("{:?}", args);

    let mut f = File::open(args.from_path.as_path())?;
    let mut buffer = String::new();
    f.read_to_string(&mut buffer)?;

    dotenv().ok();
    let translate_api_id = env::var("TRANSLATE_API_ID").unwrap();

    let client = reqwest::Client::new();
    let params = [
        ("sl", args.from_lang),
        ("tl", args.to_lang),
        ("q", buffer),
    ];

    let url = format!("{}{}", "https://translate.google.com/translate_a/single?client=at&dt=t&dt=ld&dt=qca&dt=rm&dt=bd&dj=1&hl=ja&ie=UTF-8&oe=UTF-8&inputm=2&otf=2&iid=", translate_api_id.as_str());
    let res = client.post(url)
        .header(reqwest::header::CONTENT_TYPE,  HeaderValue::from_static("application/x-www-form-urlencoded;charset=utf-8"))
        .header(reqwest::header::USER_AGENT, HeaderValue::from_static("AndroidTranslate/5.3.0.RC02.130475354-53000263 5.1 phone TRANSLATE_OPM5_TEST_1"))
        .form(&params)
        .send()
        .await
        .unwrap();

    let mut tf =  File::create(args.to_path)?;
    let res_buffer = res.text().await.unwrap();
    tf.write_fmt(format_args!("{}", res_buffer));

    Ok(())
}
