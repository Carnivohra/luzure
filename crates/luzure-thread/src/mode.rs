#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ThreadMode {
    Automatic,
    MainThread,
    Threaded,
}
