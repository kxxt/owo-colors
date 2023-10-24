//! Color types for used for being generic over the color
use crate::{BgColorDisplay, BgDynColorDisplay, FgColorDisplay, FgDynColorDisplay};
use core::fmt;

macro_rules! colors {
    ($(
        $color:ident $fg:literal $bg:literal
    ),* $(,)?) => {

        pub(crate) mod ansi_colors {
            use core::fmt;

            #[allow(unused_imports)]
            use crate::OwoColorize;

            /// Available standard ANSI colors for use with [`OwoColorize::color`](OwoColorize::color)
            /// or [`OwoColorize::on_color`](OwoColorize::on_color)
            #[allow(missing_docs)]
            #[derive(Copy, Clone, Debug, PartialEq, Eq)]
            pub enum AnsiColors {
                $(
                    $color,
                )*
            }

            impl crate::DynColor for AnsiColors {
                fn fmt_ansi_fg(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    let color = match self {
                        $(
                            AnsiColors::$color => concat!("\x1b[", stringify!($fg), "m"),
                        )*
                    };

                    write!(f, "{}", color)
                }

                fn fmt_ansi_bg(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    let color = match self {
                        $(
                            AnsiColors::$color => concat!("\x1b[", stringify!($bg), "m"),
                        )*
                    };

                    write!(f, "{}", color)
                }

                fn fmt_raw_ansi_fg(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    let color = match self {
                        $(
                            AnsiColors::$color => stringify!($fg),
                        )*
                    };

                    f.write_str(color)
                }

                fn fmt_raw_ansi_bg(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    let color = match self {
                        $(
                            AnsiColors::$color => stringify!($bg),
                        )*
                    };

                    f.write_str(color)
                }

                #[doc(hidden)]
                fn get_dyncolors_fg(&self) -> crate::DynColors {
                    crate::DynColors::Ansi(*self)
                }

                #[doc(hidden)]
                fn get_dyncolors_bg(&self) -> crate::DynColors {
                    crate::DynColors::Ansi(*self)
                }
            }
        }

        $(
            /// A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods.
            pub struct $color;

            impl crate::Color for $color {
                const ANSI_FG: &'static str = concat!("\x1b[", stringify!($fg), "m");
                const ANSI_BG: &'static str = concat!("\x1b[", stringify!($bg), "m");

                const RAW_ANSI_FG: &'static str = stringify!($fg);
                const RAW_ANSI_BG: &'static str = stringify!($bg);

                #[doc(hidden)]
                type DynEquivalent = ansi_colors::AnsiColors;

                #[doc(hidden)]
                const DYN_EQUIVALENT: Self::DynEquivalent = ansi_colors::AnsiColors::$color;

                #[doc(hidden)]
                fn into_dyncolors() -> crate::DynColors {
                    crate::DynColors::Ansi(ansi_colors::AnsiColors::$color)
                }
            }
        )*

    };
}

colors! {
    Black   30 40,
    Red     31 41,
    Green   32 42,
    Yellow  33 43,
    Blue    34 44,
    Magenta 35 45,
    Cyan    36 46,
    White   37 47,
    Default   39 49,

    BrightBlack   90 100,
    BrightRed     91 101,
    BrightGreen   92 102,
    BrightYellow  93 103,
    BrightBlue    94 104,
    BrightMagenta 95 105,
    BrightCyan    96 106,
    BrightWhite   97 107,
}

