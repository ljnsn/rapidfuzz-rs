use crate::distance::common::ScoreAlignment;
use std::fmt::Debug;

#[derive(Default, Copy, Clone)]
pub struct NoScoreCutoff;
#[derive(Default, Copy, Clone)]
pub struct WithScoreCutoff<T>(pub T);

pub trait DistanceCutoff<T>
where
    T: Copy,
{
    type Output: Copy + Into<Option<T>> + PartialEq + Debug;

    fn cutoff(&self) -> Option<T>;
    fn score(&self, raw: T) -> Self::Output;
}

impl<T> DistanceCutoff<T> for NoScoreCutoff
where
    T: Copy + PartialEq + Debug,
{
    type Output = T;

    fn cutoff(&self) -> Option<T> {
        None
    }

    fn score(&self, raw: T) -> Self::Output {
        raw
    }
}

impl<T> DistanceCutoff<T> for WithScoreCutoff<T>
where
    T: Copy + PartialOrd + Debug,
{
    type Output = Option<T>;

    fn cutoff(&self) -> Option<T> {
        Some(self.0)
    }

    fn score(&self, raw: T) -> Self::Output {
        (raw <= self.0).then_some(raw)
    }
}

pub trait SimilarityCutoff<T>
where
    T: Copy,
{
    type Output: Copy + Into<Option<T>> + PartialEq + Debug;
    type AlignmentOutput: Copy + Into<Option<ScoreAlignment>> + PartialEq + Debug;

    fn cutoff(&self) -> Option<T>;
    fn score(&self, raw: T) -> Self::Output;
    fn alignment(&self, raw: Option<ScoreAlignment>) -> Self::AlignmentOutput;
}

impl<T> SimilarityCutoff<T> for NoScoreCutoff
where
    T: Copy + PartialEq + Debug,
{
    type Output = T;
    type AlignmentOutput = ScoreAlignment;

    fn cutoff(&self) -> Option<T> {
        None
    }

    fn score(&self, raw: T) -> Self::Output {
        raw
    }

    fn alignment(&self, raw: Option<ScoreAlignment>) -> Self::AlignmentOutput {
        raw.unwrap()
    }
}

impl<T> SimilarityCutoff<T> for WithScoreCutoff<T>
where
    T: Copy + PartialOrd + Debug,
{
    type Output = Option<T>;
    type AlignmentOutput = Option<ScoreAlignment>;

    fn cutoff(&self) -> Option<T> {
        Some(self.0)
    }

    fn score(&self, raw: T) -> Self::Output {
        (raw >= self.0).then_some(raw)
    }

    fn alignment(&self, raw: Option<ScoreAlignment>) -> Self::AlignmentOutput {
        raw
    }
}
