mod model;
pub use model::*;

use cfg_if::cfg_if;
cfg_if! {
    if #[cfg(feature="ssr")] {
        mod service;
        pub use service::*;
    }
}