use cfg_if::cfg_if;
cfg_if! {
    if #[cfg(feature="ssr")] {
        mod backend;
        pub use backend::*;
    }
}

pub mod frontend;
pub use frontend::*;

pub mod utils;
pub use utils::*;

pub mod pages;