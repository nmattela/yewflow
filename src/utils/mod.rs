use std::cell::Cell;

pub type Position = (i32, i32);

pub fn nonce() -> u32 {
    thread_local! {
        static NONCE: Cell<u32> = Default::default();
    }

    NONCE
        .try_with(|nonce| {
            nonce.set(nonce.get().wrapping_add(1));
            nonce.get()
        })
        .expect("NONCE thread local key init failed")
}