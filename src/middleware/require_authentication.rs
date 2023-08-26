use axum::{
    http::Request,
    middleware::Next,
    response::{IntoResponse, Redirect, Response},
};

use crate::{providers::avored_session_provider::AvoRedSession, models::admin_user_model::AdminUser};


pub async fn require_authentication<T>(
    session: AvoRedSession,
    mut request: Request<T>,
    next: Next<T>,
) -> Result<Response, impl IntoResponse> {
    let decoded = session.get("logged_in_user");
    if decoded.is_none() {
        return Err(Redirect::to("/admin/login").into_response());
    }

    let token_data: AdminUser = decoded.unwrap();
    
    let user = AdminUser {
        id: token_data.id,
        full_name: token_data.full_name,
        email: token_data.email,
        password: token_data.password,
        profile_image: token_data.profile_image,
        is_super_admin: token_data.is_super_admin,
        created_at: token_data.created_at,
        updated_at: token_data.updated_at,
        created_by: token_data.created_by,
        updated_by: token_data.updated_by,
    };

    request.extensions_mut().insert(user);

    *request.uri_mut() = "/admin/login".parse().unwrap();

    Ok(next.run(request).await)
}