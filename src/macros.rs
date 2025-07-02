#![allow(unused_macros)]

macro_rules! error {
    ($($target:ident:)? $fmt_str:literal $($fmt_args:tt)*) => {
        ::log::error!(target: stringify!($($target)?), $fmt_str $($fmt_args)*)
    };
}
macro_rules! warn {
    ($($target:ident:)? $fmt_str:literal $($fmt_args:tt)*) => {
        ::log::warn!(target: stringify!($($target)?), $fmt_str $($fmt_args)*)
    };
}
macro_rules! info {
    ($($target:ident:)? $fmt_str:literal $($fmt_args:tt)*) => {
        ::log::info!(target: stringify!($($target)?), $fmt_str $($fmt_args)*)
    };
}
macro_rules! debug {
    ($($target:ident:)? $fmt_str:literal $($fmt_args:tt)*) => {
        ::log::debug!(target: stringify!($($target)?), $fmt_str $($fmt_args)*)
    };
}
macro_rules! trace {
    ($($target:ident:)? $fmt_str:literal $($fmt_args:tt)*) => {
        ::log::trace!(target: stringify!($($target)?), $fmt_str $($fmt_args)*)
    };
}
