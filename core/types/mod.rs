pub mod value;
pub mod limits;
pub mod elem;
pub mod block;

const KIBIBYTE: usize = 1024;
pub const PAGE_SIZE: usize = KIBIBYTE * 64;
