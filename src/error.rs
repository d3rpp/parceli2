#[derive(Debug, Error)]
pub enum ParceliError {
    #[error("IO Error: {0:#?}")]
    IO(#[from] std::io::Error),

    #[error("HTTP Error: {0:#?}")]
    Http(#[from] reqwest::Error),

    #[error("Deserialising Error: {0:#?}")]
    Serde(#[from] serde_json::Error),

    #[error("No Data")]
    NoData,
}
