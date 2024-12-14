pub mod model;
pub use model::{Text, Item, Commit};

pub mod view;
pub use view::*;

pub mod diff;
pub use diff::*;

use cfg_if::cfg_if;
cfg_if! {
    if #[cfg(feature="ssr")] {
        pub mod service;
        pub use service::*;
    }
}