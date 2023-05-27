/// Resize trait for resizing data structures that rely on a fixed var/lit count
pub trait Resize {
    fn resize(&mut self, var_count: usize);
}
