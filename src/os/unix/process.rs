use process::Command;

pub trait CommandExt {
    fn uid(&mut self, id: u32) -> &mut Command;
    fn gid(&mut self, id: u32) -> &mut Command;
}
