use models::entities::services::Model as ServiceModel;
use utils::{get_config, CodeMessage, Outcome};

use crate::data::ServicesData;

pub struct ServicesCore;

impl ServicesCore {
    pub async fn select_services(
        service_model: ServiceModel,
    ) -> Outcome<Vec<ServiceModel>, CodeMessage, CodeMessage> {
        ServicesData::select_services(&get_config().await.db_url, service_model).await
    }
}
