use actix_web::{ HttpResponse, web, get, post, put, delete };

use crate::model::user::User;

#[get("users/{user_id}")]
pub async fn get_user(path: web::Path<String>) -> HttpResponse {
    let user_id = path.into_inner();
    let response = format!("Get user with id: {}", user_id);
    HttpResponse::Ok().body(response)
}

#[post("/users")]
pub async fn create_user(user: web::Json<User>) -> HttpResponse {
    HttpResponse::Ok().body(format!("Create user resource {:?}", user.into_inner()))
}

#[put("/users/{user_id}")]
pub async fn update_user(path: web::Path<String>, user: web::Json<User>) -> HttpResponse {
    let user_id = path.into_inner();
    let response = format!("Update user with id: {}, result {:?}", user_id, user.into_inner());
    HttpResponse::Ok().body(response)
}

#[delete("/users/{user_id}")]
pub async fn delete_user(path: web::Path<String>) -> HttpResponse {
    let user_id = path.into_inner();
    let response = format!("Deleted user with id: {}", user_id);
    HttpResponse::Ok().body(response)
}

#[cfg(test)]
mod tests {
    use actix_web::{test, App};

    use super::*;

    #[actix_web::test]
    async fn test_get_user() {
        let app = test::init_service(App::new().service(get_user)).await;

        let req = test::TestRequest::get().uri("/users/1").to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_create_user() {
        // let app = test::init_service(App::new().service(create_user)).await;

        // let user = User{
        //     email_address: "kkanyi@gmail.com".to_string(),
        //     first_name: "kevin".to_string(),
        //     middle_name: "kanyi".to_string(),
        //     last_name: "mbugua".to_string(),
        // };

        // let req = test::TestRequest::post().uri("/users").set_json(user).to_request();
        // let resp = test::call_service(&app, req).await;
        // assert!(resp.status().is_success());
    }
}