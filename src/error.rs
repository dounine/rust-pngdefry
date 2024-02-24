#[derive(Debug)]
pub enum PngdefryError {
    Msg(String),
}

unsafe impl Send for PngdefryError {}
unsafe impl Sync for PngdefryError {}

