use std::sync::atomic::AtomicU8;

mod types;

pub const SLOT_HASH_ACCOUNT: &str = "SysvarS1otHashes111111111111111111111111111";

pub struct PluginInner {
    startup_status: AtomicU8,
}
