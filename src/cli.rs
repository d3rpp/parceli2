use super::CONFIG;
use console::style;

#[derive(Debug, Parser)]
#[command(author = "d3rpp & freehelpdesk")]
#[command(
    about = "CLI implementation of a package/mail tracker for USPS/UPS/FedEx made in Rust using the parcel library and a bit of smarts."
)]
#[command(version, long_about = None)]
pub struct CliArgs {
    /// Parcel IDs
    #[arg(last = true)]
    parcel_ids: Vec<String>,

    /// Ship24 API Key
    ///
    /// ---
    ///
    /// You can also put this inside ~/.parceli.toml and it'll be fetched automatically.
    ///
    /// ```toml
    /// # ~/.parceli.toml
    /// api_key = "..."
    /// ```
    #[arg(short, long, env = "API_KEY")]
    api_key: Option<String>,

    /// List the tracking history
    #[arg(short, long)]
    list: bool,
}

#[derive(Debug)]
pub struct Args {
    pub parcel_ids: Vec<String>,
    pub api_key: String,
    pub list: bool,
}

impl From<CliArgs> for Args {
    fn from(value: CliArgs) -> Self {
        let parcel_ids = if value.parcel_ids.is_empty() {
            eprintln!(
                "[{}] - Please specify one or more Tracking IDs",
                style("ERROR").red()
            );

            std::process::exit(1);
        } else {
            value.parcel_ids
        };

        let api_key = match value.api_key {
            Some(key) => key,
            None => match &CONFIG.api_key {
                Some(api_key) => api_key.clone(),
                None => {
                    eprintln!(
						"[{}] - Unable to Get API Key, either add it in ~/.parceli.toml, set it as the API_KEY environment variable, or use the --api-key argument",
						style("ERROR").red()
					);

                    std::process::exit(2);
                }
            },
        };

        Args {
            parcel_ids,
            api_key,
            list: value.list,
        }
    }
}
