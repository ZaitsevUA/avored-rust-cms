use std::sync::Arc;

use argon2::{Argon2, PasswordHash, PasswordVerifier};
use avored_qs::AvoRedForm;
use axum::extract::State;
use axum::response::{IntoResponse, Redirect};
use validator::{HasLen, Validate, ValidationErrors, ValidationErrorsKind};

use crate::avored_state::AvoRedState;
use crate::models::admin_user_model::AdminUser;
use crate::providers::avored_session_provider::AvoRedSession;
use crate::requests::authenticate_admin_user_request::AuthenticateAdminUserRequest;

pub async fn authenticate_admin_user_handler(
    mut session: AvoRedSession,
    state: State<Arc<AvoRedState>>,
    AvoRedForm(payload): AvoRedForm<AuthenticateAdminUserRequest>,
) -> impl IntoResponse {
    let validation_error_list = match payload.validate() {
        Ok(_) => ValidationErrors::new(),
        Err(errors) => errors,
    };

    for (field_name, error) in validation_error_list.errors() {
        match &error {
            ValidationErrorsKind::Field(field_errors) => {
                for field_error in field_errors {
                    let message = match &field_error.message {
                        Some(message) => message,
                        None => continue,
                    };

                    if !message.is_empty() {
                        // let key = field_name.clone();
                        let validation_key = format!("validation_error_{}", field_name);
                        session
                            .insert(&validation_key, message)
                            .expect("Could not store the validation errors into session.");
                    }
                }
            }
            ValidationErrorsKind::Struct(_) => continue,
            ValidationErrorsKind::List(_) => continue,
        }
    }
    if validation_error_list.errors().length() > 0 {
        return Err(Redirect::to("/admin/login").into_response());
    }

    let admin_user_model = match state
        .admin_user_repository
        .find_by_email(&state.datastore, &state.database_session, payload.email)
        .await
    {
        Ok(data) => data,
        Err(_) => AdminUser::empty_admin_user(),
    };

    let is_valid = match PasswordHash::new(&admin_user_model.password) {
        Ok(parsed_hash) => Argon2::default()
            .verify_password(payload.password.as_bytes(), &parsed_hash)
            .map_or(false, |_| true),
        Err(_) => false,
    };

    if !is_valid {
        let validation_error = String::from("your email address or password did not match");

        session
            .insert("validation_error_email", validation_error)
            .expect("Could not store the validation errors into session.");

        return Err(Redirect::to("/admin/login").into_response());
    }

    let mut profile_image = String::from("https://via.placeholder.com/100x100");

    if admin_user_model.profile_image.len() > 0 {
        profile_image = format!("/public/{}", admin_user_model.profile_image);
    }
 
    let admin_user_model = AdminUser {
        id: admin_user_model.id,
        full_name: admin_user_model.full_name,
        email: admin_user_model.email,
        password: admin_user_model.password,
        profile_image: profile_image,
        is_super_admin: admin_user_model.is_super_admin,
        created_at: admin_user_model.created_at,
        updated_at: admin_user_model.updated_at,
        created_by: admin_user_model.created_by,
        updated_by: admin_user_model.updated_by,
    };

    session
        .insert("logged_in_user", admin_user_model)
        .expect("Could not store the answer.");

    Ok(Redirect::to("/admin").into_response())
}