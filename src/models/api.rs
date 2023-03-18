use super::OString;
use serde::de::DeserializeOwned;

/// Wrapper Struct around all
#[derive(Debug, Deserialize)]
#[repr(transparent)]
pub struct ApiResponse<T> {
    data: T,
}

impl<T> ApiResponse<T>
where
    T: DeserializeOwned,
{
    #[must_use]
    pub fn inner(self) -> T {
        self.data
    }
}

pub type TrackingsResponse = ApiResponse<Trackings>;

#[derive(Debug, Deserialize)]
#[repr(transparent)]
pub struct Trackings {
    trackings: Vec<TrackingData>,
}

impl Trackings {
    #[must_use]
    pub fn inner(self) -> Vec<TrackingData> {
        self.trackings
    }
}

#[derive(Debug, Deserialize)]
pub struct TrackingData {
    pub shipment: Shipment,
    pub events: Vec<Event>,
    // pub statistics: Statistics,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Shipment {
    // pub shipment_id: OString,
    // pub status_code: OString,
    // pub status_category: OString,
    // pub status_milestone: OString,
    // pub origin_country_code: OString,
    // pub destination_country_code: OString,
    // pub delivery: DeliveryDetails,
    // pub tracking_numbers: Vec<TrackingNumber>,
    pub recipient: Recipient,
}

// #[derive(Debug, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct DeliveryDetails {
//     pub estimated_delivery_date: OString,
//     pub service: OString,
//     pub signed_by: OString,
// }

// #[derive(Debug, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct TrackingNumber {
//     pub number: String,
// }

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Recipient {
    // pub name: OString,
    // pub address: OString,
    // pub post_code: OString,
    pub city: OString,
    // pub subdivision: OString,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    pub tracking_number: String,
    // pub event_tracking_number: String,
    pub status: String,
    // pub occurence_date_time: String,
    // pub order: Option<isize>,
    pub location: OString,
    // pub source_code: OString,
    pub courier_code: OString,
    // pub status_code: OString,
    // pub status_category: OString,
    // pub status_milestone: String,
    pub datetime: String,
    // pub utc_offset: String,
    // pub has_no_time: String,
}

// #[derive(Debug, Deserialize)]
// pub struct Statistics {
//     pub timestamps: Timestamps,
// }

// #[derive(Debug, Deserialize)]
// pub struct Timestamps {
//     pub in_transit_datetime: OString,
//     pub out_for_delivery_datetime: OString,
//     pub failed_attempt_datetime: OString,
//     pub available_for_pickup_datetime: OString,
//     pub exception_datetime: OString,
//     pub delivered_datetime: OString,
// }
