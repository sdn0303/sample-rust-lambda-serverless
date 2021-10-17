use lambda_runtime::{handler_fn, Context, Error};
use log::LevelFilter;
use serde_json::Value;
use simple_logger::SimpleLogger;


async fn handler(event: Value, _ctx: Context) -> Result<Value, Error> {
    log::info!(event);
    Ok(event)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    SimpleLogger::new().with_level(LevelFilter::Info).init().unwrap();
    lambda_runtime::run(
        handler_fn(move |e, c| {
            log::info!("start to process something");
            handler(e, c)
        })
    ).await?;
    Ok(())
}
