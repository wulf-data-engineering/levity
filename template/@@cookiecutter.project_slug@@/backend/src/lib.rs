pub mod shared;

rust_i18n::i18n!("locales", fallback = "en");

pub use shared::aws_config::{self, *};
pub use shared::cognito_user_pool_event::*;
pub use shared::http::{self, *};
pub use shared::protocols::{self, *};
pub use shared::dynamodb;
pub use shared::users;