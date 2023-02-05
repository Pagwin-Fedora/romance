extern crate tokio;
extern crate dirs;
extern crate async_trait;
extern crate serde;
extern crate serde_json;
extern crate uuid;
extern crate serde_yaml;
mod job;
mod env;
mod setup;

#[tokio::main]
async fn main() -> Result<(),std::io::Error> {
    setup::setup_dirs()?;
    let jobs = setup::collect_jobs().await?;
    for mut job in jobs{
        setup::reset_repo()?;
        job.execute_steps().await;
    }
    Ok(())
}
