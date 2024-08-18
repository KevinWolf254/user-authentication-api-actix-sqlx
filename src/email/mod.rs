pub mod email_confirmation;

pub struct EmailDetails<'a> {
    pub subject: &'a str,
    pub to: &'a str,
    pub from: &'a str,
}