pub trait MetadataExt {
    fn mode(&self) -> u32;
    fn uid(&self) -> u32;
    fn gid(&self) -> u32;
    fn size(&self) -> u64;
}
