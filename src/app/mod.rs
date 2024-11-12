mod frontend;
mod pages;

pub use frontend::*;

use cfg_if::cfg_if;
cfg_if! {
    if #[cfg(feature="ssr")] {
        mod backend;
        pub use backend::*;
    }
}