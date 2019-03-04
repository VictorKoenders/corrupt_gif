use lambda_runtime::{error::LambdaErrorExt, lambda, Context};
use reqwest::Error as ReqwestError;
use rusoto_core::Region;
use rusoto_s3::{PutObjectError, PutObjectRequest, S3Client, S3};
use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Clone)]
struct Input {
    #[serde(rename = "sourceUrl")]
    source_url: String,
}

#[derive(Serialize, Clone)]
struct Output {
    #[serde(rename = "outputUrl")]
    output_url: String,
}

fn main() {
    lambda!(my_handler);
}

#[derive(Default)]
pub struct EnvConfig {
    pub region: Region,
    pub bucket_name: String,
    pub base_url: String,
}

fn my_handler(input: Input, _c: Context) -> Result<Output, Error> {
    if input.source_url.is_empty() {
        return Err(Error::NoSourceUrl);
    }

    // TODO: Get this from env
    let config = EnvConfig::default();

    let mut result = reqwest::get(&input.source_url).map_err(Error::ReqwestError)?;
    let mut source = Vec::new();
    result.copy_to(&mut source).map_err(Error::ReqwestError)?;

    let output = corrupted_gif::generate(&source);

    let client = S3Client::new(config.region);
    let id = Uuid::new_v4();
    client
        .put_object(PutObjectRequest {
            body: Some(output.into()),
            bucket: config.bucket_name,
            content_type: Some(String::from("image/gif")),
            key: format!("{}.gif", id),
            acl: Some(String::from("public-read")),
            ..Default::default()
        })
        .sync()
        .map_err(Error::PutObjectError)?;

    Ok(Output {
        output_url: format!("{}{}.gif", config.base_url, id),
    })
}

#[derive(Debug)]
pub enum Error {
    NoSourceUrl,
    ReqwestError(ReqwestError),
    PutObjectError(PutObjectError),
}

impl std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::NoSourceUrl => write!(fmt, "Missing parameter: sourceUrl"),
            Error::ReqwestError(e) => write!(fmt, "Could not load source image: {}", e),
            Error::PutObjectError(e) => write!(fmt, "Could not upload gif to AWS: {}", e),
        }
    }
}

impl LambdaErrorExt for Error {
    fn error_type(&self) -> &str {
        "Error"
    }
}

impl std::error::Error for Error {}
