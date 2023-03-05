pub mod handler;
pub mod file;

// For this dummy application we only need two IO event
#[derive(Debug, Clone)]
pub enum IoEvent {
    Initialize, 
    Timer,
    TimeUp,
    FinishText
}
