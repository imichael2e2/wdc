macro_rules! dbgg {
    () => {
        #[cfg(feature = "dev")]
        dbg!();
    };
    ($val:expr $(,)?) => {
        #[cfg(feature = "dev")]
        dbg!($val);
    };
    ($($val:expr),+ $(,)?) => {
        #[cfg(feature = "dev")]
        ($(dbg!($val)),+);
    };
}
macro_rules! dbgmsg {
    ($fmtstr:expr) => {
        #[cfg(feature = "dev")]
        let dbgmsg = format!($fmtstr);
        #[cfg(feature = "dev")]
        dbg!(dbgmsg);
    };
    ($fmtstr:expr, $($val:expr),+ $(,)?) => {
        #[cfg(feature = "dev")]
        let dbgmsg = format!($fmtstr, $($val),+);
        #[cfg(feature = "dev")]
        dbg!(dbgmsg);
    };
}
macro_rules! run_diag {
    ($phase:expr, $blk:block) => {
        #[cfg(feature = "diag")]
        let start = std::time::Instant::now();

        $blk;

        #[cfg(feature = "diag")]
        let dura = start.elapsed();
        #[cfg(feature = "diag")]
        let diag_msg = format!("{}: {:?}", $phase, dura);
        #[cfg(feature = "diag")]
        dbg!(diag_msg);
    };
}
