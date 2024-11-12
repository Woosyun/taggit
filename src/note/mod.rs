mod model;

pub use model::Note;

use cfg_if::cfg_if;
cfg_if! {
    if #[cfg(feature="ssr")] {
        mod service;
        pub use service::NoteService;
    }
}