
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Ctrl-C handler error")]
    CtrlC(#[from] ctrlc::Error),

    #[error("Killed by Ctrl-C")]
    KilledByCtrlC,

    #[error("I/O error")]
    IOError(#[from] std::io::Error),

    #[cfg(target_os = "linux")]
    #[error("Missing prerequisites")]
    MissingPrerequisites,

}
