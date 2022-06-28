mod dashboard;
mod logout;
mod newsletter;
mod password;

pub use dashboard::{admin_dashboard, get_username};
pub use logout::log_out;
pub use newsletter::{newsletter_form, publish_newsletter};
pub use password::*;
