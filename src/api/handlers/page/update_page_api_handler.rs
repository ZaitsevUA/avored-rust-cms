use std::sync::Arc;

use crate::error::Error;
use crate::models::page_model::{PageModel, UpdatableComponentContentModel, UpdatableComponentFieldContentModel, UpdatablePageComponentFieldDataModel, UpdatablePageModel};
use crate::models::validation_error::ErrorResponse;
use crate::{
    api::handlers::page::request::update_page_request::UpdatePageRequest,
    avored_state::AvoRedState, error::Result
};

use axum::{Extension, extract::{Path as AxumPath, State}, Json};
use serde::Serialize;
use crate::models::token_claim_model::LoggedInUser;

pub async fn update_page_api_handler(
    Extension(logged_in_user): Extension<LoggedInUser>,
    AxumPath(page_id): AxumPath<String>,
    state: State<Arc<AvoRedState>>,
    Json(payload): Json<UpdatePageRequest>,
) -> Result<Json<UpdatablePageResponse>> {
    println!("->> {:<12} - update_page_api_handler", "HANDLER");

    let has_permission_bool = state
        .admin_user_service
        .has_permission(logged_in_user.clone(), String::from("page_edit"))
        .await?;
    if !has_permission_bool {
        return Err(Error::FORBIDDEN);
    }

    let error_messages = payload.validate()?;

    if !error_messages.is_empty() {
        let error_response = ErrorResponse {
            status: false,
            errors: error_messages
        };

        return Err(Error::BadRequestError(error_response));
    }

    let mut updatable_page = UpdatablePageModel {
        id: page_id,
        name: payload.name,
        identifier: payload.identifier,
        component_contents: vec![],
        logged_in_username: logged_in_user.email.clone(),
    };


    for payload_component_content in payload.components_content {
        let mut  updatable_component_content_model = UpdatableComponentContentModel {
            id: payload_component_content.id,
            name: payload_component_content.name,
            identifier: payload_component_content.identifier,
            fields: vec![],
        };

        for  payload_component_fields_data in  payload_component_content.fields {
            let mut payload_field_data_model_options: Vec<UpdatablePageComponentFieldDataModel> = Vec::new();
            let  payload_field_options_data = payload_component_fields_data.field_data.unwrap_or(Vec::new());

            for payload_component_field_data_option in payload_field_options_data {
                let creatable_page_field_option_data = UpdatablePageComponentFieldDataModel {
                    label: payload_component_field_data_option.label,
                    value: payload_component_field_data_option.value,
                };
                payload_field_data_model_options.push(creatable_page_field_option_data);
            }

            let updatable_component_field_content = UpdatableComponentFieldContentModel {
                id: payload_component_fields_data.id,
                name: payload_component_fields_data.name,
                identifier: payload_component_fields_data.identifier,
                field_type: payload_component_fields_data.field_type,
                field_content: payload_component_fields_data.field_content,
                field_data: payload_field_data_model_options
            };

            updatable_component_content_model.fields.push(updatable_component_field_content);
        }

        updatable_page.component_contents.push(updatable_component_content_model);
    }



    let updated_page_model = state
        .page_service
        .update_page(&state.db, updatable_page, logged_in_user)
        .await?;
    let response = UpdatablePageResponse {
        status: true,
        page_model: updated_page_model
    };

    Ok(Json(response))
}


#[derive(Serialize, Debug)]
pub struct UpdatablePageResponse {
    pub status: bool,
    pub page_model: PageModel
}
