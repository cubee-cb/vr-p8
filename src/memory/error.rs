use crate::runtime_connection;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Runtime connection error")]
    RuntimeConnection(#[from] runtime_connection::Error),

    #[error("Ctrl-C handler error")]
    CtrlC(#[from] ctrlc::Error),

    #[error("Killed by Ctrl-C")]
    KilledByCtrlC,

    #[error("Channel error")]
    RecvError(#[from] std::sync::mpsc::RecvError),

    #[error("I/O error")]
    IOError(#[from] std::io::Error),

    #[cfg(target_os = "linux")]
    #[error("Missing prerequisites")]
    MissingPrerequisites,

    #[cfg(target_os = "linux")]
    #[error("Capabilities error")]
    CapsError(#[from] caps::errors::CapsError),

}
