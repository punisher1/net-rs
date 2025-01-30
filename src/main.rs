use app::App;

mod app;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), anyhow::Error> {
    let mut terminal = ratatui::init();
    let app = App::default().run(&mut terminal)?;

    ratatui::restore();

    Ok(app)
}
