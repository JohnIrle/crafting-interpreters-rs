pub mod chunk;
pub mod value;

pub fn grow_capacity(capacity: i32) -> i32 {
    match capacity {
        ..=7 => 8,
        _ => capacity * 2,
    }
}
