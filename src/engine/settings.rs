#[derive(Ord, PartialOrd, PartialEq, Eq)]
pub enum PerfLevel {
    VeryHighPerf,
    HighPerf,
    AveragePerf,
    HighDetails,
    VeryHighDetails,
}
pub const PERF_LEVEL: PerfLevel = PerfLevel::HighPerf;

macro_rules! perf_level {
    (
        $fst: expr
        $(
            => $level: ident
            $value: expr
        )*
    ) => {
        {
            let v = crate::engine::settings::PERF_LEVEL;
            let mut res = $fst;
            $(
                if v >= crate::engine::settings::PerfLevel::$level {
                    res = $value;
                }
            )*
            res
        }
    };
}
pub(crate) use perf_level;
