#[cfg(all(feature = "color", not(target_os = "windows")))]
use ansi_term::ANSIString;

#[cfg(all(feature = "color", not(target_os = "windows")))]
use ansi_term::Colour::{Green, Red, Yellow};

use std::env;
use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq)]
#[doc(hidden)]
pub enum ColorWhen {
    Auto,
    Always,
    Never,
}

#[cfg(feature = "color")]
pub(crate) fn is_a_tty(stderr: bool) -> bool {
    debugln!("is_a_tty: stderr={:?}", stderr);
    let stream = if stderr {
        atty::Stream::Stderr
    } else {
        atty::Stream::Stdout
    };
    atty::is(stream)
}

#[cfg(not(feature = "color"))]
pub(crate) fn is_a_tty(_: bool) -> bool {
    debugln!("is_a_tty;");
    false
}

pub(crate) fn is_term_dumb() -> bool {
    env::var("TERM").ok() == Some(String::from("dumb"))
}

pub(crate) struct ColorizerOption {
    pub(crate) use_stderr: bool,
    pub(crate) when: ColorWhen,
}

pub(crate) struct Colorizer {
    when: ColorWhen,
}

macro_rules! color {
    ($_self:ident, $c:ident, $m:expr) => {
        match $_self.when {
            ColorWhen::Auto => Format::$c($m),
            ColorWhen::Always => Format::$c($m),
            ColorWhen::Never => Format::None($m),
        }
    };
}

impl Colorizer {
    pub(crate) fn new(option: &ColorizerOption) -> Colorizer {
        let is_a_tty = is_a_tty(option.use_stderr);
        let is_term_dumb = is_term_dumb();
        Colorizer {
            when: if is_a_tty && !is_term_dumb {
                option.when
            } else {
                ColorWhen::Never
            },
        }
    }

    pub(crate) fn good<T>(&self, msg: T) -> Format<T>
    where
        T: fmt::Display + AsRef<str>,
    {
        debugln!("Colorizer::good;");
        color!(self, Good, msg)
    }

    pub(crate) fn warning<T>(&self, msg: T) -> Format<T>
    where
        T: fmt::Display + AsRef<str>,
    {
        debugln!("Colorizer::warning;");
        color!(self, Warning, msg)
    }

    pub(crate) fn error<T>(&self, msg: T) -> Format<T>
    where
        T: fmt::Display + AsRef<str>,
    {
        debugln!("Colorizer::error;");
        color!(self, Error, msg)
    }

    pub(crate) fn none<T>(&self, msg: T) -> Format<T>
    where
        T: fmt::Display + AsRef<str>,
    {
        debugln!("Colorizer::none;");
        Format::None(msg)
    }
}

impl Default for Colorizer {
    fn default() -> Self {
        Colorizer::new(&ColorizerOption {
            use_stderr: true,
            when: ColorWhen::Auto,
        })
    }
}

/// Defines styles for different types of error messages. Defaults to Error=Red, Warning=Yellow,
/// and Good=Green
#[derive(Debug)]
pub(crate) enum Format<T> {
    /// Defines the style used for errors, defaults to Red
    Error(T),
    /// Defines the style used for warnings, defaults to Yellow
    Warning(T),
    /// Defines the style used for good values, defaults to Green
    Good(T),
    /// Defines no formatting style
    None(T),
}

#[cfg(all(feature = "color", not(target_os = "windows")))]
impl<T: AsRef<str>> Format<T> {
    fn format(&self) -> ANSIString {
        match *self {
            Format::Error(ref e) => Red.bold().paint(e.as_ref()),
            Format::Warning(ref e) => Yellow.paint(e.as_ref()),
            Format::Good(ref e) => Green.paint(e.as_ref()),
            Format::None(ref e) => ANSIString::from(e.as_ref()),
        }
    }
}

#[cfg(any(not(feature = "color"), target_os = "windows"))]
impl<T: fmt::Display> Format<T> {
    fn format(&self) -> &T {
        match *self {
            Format::Error(ref e) => e,
            Format::Warning(ref e) => e,
            Format::Good(ref e) => e,
            Format::None(ref e) => e,
        }
    }
}

#[cfg(all(feature = "color", not(target_os = "windows")))]
impl<T: AsRef<str>> fmt::Display for Format<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.format())
    }
}

#[cfg(any(not(feature = "color"), target_os = "windows"))]
impl<T: fmt::Display> fmt::Display for Format<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.format())
    }
}

#[cfg(all(test, feature = "color", not(target_os = "windows")))]
mod test {
    use super::Format;
    use ansi_term::ANSIString;
    use ansi_term::Colour::{Green, Red, Yellow};

    #[test]
    fn colored_output() {
        let err = Format::Error("error");
        assert_eq!(
            &*format!("{}", err),
            &*format!("{}", Red.bold().paint("error"))
        );
        let good = Format::Good("good");
        assert_eq!(&*format!("{}", good), &*format!("{}", Green.paint("good")));
        let warn = Format::Warning("warn");
        assert_eq!(&*format!("{}", warn), &*format!("{}", Yellow.paint("warn")));
        let none = Format::None("none");
        assert_eq!(
            &*format!("{}", none),
            &*format!("{}", ANSIString::from("none"))
        );
    }
}
