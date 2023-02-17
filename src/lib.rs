#[macro_export]
macro_rules! elvis {
    ($option:tt ?: $block_f:tt $(?: $block:tt)*) => {
        '__elvis: {
        let __elvis_ret = match $option {
            Some(v) => break '__elvis v,
            None => $block_f,
        };
        $(
        let __elvis_ret = match __elvis_ret {
            Some(v) => break '__elvis v,
            None => $block,
        };
        )*
        __elvis_ret
    }}
}