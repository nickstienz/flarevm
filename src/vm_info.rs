#[derive(Debug)]
pub struct VMInfo {
    pub version: u32,
    pub experimental: bool,
    pub name: &'static str,
    pub authors: &'static str,
}
