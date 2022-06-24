use crate::helpers::{assert_is_redirect_to, spawn_app};

#[tokio::test]
async fn an_error_flash_message_is_set_on_failure() {
    // Arrange
    let app = spawn_app().await;

    // Try to login
    let login_body = serde_json::json!({
    "username": "random-username",
    "password": "random-password"
    });
    let response = app.post_login(&login_body).await;

    // Assert
    assert_is_redirect_to(&response, "/login");

    // Follow the redirect
    let html_page = app.get_login_html().await;
    assert!(html_page.contains(r#"<p><i>Authentication failed</i></p>"#));

    // Reload the login page
    let html_page = app.get_login_html().await;
    assert!(!html_page.contains(r#"<p><i>Authentication failed</i></p>"#));
}

#[tokio::test]
async fn redirect_to_admin_dashboard_after_login_success() {
    // Arrange
    let app = spawn_app().await;
    // Act
    let login_body = serde_json::json!({
    "username": &app.test_user.username,
    "password": &app.test_user.password
    });
    let response = app.post_login(&login_body).await;
    assert_is_redirect_to(&response, "/admin/dashboard");
    // Act
    let html_page = app.get_admin_dashboard().await;
    assert!(html_page.contains(&format!("Welcome {}", app.test_user.username)));
}
