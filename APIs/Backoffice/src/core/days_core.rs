use models::entities::days::Model as DaysModel;
use utils::{get_config, CodeMessage, Outcome};

use crate::data::DaysData;

pub struct DaysCore;

impl DaysCore {
    pub async fn select_day(
        mut days_model: DaysModel,
        find_all: bool,
    ) -> Outcome<Vec<DaysModel>, CodeMessage, CodeMessage> {
        if let Some(val) = days_model.name.as_ref() {
            match val.as_ref() {
                "Sun" => days_model.name = Some(format!("Sunday")),
                "Mon" => days_model.name = Some(format!("Monday")),
                "Tue" => days_model.name = Some(format!("Tuesday")),
                "Wed" => days_model.name = Some(format!("Wednesday")),
                "Thu" => days_model.name = Some(format!("Thursday")),
                "Fri" => days_model.name = Some(format!("Friday")),
                "Sat" => days_model.name = Some(format!("Saturday")),
                _ => (),
            }
        }

        DaysData::select_day(&get_config().await.db_url, days_model, find_all).await
    }
}
