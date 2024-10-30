use sea_orm::prelude::{Date, Time};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Availability {
    pub service_id: i32,
    pub date: Date,
    pub time_slots: Vec<Time>,
    pub open_time: Time,
    pub close_time: Time,
    pub lunch_from_time: Time,
    pub lunch_to_time: Time,
}

impl Default for Availability {
    fn default() -> Self {
        Self {
            service_id: Default::default(),
            date: Default::default(),
            time_slots: Default::default(),
            open_time: Default::default(),
            close_time: Default::default(),
            lunch_from_time: Default::default(),
            lunch_to_time: Default::default(),
        }
    }
}
