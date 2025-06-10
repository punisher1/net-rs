mod cli;
mod ui;
// mod protocols;
mod config;
mod crossterm;
mod utils;
mod app;

use std::time::Duration;

use anyhow::Result;
use cli::args::parse_args;


#[tokio::main]
async fn main() -> Result<()> {
    // 解析命令行参数
    let args = parse_args();

    // 运行主应用
    // let app = App::new(args)?;
    // app.run().await?;

    let tick_rate = Duration::from_millis(100);
    crossterm::run(tick_rate, true,args).unwrap();
    Ok(())
}
