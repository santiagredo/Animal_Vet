use models::entities::pet_types::Model;
use utils::{CodeMessage, Outcome};

use crate::data::PetTypesData;

pub struct PetTypesCore;

impl PetTypesCore {
    pub async fn select_pet_types(
        pet_type_model: Model,
    ) -> Outcome<Vec<Model>, CodeMessage, CodeMessage> {
        PetTypesData::select_pet_types(pet_type_model).await
    }
}
