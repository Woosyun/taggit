pub mod model;
pub use model::{Text, Item};

pub mod diff;
pub use diff::*;

use cfg_if::cfg_if;
cfg_if! {
    if #[cfg(feature="ssr")] {
        pub mod service;
        pub use service::*;
    }
}