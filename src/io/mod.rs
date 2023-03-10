pub mod file;
pub mod handler;

// For this dummy application we only need two IO event
#[derive(Debug, Clone)]
pub enum IoEvent {
    Initialize,
    Timer,
    TimeUp,
    FinishText,
}
