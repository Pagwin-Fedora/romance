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

#[tokio::main( flavor = "current_thread")]
async fn main() -> Result<(),std::io::Error> {
    let args = std::env::args().collect::<Vec<String>>();
    if let Some("xoxo") = args.get(0).map(String::as_str){
        println!("huggies and kissies xoxo");
        std::process::exit(0);
    }
    setup::setup_dirs()?;
    setup::reset_repo()?;
    let jobs = setup::collect_jobs().await?;
    for mut job in jobs{
        setup::reset_repo()?;
        job.status_update().await?;
        job.execute_steps().await;
    }
    Ok(())
}
