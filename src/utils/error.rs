use crate::prelude::*;
use std::fmt::Debug;

#[derive(Clone)]
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
impl Debug for AnyReport {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.debug)
    }
}

#[allow(clippy::absolute_paths)]
impl Display for AnyReport {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.debug)
    }
}
