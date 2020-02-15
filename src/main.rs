use aws_nuke::{config, nuke};
use lambda_runtime::{error::HandlerError, lambda, Context};
use serde_derive::Deserialize;
use {chrono::Utc, fern};
use rusoto_s3::{S3, S3Client, GetObjectRequest};
use rusoto_core::Region;
use std::str::FromStr;
use std::io::Read;

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Event {
    /// Name of the bucket in S3 where the config file exists
    bucket_name: String,
    /// Path in S3 to the config file that aws-nuke accepts
    object_key: String,
    /// Region where the config file's bucket is located in
    region: String,
    /// Logging level 0 -> Error, 1 -> Warn, 2 -> Info, 3 -> Debug, 4 -> Trace
    log_level: u8
}

fn main() {
    lambda!(handler)
}

fn handler(event: Event, _: Context) -> Result<(), HandlerError> {
    setup_logging(&event.log_level);

    let s3_client = S3Client::new(Region::from_str(&event.region).unwrap_or_default());
    let resp = s3_client
        .get_object(GetObjectRequest {
            bucket: event.bucket_name,
            key: event.object_key,
            ..Default::default()
        })
        .sync()
        .expect("Failed to get the config file from S3");

    let mut stream = resp.body.unwrap().into_blocking_read();
    let mut bytes: Vec<u8> = Vec::new();
    stream.read_to_end(&mut bytes).unwrap();
    let file_contents = std::str::from_utf8(&bytes).expect("Failed to read the contents of the config file");

    let config = config::parse_config(&file_contents);

    println!("{:?}", config);

    nuke::Nuke::new(config).run().unwrap();

    Ok(())
}

fn setup_logging(logging_level: &u8) {
    let level = match logging_level {
        0 => log::LevelFilter::Error,
        1 => log::LevelFilter::Warn,
        2 => log::LevelFilter::Info,
        3 => log::LevelFilter::Debug,
        _ => log::LevelFilter::Trace,
    };
    
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{}][{}][{}][{}] {}",
                record.module_path().unwrap(),
                record.line().unwrap(),
                Utc::now().to_rfc3339(),
                record.level(),
                message
            ))
        })
        .level(level)
        .level_for("rustls", log::LevelFilter::Info)
        .level_for("tokio_reactor", log::LevelFilter::Info)
        .level_for("hyper", log::LevelFilter::Info)
        .level_for("rusoto_core", log::LevelFilter::Info)
        .level_for("tokio_threadpool", log::LevelFilter::Info)
        .level_for("mio", log::LevelFilter::Info)
        .level_for("want", log::LevelFilter::Info)
        .chain(std::io::stdout())
        .apply()
        .expect("could not set up logging");
}