macro_rules! impl_fmt_for {
    (@@impl_code_path, $f:ident, @@$phase:ident, $target:ident) => {
        cfg_if::cfg_if!{
            if #[cfg(feature = "global-colorized-control")]{
                if crate::control::should_colorize() {
                    impl_fmt_for!(@@$phase, $f, $target);
                }
            } else {
                impl_fmt_for!(@@$phase, $f, $target);
            }
        }
    };
    (@@before, $f:ident, fg) => {
        $f.write_str(Color::ANSI_FG)?;
    };
    (@@after, $f:ident, fg) => {
        $f.write_str("\x1b[39m")?;
    };
    (@@before, $f:ident, bg) => {
        $f.write_str(Color::ANSI_BG)?;
    };
    (@@after, $f:ident, bg) => {
        $f.write_str("\x1b[49m")?;
    };
    ($($trait:path),* $(,)?) => {
        $(
            impl<'a, Color: crate::Color, T: $trait> $trait for FgColorDisplay<'a, Color, T> {
                #[inline(always)]
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    impl_fmt_for!(@@impl_code_path, f, @@before, fg);
                    <T as $trait>::fmt(&self.0, f)?;
                    impl_fmt_for!(@@impl_code_path, f, @@after, fg);
                    Ok(())
                }
            }

            impl<'a, Color: crate::Color, T: $trait> $trait for BgColorDisplay<'a, Color, T> {
                #[inline(always)]
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    impl_fmt_for!(@@impl_code_path, f, @@before, bg);
                    <T as $trait>::fmt(&self.0, f)?;
                    impl_fmt_for!(@@impl_code_path, f, @@after, bg);
                    Ok(())
                }
            }
        )*
    };
}

impl_fmt_for! {
    fmt::Display,
    fmt::Debug,
    fmt::UpperHex,
    fmt::LowerHex,
    fmt::Binary,
    fmt::UpperExp,
    fmt::LowerExp,
    fmt::Octal,
    fmt::Pointer,
}

macro_rules! impl_fmt_for_dyn {
    (@@impl_code_path, $f:ident, $self:ident, @@$phase:ident, $target:ident) => {
        cfg_if::cfg_if!{
            if #[cfg(feature = "global-colorized-control")]{
                if crate::control::should_colorize() {
                    impl_fmt_for_dyn!(@@$phase, $f, $self, $target);
                }
            } else {
                impl_fmt_for_dyn!(@@$phase, $f, $self, $target);
            }
        }
    };
    (@@before, $f:ident,$self:ident, fg) => {
        ($self.1).fmt_ansi_fg($f)?;
    };
    (@@after, $f:ident,$self:ident, fg) => {
        $f.write_str("\x1b[39m")?;
    };
    (@@before, $f:ident, $self:ident, bg) => {
        ($self.1).fmt_ansi_bg($f)?;
    };
    (@@after, $f:ident, $self:ident, bg) => {
        $f.write_str("\x1b[49m")?;
    };
    ($($trait:path),* $(,)?) => {
        $(
            impl<'a, Color: crate::DynColor, T: $trait> $trait for FgDynColorDisplay<'a, Color, T> {
                #[inline(always)]
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    impl_fmt_for_dyn!(@@impl_code_path, f, self, @@before, fg);
                    <T as $trait>::fmt(&self.0, f)?;
                    impl_fmt_for_dyn!(@@impl_code_path, f, self, @@after, fg);
                    Ok(())
                }
            }

            impl<'a, Color: crate::DynColor, T: $trait> $trait for BgDynColorDisplay<'a, Color, T> {
                #[inline(always)]
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    impl_fmt_for_dyn!(@@impl_code_path, f, self, @@before, bg);
                    <T as $trait>::fmt(&self.0, f)?;
                    impl_fmt_for_dyn!(@@impl_code_path, f, self, @@after, bg);
                    Ok(())
                }
            }
        )*
    };
}

impl_fmt_for_dyn! {
    fmt::Display,
    fmt::Debug,
    fmt::UpperHex,
    fmt::LowerHex,
    fmt::Binary,
    fmt::UpperExp,
    fmt::LowerExp,
    fmt::Octal,
    fmt::Pointer,
}

/// CSS named colors. Not as widely supported as standard ANSI as it relies on 48bit color support.
///
/// Reference: <https://www.w3schools.com/cssref/css_colors.asp>
/// Reference: <https://developer.mozilla.org/en-US/docs/Web/CSS/color_value>
pub mod css;
/// XTerm 256-bit colors. Not as widely supported as standard ANSI but contains 240 more colors.
pub mod xterm;

mod custom;

pub use custom::CustomColor;

pub(crate) mod dynamic;
