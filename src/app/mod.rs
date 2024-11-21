mod frontend;
mod pages;

pub use frontend::*;
pub use pages::*;

use cfg_if::cfg_if;
cfg_if! {
    if #[cfg(feature="ssr")] {
        mod backend;
        pub use backend::*;
    }
}