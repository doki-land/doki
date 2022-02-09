use super::*;

macro_rules! error_wrap {
    ($t:ty => $name:ident) => {
        impl From<$t> for DokiError {
            fn from(e: $t) -> Self {
                Self { kind: Box::new(DokiErrorKind::$name(e)), level: DiagnosticLevel::None, file: None, range: None }
            }
        }
    };
    ($($t:ty => $name:ident),+ $(,)?) => (
        $(error_wrap!($t=>$name);)+
    );
}

error_wrap![
    std::io::Error  => IOError,
    std::fmt::Error => FormatError,
];

impl From<Infallible> for DokiError {
    fn from(_: Infallible) -> Self {
        Self::unreachable()
    }
}

impl From<()> for DokiError {
    fn from(_: ()) -> Self {
        Self::unreachable()
    }
}

impl From<ParseError> for DokiError {
    fn from(e: ParseError) -> Self {
        Self::syntax_error(e.to_string())
    }
}
