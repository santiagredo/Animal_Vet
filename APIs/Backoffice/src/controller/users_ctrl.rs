use crate::core::UsersCore;
use actix_session::Session;
use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use models::entities::users::Model;
use security::core::{SessionCore, UserRolesCore};
use utils::{Outcome, CREATE_PERMISSION, DELETE_PERMISSION, READ_PERMISSION, UPDATE_PERMISSION};

#[post("")]
pub async fn insert_user(json: web::Json<Model>, mut session: Session) -> impl Responder {
    // User must be logged in
    let session_core = match SessionCore::session_validator(&mut session).await {
        Outcome::Error(err) => return HttpResponse::build(err.http_code).json(err.message),
        Outcome::Failure(fail) => return HttpResponse::build(fail.http_code).json(fail.message),
        Outcome::Success(session) => {
            match session.role.is_backoffice_enabled {
                Some(true) => (),
                _ => {
                    return HttpResponse::Unauthorized()
                        .json(format!("User is not authorized to use this endpoint"))
                }
            }

            // User must be assigned to a user role that can create users
            let user_permissions = match session.role.user_permissions {
                None => return HttpResponse::Unauthorized().json("User doesn't have permissions"),
                Some(val) => val,
            };

            if !UserRolesCore::has_permission(user_permissions, CREATE_PERMISSION) {
                return HttpResponse::Unauthorized().json("User can't create new users");
            }

            // New user can't have a lower hierarchy than the user who is creating it
            let user_hierarchy_level = match session.role.hierarchy_level {
                None => {
                    return HttpResponse::Unauthorized().json("User doesn't have hierarchy level")
                }
                Some(val) => val,
            };

            let parsed_user_role = match json.role {
                None => return HttpResponse::BadRequest().json("Missing new user role id"),
                Some(val) => val,
            };

            match UserRolesCore::select_role(models::entities::user_roles::Model {
                user_role_id: parsed_user_role,
                ..Default::default()
            })
            .await
            {
                Outcome::Error(err) => return HttpResponse::build(err.http_code).json(err.message),
                Outcome::Failure(fail) => {
                    return HttpResponse::build(fail.http_code).json(fail.message)
                }
                Outcome::Success(role) => {
                    if role.hierarchy_level.is_none() {
                        return HttpResponse::InternalServerError()
                            .json("Failed to retrieve new user's hierarchy level");
                    };

                    if role
                        .hierarchy_level
                        .is_some_and(|level| level < user_hierarchy_level)
                    {
                        return HttpResponse::BadRequest().json("New user can't have a lower hierarchy than the user who is creating it");
                    }
                }
            };

            session
        }
    };

    match UsersCore::insert_user(json.0, session_core).await {
        Outcome::Error(err) => HttpResponse::build(err.http_code).json(err.message),
        Outcome::Failure(fail) => HttpResponse::build(fail.http_code).json(fail.message),
        Outcome::Success(val) => HttpResponse::Ok().json(val),
    }
}

#[get("")]
pub async fn select_user(query: web::Query<Model>, mut session: Session) -> impl Responder {
    match SessionCore::session_validator(&mut session).await {
        Outcome::Error(err) => return HttpResponse::build(err.http_code).json(err.message),
        Outcome::Failure(fail) => return HttpResponse::build(fail.http_code).json(fail.message),
        Outcome::Success(session) => {
            match session.role.is_backoffice_enabled {
                Some(true) => (),
                _ => {
                    return HttpResponse::Unauthorized()
                        .json(format!("User is not authorized to use this endpoint"))
                }
            }

            // User must be assigned to a user role that can read users or must be reading its own profile
            let user_permissions = match session.role.user_permissions {
                None => return HttpResponse::Unauthorized().json("User doesn't have permissions"),
                Some(val) => val,
            };

            if !UserRolesCore::has_permission(user_permissions, READ_PERMISSION)
                && session.user.user_id != query.user_id
            {
                return HttpResponse::Unauthorized().json("User can't read other users");
            }

            session
        }
    };

    match UsersCore::select_user(query.0).await {
        Outcome::Error(err) => HttpResponse::build(err.http_code).json(err.message),
        Outcome::Failure(fail) => HttpResponse::build(fail.http_code).json(fail.message),
        Outcome::Success(val) => HttpResponse::Ok().json(val),
    }
}

