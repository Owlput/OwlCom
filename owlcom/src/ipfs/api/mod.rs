use std::fmt::Display;

pub mod v0;

#[derive(Debug)]
pub enum FileTransferError {
    FileNotExist,
    NotAFile,
    Fs(std::io::Error),
    Reqwest(reqwest::Error),
}

impl Display for FileTransferError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileTransferError::FileNotExist => f.write_str("file not found"),
            FileTransferError::NotAFile => f.write_str("path specified is not a file"),
            FileTransferError::Fs(e) => f.write_str(&format!("file system error:{}", e)),
            FileTransferError::Reqwest(e) => f.write_str(&format!("reqwest error:{}", e)),
        }
    }
}

impl std::error::Error for FileTransferError {}