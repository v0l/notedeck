mod app;
//mod camera;
mod error;
//mod note;
//mod block;
mod abbrev;
pub mod accounts;
mod actionbar;
pub mod app_creation;
mod app_style;
mod args;
mod colors;
mod column;
mod deck_state;
mod decks;
mod draft;
mod frame_history;
mod images;
mod key_parsing;
pub mod login_manager;
mod multi_subscriber;
mod nav;
mod notes_holder;
mod post;
mod profile;
pub mod relay_pool_manager;
mod route;
mod subscriptions;
mod support;
mod test_data;
mod thread;
mod timeline;
pub mod ui;
mod unknowns;
mod view_state;

#[cfg(test)]
#[macro_use]
mod test_utils;

pub mod storage;

pub use app::Damus;
pub use error::Error;
pub use profile::DisplayName;

pub type Result<T> = std::result::Result<T, error::Error>;
