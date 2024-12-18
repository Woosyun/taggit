use cfg_if::cfg_if;

pub fn test_dbg(T) {
    cfg_if! {
        if #[cfg(test)] {
            dbg!(T);
        }
    }
}