use actix_web::http;
use models::entities::medical_records::Model as MedicalRecordsModel;
use security::core::SessionCore;
use utils::{get_config, CodeMessage, Outcome};

use crate::data::MedicalRecordsData;

pub struct MedicalRecordsCore;

impl MedicalRecordsCore {
    pub async fn insert_medical_record(
        mut medical_records_model: MedicalRecordsModel,
        session_core: SessionCore,
    ) -> Outcome<MedicalRecordsModel, CodeMessage, CodeMessage> {
        medical_records_model.created_by_user_id = Some(session_core.user.user_id);

        let parsed_medical_record = match Self::parse_medical_records(medical_records_model).await {
            Err(err) => {
                return Outcome::Failure(CodeMessage {
                    http_code: http::StatusCode::BAD_REQUEST,
                    message: err,
                })
            }
            Ok(val) => val,
        };

        MedicalRecordsData::insert_medical_record(
            &get_config().await.db_url,
            parsed_medical_record,
            session_core,
        )
        .await
    }

    pub async fn select_medical_records(
        medical_record_model: MedicalRecordsModel,
    ) -> Outcome<Vec<MedicalRecordsModel>, CodeMessage, CodeMessage> {
        MedicalRecordsData::select_medical_records(&get_config().await.db_url, medical_record_model)
            .await
    }
}

impl MedicalRecordsCore {
    async fn parse_medical_records(
        medical_records_model: MedicalRecordsModel,
    ) -> Result<MedicalRecordsModel, String> {
        let pet_id = match medical_records_model.pet_id {
            None => return Err(format!("Pet ID cannot be empty")),
            Some(val) => Some(val),
        };

        let comments = match medical_records_model.comments {
            None => return Err(format!("Comments cannot be empty")),
            Some(val) => Some(val),
        };

        let created_by_user_id = match medical_records_model.created_by_user_id {
            None => return Err(format!("Created By User ID cannot be empty")),
            Some(val) => Some(val),
        };

        Ok(MedicalRecordsModel {
            pet_id,
            comments,
            created_by_user_id,
            ..Default::default()
        })
    }
}
