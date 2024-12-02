use cfg_if::cfg_if;
cfg_if! {
    if #[cfg(feature="ssr")] {
        mod backend;
        pub use backend::*;
    }
}

mod frontend;
pub use frontend::*;

pub mod utils;
pub use utils::*;

pub mod pages;