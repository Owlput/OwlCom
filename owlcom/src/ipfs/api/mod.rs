pub mod v0;

#[derive(Debug)]
pub enum FileTransferError {
    FileNotExist,
    NotAFile,
    Fs(std::io::Error),
    Reqwest(reqwest::Error),
}