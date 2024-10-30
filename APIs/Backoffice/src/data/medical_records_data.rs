use actix_web::http;
use chrono::{Local, NaiveDateTime};
use models::entities::medical_records::{ActiveModel, Column, Entity as MedicalRecords, Model as MedicalRecordsModel};
use sea_orm::{ActiveModelTrait, ActiveValue, ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter};
use security::core::SessionCore;
use tracing::error_span;
use utils::{CodeMessage, Outcome};

pub struct MedicalRecordsData;

impl MedicalRecordsData {
    pub async fn insert_medical_record(
        db: &DatabaseConnection,
        medical_record_model: MedicalRecordsModel,
        session_core: SessionCore,
    ) -> Outcome<MedicalRecordsModel, CodeMessage, CodeMessage> {
        let current_date = Local::now();

        let medical_record = ActiveModel {
            pet_id: ActiveValue::Set(medical_record_model.pet_id),
            creation_date: ActiveValue::Set(Some(NaiveDateTime::new(
                current_date.date_naive(),
                current_date.time(),
            ))),
            comments: ActiveValue::Set(medical_record_model.comments),
            created_by_user_id: ActiveValue::Set(Some(session_core.user.user_id)),
            ..Default::default()
        };

        match medical_record.insert(db).await {
            Err(err) => {
                error_span!("error - database", error = ?err);

                return Outcome::Error(CodeMessage {
                    http_code: http::StatusCode::INTERNAL_SERVER_ERROR,
                    message: err.to_string(),
                });
            }
            Ok(val) => Outcome::Success(val),
        }
    }

    pub async fn select_medical_records(
        db: &DatabaseConnection,
        medical_record_model: MedicalRecordsModel
    ) -> Outcome<Vec<MedicalRecordsModel>, CodeMessage, CodeMessage> {
        let mut condition = Condition::all();

        if medical_record_model.medical_record_id != 0 {
            condition = condition.add(Column::MedicalRecordId.eq(medical_record_model.medical_record_id));
        }

        if medical_record_model.pet_id.is_some_and(|x| x != 0) {
            condition = condition.add(Column::PetId.eq(medical_record_model.pet_id));
        }

        if condition.len() > 0 {
            match MedicalRecords::find().filter(condition).all(db).await {
                Err(err) => {
                    error_span!("error - database", error = ?err);
                    return Outcome::Error(CodeMessage {
                        http_code: http::StatusCode::INTERNAL_SERVER_ERROR,
                        message: err.to_string(),
                    });
                }
                Ok(val) => {
                    if val.len() < 1 {
                        return Outcome::Failure(CodeMessage {
                            http_code: http::StatusCode::BAD_REQUEST,
                            message: String::from("Medical Records not found"),
                        });
                    }

                    return Outcome::Success(val);
                }
            }
        }

        Outcome::Failure(CodeMessage {
            http_code: http::StatusCode::BAD_REQUEST,
            message: String::from("No searching parameters"),
        })
    }
}
