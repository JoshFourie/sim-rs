use crate::context::grid;
use std::fmt;

#[derive(Debug,Clone)]
pub enum Message { 
    Report(Report),
    // Other
}

#[derive(Clone)]
pub struct Report {
    kind: ReportKind,
    inner: Vec<grid::PointIndex>
}

impl PartialEq<ReportKind> for Report {
    fn eq(&self, rhs: &ReportKind) -> bool {
        &self.kind == rhs
    }
}

impl Report {
    pub fn new(kind: ReportKind, inner: Vec<grid::PointIndex>) -> Self {
        Self{inner,kind}
    }

    pub fn as_inner(&self) -> &Vec<grid::PointIndex> {
        &self.inner
    }

    pub fn is(&self, target: &ReportKind) -> bool {
        self == target
    }
}

impl fmt::Debug for Report {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?}: {:?}",
            self.kind, self.inner
        )
    }
}

#[derive(Debug,Clone,PartialEq)]
pub enum ReportKind {
    Allies,
    Enemies
}
