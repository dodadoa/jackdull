use std::sync::Arc;
use std::time::Duration;

use eyre::Result;
use log::{error, info};

use super::IoEvent;
use crate::{app::App, io::file::read_file};

pub struct IoAsyncHandler {
    app: Arc<tokio::sync::Mutex<App>>,
}

impl IoAsyncHandler {
    pub fn new(app: Arc<tokio::sync::Mutex<App>>) -> Self {
        Self { app }
    }

    pub async fn handle_io_event(&mut self, io_event: IoEvent) {
        let result = match io_event {
            IoEvent::Initialize => self.do_initialize().await,
            IoEvent::Timer => self.timer().await,
            IoEvent::TimeUp => self.timeup().await,
            IoEvent::FinishText => self.finished_text().await,
        };

        if let Err(err) = result {
            error!("Oops, something wrong happen: {:?}", err);
        }
    }

    async fn do_initialize(&mut self) -> Result<()> {
        info!("🚀 Initialize the application");

        let mut app = self.app.lock().await;
        app.loading();

        tokio::time::sleep(Duration::from_secs(1)).await;
        app.initialized();

        let data_from_file = read_file().await;
        app.set_typing_information(data_from_file.clone());

        app.load_text(data_from_file.content);
        app.set_words_count(data_from_file.words_count);

        info!("👍 Application initialized");

        app.loaded();

        Ok(())
    }

    async fn timer(&mut self) -> Result<()> {
        let mut app = self.app.lock().await;
        app.update_on_tick().await;
        info!("Increase timer");

        Ok(())
    }

    async fn timeup(&mut self) -> Result<()> {
        let mut app = self.app.lock().await;
        app.send_message_timeup().await;
        info!("Time is up");

        Ok(())
    }

    async fn finished_text(&mut self) -> Result<()> {
        let mut app = self.app.lock().await;
        app.finished_text().await;
        info!("Finished text");

        Ok(())
    }
}
