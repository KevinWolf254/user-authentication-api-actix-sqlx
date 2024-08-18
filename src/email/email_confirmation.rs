use askama::Template;

#[derive(Template)]
#[template(path = "email_confirmation_template.html")]
pub struct EmailConfirmationTemplate {
    pub code: i32,
    pub recipient: String,
}