use std::fmt::Display;

use chrono::{DateTime, Utc};
use console::style;

use crate::ARGS;

use super::api::TrackingsResponse;
use super::{api, OString};

#[derive(Debug, Clone)]
pub struct Parcel {
    pub tracking_number: String,
    pub courier_code: OString,
    pub city: OString,
    pub location: OString,
    pub events: Vec<Event>,
}

impl From<TrackingsResponse> for Vec<Parcel> {
    fn from(value: TrackingsResponse) -> Self {
        let unwrapped_value = value.inner().inner();

        let mut vector = vec![];

        for data in unwrapped_value {
            vector.push(Parcel {
                tracking_number: data.events[0].tracking_number.clone(),
                courier_code: data.events[0].courier_code.clone(),
                city: data.shipment.recipient.city,
                location: data.events[0].location.clone(),
                events: {
                    let mut events = vec![];

                    for i in data.events {
                        events.push(i.into())
                    }

                    events
                },
            })
        }

        vector
    }
}

impl Display for Parcel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{} {}",
            style("Package").green().bold(),
            style(self.tracking_number.as_str()).underlined()
        )?;

        if self.courier_code.is_some() {
            writeln!(
                f,
                "\t{} {}",
                style("Courier").underlined().bold(),
                self.courier_code
            )?;
        }

        if self.location.is_some() {
            writeln!(
                f,
                "\t{} {}",
                style("Location").underlined().bold(),
                self.location
            )?;
        } else {
            writeln!(
                f,
                "\t{} {}",
                style("Location").underlined().bold(),
                self.city
            )?;
        }

        if self.events.len() > 0 {
            writeln!(
                f,
                "\t{}   {}",
                style("Status").underlined().bold(),
                self.events[0].status
            )?;
        }

        if ARGS.list {
            for i in &self.events {
                writeln!(f, "{}", i)?;
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Event {
    pub status: String,
    pub location: OString,
    pub datetime: String,
}

impl From<api::Event> for Event {
    fn from(value: api::Event) -> Self {
        Event {
            status: value.status,
            location: value.location,
            datetime: value.datetime,
        }
    }
}

impl Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let options = textwrap::Options::new(50).subsequent_indent("\t\t\t\t\t   ");

            writeln!(
                f,
                "\t\t {} | {}: {}",
                self.datetime.parse::<DateTime<Utc>>().unwrap(),
                self.location,
                textwrap::fill(self.status.as_str(), options)
            )
    }
}
