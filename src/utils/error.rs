use crate::prelude::*;

pub struct AnyReport {
    debug: String,
}

#[allow(clippy::absolute_paths)]
impl<T: std::error::Error> From<Report<T>> for AnyReport {
    fn from(report: Report<T>) -> Self {
        Self {
            debug: format!("{report:?}"),
        }
    }
}

#[allow(clippy::absolute_paths)]
impl Display for AnyReport {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.debug)
    }
}
