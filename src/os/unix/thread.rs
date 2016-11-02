pub type RawPthread = usize;

pub trait JoinHandleExt {
    fn as_pthread_t(&self) -> RawPthread;
    fn into_pthread_t(self) -> RawPthread;
}