#[patch("")]
pub async fn update_user(json: web::Json<Model>, mut session: Session) -> impl Responder {
    let session_core = match SessionCore::session_validator(&mut session).await {
        Outcome::Error(err) => return HttpResponse::build(err.http_code).json(err.message),
        Outcome::Failure(fail) => return HttpResponse::build(fail.http_code).json(fail.message),
        Outcome::Success(session) => {
            match session.role.is_backoffice_enabled {
                Some(true) => (),
                _ => {
                    return HttpResponse::Unauthorized()
                        .json(format!("User is not authorized to use this endpoint"))
                }
            }

            // User must be assigned to a user role that can edit users or must be editing its own profile
            let user_permissions = match session.role.user_permissions {
                None => return HttpResponse::Unauthorized().json("User doesn't have permissions"),
                Some(val) => val,
            };

            if !UserRolesCore::has_permission(user_permissions, UPDATE_PERMISSION)
                && session.user.user_id != json.user_id
            {
                return HttpResponse::Unauthorized().json("User can't edit other users");
            }

            // Hierarchy can't be lower than the user sending the update
            let user_hierarchy_level = match session.role.hierarchy_level {
                None => {
                    return HttpResponse::Unauthorized().json("User doesn't have hierarchy level")
                }
                Some(val) => val,
            };

            let parsed_user_role = match json.role {
                None => return HttpResponse::BadRequest().json("Missing user role id"),
                Some(val) => val,
            };

            match UserRolesCore::select_role(models::entities::user_roles::Model {
                user_role_id: parsed_user_role,
                ..Default::default()
            })
            .await
            {
                Outcome::Error(err) => return HttpResponse::build(err.http_code).json(err.message),
                Outcome::Failure(fail) => {
                    return HttpResponse::build(fail.http_code).json(fail.message)
                }
                Outcome::Success(role) => {
                    if role.hierarchy_level.is_none() {
                        return HttpResponse::InternalServerError()
                            .json("Failed to retrieve user's hierarchy level");
                    };

                    if role
                        .hierarchy_level
                        .is_some_and(|level| level <= user_hierarchy_level)
                    {
                        return HttpResponse::BadRequest().json(
                            "User can't have a lower hierarchy than the user who is updating it",
                        );
                    }
                }
            };

            session
        }
    };

    match UsersCore::update_user(json.0, session_core).await {
        Outcome::Error(err) => HttpResponse::build(err.http_code).json(err.message),
        Outcome::Failure(fail) => HttpResponse::build(fail.http_code).json(fail.message),
        Outcome::Success(val) => HttpResponse::Ok().json(val),
    }
}

#[delete("")]
pub async fn delete_user(json: web::Json<Model>, mut session: Session) -> impl Responder {
    let session_core = match SessionCore::session_validator(&mut session).await {
        Outcome::Error(err) => return HttpResponse::build(err.http_code).json(err.message),
        Outcome::Failure(fail) => return HttpResponse::build(fail.http_code).json(fail.message),
        Outcome::Success(session) => {
            match session.role.is_backoffice_enabled {
                Some(true) => (),
                _ => {
                    return HttpResponse::Unauthorized()
                        .json(format!("User is not authorized to use this endpoint"))
                }
            }

            // User must be assigned to a user role that can edit users or must be deleting its own profile
            let user_permissions = match session.role.user_permissions {
                None => return HttpResponse::Unauthorized().json("User doesn't have permissions"),
                Some(val) => val,
            };

            if !UserRolesCore::has_permission(user_permissions, DELETE_PERMISSION)
                && session.user.user_id != json.user_id
            {
                return HttpResponse::Unauthorized().json("User can't delete other users");
            }

            session
        }
    };

    match UsersCore::delete_user(json.0, session_core).await {
        Outcome::Error(err) => HttpResponse::build(err.http_code).json(err.message),
        Outcome::Failure(fail) => HttpResponse::build(fail.http_code).json(fail.message),
        Outcome::Success(val) => HttpResponse::Ok().json(val),
    }
}
