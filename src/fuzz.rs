use crate::common::{NoScoreCutoff, SimilarityCutoff, WithScoreCutoff};
use crate::details::distance::MetricUsize;
use crate::distance::common::ScoreAlignment;
use crate::distance::indel;
use crate::HashableChar;
use std::collections::HashSet;

#[must_use]
#[derive(Clone, Copy, Debug)]
pub struct Args<ResultType, CutoffType> {
    score_cutoff: CutoffType,
    score_hint: Option<ResultType>,
}

impl<ResultType> Default for Args<ResultType, NoScoreCutoff> {
    fn default() -> Args<ResultType, NoScoreCutoff> {
        Args {
            score_cutoff: NoScoreCutoff,
            score_hint: None,
        }
    }
}

impl<ResultType, CutoffType> Args<ResultType, CutoffType> {
    pub fn score_hint(mut self, score_hint: ResultType) -> Self {
        self.score_hint = Some(score_hint);
        self
    }

    pub fn score_cutoff(
        self,
        score_cutoff: ResultType,
    ) -> Args<ResultType, WithScoreCutoff<ResultType>> {
        Args {
            score_hint: self.score_hint,
            score_cutoff: WithScoreCutoff(score_cutoff),
        }
    }
}

/// Returns a simple ratio between two strings or `None` if `ratio < score_cutoff`
///
/// # Example
/// ```
/// use rapidfuzz::fuzz;
/// /// score is 0.9655
/// let score = fuzz::ratio("this is a test".chars(), "this is a test!".chars());
/// ```
///
pub fn ratio<Iter1, Iter2>(s1: Iter1, s2: Iter2) -> f64
where
    Iter1: IntoIterator,
    Iter1::IntoIter: DoubleEndedIterator + Clone,
    Iter2: IntoIterator,
    Iter2::IntoIter: DoubleEndedIterator + Clone,
    Iter1::Item: PartialEq<Iter2::Item> + HashableChar + Copy,
    Iter2::Item: PartialEq<Iter1::Item> + HashableChar + Copy,
{
    ratio_with_args(s1, s2, &Args::default())
}

pub fn ratio_with_args<Iter1, Iter2, CutoffType>(
    s1: Iter1,
    s2: Iter2,
    args: &Args<f64, CutoffType>,
) -> CutoffType::Output
where
    Iter1: IntoIterator,
    Iter1::IntoIter: DoubleEndedIterator + Clone,
    Iter2: IntoIterator,
    Iter2::IntoIter: DoubleEndedIterator + Clone,
    Iter1::Item: PartialEq<Iter2::Item> + HashableChar + Copy,
    Iter2::Item: PartialEq<Iter1::Item> + HashableChar + Copy,
    CutoffType: SimilarityCutoff<f64>,
{
    let s1_iter = s1.into_iter();
    let s2_iter = s2.into_iter();
    args.score_cutoff
        .score(indel::IndividualComparator {}._normalized_similarity(
            s1_iter.clone(),
            s1_iter.count(),
            s2_iter.clone(),
            s2_iter.count(),
            args.score_cutoff.cutoff(),
            args.score_hint,
        ))
}

/// `One x Many` comparisons using `ratio`
///
/// # Examples
///
/// ```
/// use rapidfuzz::fuzz;
///
/// let scorer = fuzz::RatioBatchComparator::new("this is a test".chars());
/// /// score is 0.9655
/// let score = scorer.similarity("this is a test!".chars());
/// ```
pub struct RatioBatchComparator<Elem1> {
    scorer: indel::BatchComparator<Elem1>,
}

impl<Elem1> RatioBatchComparator<Elem1>
where
    Elem1: HashableChar + Clone,
{
    pub fn new<Iter1>(s1: Iter1) -> Self
    where
        Iter1: IntoIterator<Item = Elem1>,
        Iter1::IntoIter: Clone,
    {
        Self {
            scorer: indel::BatchComparator::new(s1),
        }
    }

    /// Similarity calculated similar to [`ratio`]
    pub fn similarity<Iter2>(&self, s2: Iter2) -> f64
    where
        Iter2: IntoIterator,
        Iter2::IntoIter: DoubleEndedIterator + Clone,
        Elem1: PartialEq<Iter2::Item> + HashableChar + Copy,
        Iter2::Item: PartialEq<Elem1> + HashableChar + Copy,
    {
        self.similarity_with_args(s2, &Args::default())
    }

    pub fn similarity_with_args<Iter2, CutoffType>(
        &self,
        s2: Iter2,
        args: &Args<f64, CutoffType>,
    ) -> CutoffType::Output
    where
        Iter2: IntoIterator,
        Iter2::IntoIter: DoubleEndedIterator + Clone,
        Elem1: PartialEq<Iter2::Item> + HashableChar + Copy,
        Iter2::Item: PartialEq<Elem1> + HashableChar + Copy,
        CutoffType: SimilarityCutoff<f64>,
    {
        let s2_iter = s2.into_iter();
        args.score_cutoff
            .score(self.scorer.scorer._normalized_similarity(
                self.scorer.scorer.s1.iter().copied(),
                self.scorer.scorer.s1.len(),
                s2_iter.clone(),
                s2_iter.count(),
                args.score_cutoff.cutoff(),
                args.score_hint,
            ))
    }
}

/// Searches for the optimal alignment of the shorter string in the
/// longer string and returns the fuzz.ratio for this alignment.
///
/// # Example
/// ```
/// use rapidfuzz::fuzz;
/// /// score is 100.0
/// let score = fuzz::partial_ratio("this is a test".chars(), "this is a test!".chars());
/// ```
///
pub fn partial_ratio<Iter1, Iter2>(s1: Iter1, s2: Iter2) -> f64
where
    Iter1: IntoIterator,
    Iter1::IntoIter: DoubleEndedIterator + Clone,
    Iter2: IntoIterator,
    Iter2::IntoIter: DoubleEndedIterator + Clone,
    Iter1::Item: PartialEq<Iter2::Item> + HashableChar + Copy,
    Iter2::Item: PartialEq<Iter1::Item> + HashableChar + Copy,
{
    ratio_with_args(s1, s2, &Args::default())
}

pub fn partial_ratio_with_args<Iter1, Iter2, CutoffType>(
    s1: Iter1,
    s2: Iter2,
    args: &Args<f64, CutoffType>,
) -> CutoffType::Output
where
    Iter1: IntoIterator,
    Iter1::IntoIter: DoubleEndedIterator + Clone,
    Iter2: IntoIterator,
    Iter2::IntoIter: DoubleEndedIterator + Clone,
    Iter1::Item: PartialEq<Iter2::Item> + HashableChar + Copy,
    Iter2::Item: PartialEq<Iter1::Item> + HashableChar + Copy,
    CutoffType: SimilarityCutoff<f64, Output = f64>,
{
    let s1_iter = s1.into_iter();
    let s2_iter = s2.into_iter();

    let alignment = partial_ratio_alignment(
        s1_iter.clone(),
        s1_iter.count(),
        s2_iter.clone(),
        s2_iter.count(),
        args,
    );

    match alignment {
        Some(alignment) => alignment.score,
        None => 0.0,
    }
}

pub fn partial_ratio_alignment<Iter1, Iter2, CutoffType>(
    s1: Iter1,
    len1: usize,
    s2: Iter2,
    len2: usize,
    args: &Args<f64, CutoffType>,
) -> Option<ScoreAlignment>
where
    Iter1: IntoIterator,
    Iter1::IntoIter: DoubleEndedIterator + Clone,
    Iter2: IntoIterator,
    Iter2::IntoIter: DoubleEndedIterator + Clone,
    Iter1::Item: PartialEq<Iter2::Item> + HashableChar + Copy,
    Iter2::Item: PartialEq<Iter1::Item> + HashableChar + Copy,
    CutoffType: SimilarityCutoff<f64, Output = f64>,
{
    let s1_iter = s1.into_iter();
    let s2_iter = s2.into_iter();

    let mut score_cutoff = args.score_cutoff.cutoff().unwrap_or(0.0);

    let mut res = if len1 <= len2 {
        partial_ratio_short_needle(
            s1_iter.clone(),
            len1,
            s2_iter.clone(),
            len2,
            score_cutoff / 100.0,
            args.score_hint,
        )
    } else {
        partial_ratio_short_needle(
            s2_iter.clone(),
            len2,
            s1_iter.clone(),
            len1,
            score_cutoff / 100.0,
            args.score_hint,
        )
    };
    if (res.score != 100.0) && (len1 == len2) {
        score_cutoff = f64::max(score_cutoff, res.score);
        let res2 = if len1 <= len2 {
            partial_ratio_short_needle(
                s2_iter.clone(),
                len2,
                s1_iter.clone(),
                len1,
                score_cutoff / 100.0,
                args.score_hint,
            )
        } else {
            partial_ratio_short_needle(
                s1_iter.clone(),
                len1,
                s2_iter.clone(),
                len2,
                score_cutoff / 100.0,
                args.score_hint,
            )
        };
        if res2.score > res.score {
            res = ScoreAlignment {
                score: res2.score,
                src_start: res2.dest_start,
                src_end: res2.dest_end,
                dest_start: res2.src_start,
                dest_end: res2.src_end,
            };
        }
    }

    if res.score < score_cutoff {
        return None;
    }

    // do we need this? why not just `Some(res)`?
    if len1 <= len2 {
        return Some(res);
    }

    Some(ScoreAlignment {
        score: res.score,
        src_start: res.dest_start,
        src_end: res.dest_end,
        dest_start: res.src_start,
        dest_end: res.src_end,
    })
}

/**
implementation of partial_ratio for needles <= 64. assumes s1 is already the
shorter string
*/
fn partial_ratio_short_needle<Iter1, Iter2>(
    s1: Iter1,
    len1: usize,
    s2: Iter2,
    len2: usize,
    mut score_cutoff: f64,
    score_hint: Option<f64>,
) -> ScoreAlignment
where
    Iter1: IntoIterator,
    Iter1::IntoIter: DoubleEndedIterator + Clone,
    Iter2: IntoIterator,
    Iter2::IntoIter: DoubleEndedIterator + Clone,
    Iter1::Item: PartialEq<Iter2::Item> + HashableChar + Copy,
    Iter2::Item: PartialEq<Iter1::Item> + HashableChar + Copy,
{
    if len1 == 0 {
        return ScoreAlignment {
            score: 0.0,
            src_start: 0,
            src_end: 0,
            dest_start: 0,
            dest_end: 0,
        };
    }

    let s1_iter = s1.into_iter();
    let s2_iter = s2.into_iter();

    let s1_char_set = s1_iter
        .clone()
        .map(|c| c.hash_char())
        .collect::<HashSet<_>>();

    let mut res = ScoreAlignment {
        score: 0.0,
        src_start: 0,
        src_end: len1,
        dest_start: 0,
        dest_end: len1,
    };

    let indel_comp = indel::BatchComparator::new(s1_iter.clone());

    for i in 1..len1 {
        let substr_last = s2_iter.clone().nth(i - 1).unwrap();
        if !s1_char_set.contains(&substr_last.hash_char()) {
            continue;
        }

        let ls_ratio = indel_comp
            .normalized_similarity_with_args(
                s2_iter.clone().take(i).collect::<Vec<_>>().into_iter(),
                &indel::Args {
                    score_cutoff: WithScoreCutoff(score_cutoff),
                    score_hint,
                },
            )
            .expect("score should be calculated");
        if ls_ratio > res.score {
            score_cutoff = ls_ratio;
            res.score = ls_ratio;
            res.dest_start = 0;
            res.dest_end = i;
            if res.score == 1.0 {
                res.score = 100.0;
                return res;
            }
        }
    }

    let window_end = len2 - len1;
    for i in 0..window_end {
        let substr_last = s2_iter.clone().nth(i + len1 - 1).unwrap();
        if !s1_char_set.contains(&substr_last.hash_char()) {
            continue;
        }

        let ls_ratio = indel_comp
            .normalized_similarity_with_args(
                s2_iter
                    .clone()
                    .skip(i)
                    .take(len1)
                    .collect::<Vec<_>>()
                    .into_iter(),
                &indel::Args {
                    score_cutoff: WithScoreCutoff(score_cutoff),
                    score_hint,
                },
            )
            .expect("score should be calculated");
        if ls_ratio > res.score {
            score_cutoff = ls_ratio;
            res.score = ls_ratio;
            res.dest_start = i;
            res.dest_end = i + len1;
            if res.score == 1.0 {
                res.score = 100.0;
                return res;
            }
        }
    }

    for i in window_end..len2 {
        let substr_first = s2_iter.clone().nth(i).unwrap();
        if !s1_char_set.contains(&substr_first.hash_char()) {
            continue;
        }

        let ls_ratio = indel_comp
            .normalized_similarity_with_args(
                s2_iter.clone().skip(i).collect::<Vec<_>>().into_iter(),
                &indel::Args {
                    score_cutoff: WithScoreCutoff(score_cutoff),
                    score_hint,
                },
            )
            .expect("score should be calculated");
        if ls_ratio > res.score {
            score_cutoff = ls_ratio;
            res.score = ls_ratio;
            res.dest_start = i;
            res.dest_end = len2;
            if res.score == 1.0 {
                res.score = 100.0;
                return res;
            }
        }
    }

    res.score *= 100.0;
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    static S1: &str = "new york mets";
    static S3: &str = "the wonderful new york mets";
    //static S4: &str = "new york mets vs atlanta braves";
    //static S5: &str = "atlanta braves vs new york mets";
    //static S7: &str = "new york city mets - atlanta braves";
    // test silly corner cases
    static S8: &str = "{";
    static S9: &str = "{a";
    //static S10: &str = "a{";
    //static S10A: &str = "{b";

    macro_rules! assert_delta {
        ($x:expr, $y:expr) => {
            match ($x, $y) {
                (None, None) => {}
                (Some(val1), Some(val2)) => {
                    if (val1 - val2).abs() > 0.0001 {
                        panic!("{:?} != {:?}", $x, $y);
                    }
                }
                (_, _) => panic!("{:?} != {:?}", $x, $y),
            }
        };
    }

    #[test]
    fn test_equal() {
        assert_delta!(
            Some(1.0),
            Some(ratio_with_args(S1.chars(), S1.chars(), &Args::default()))
        );
        assert_delta!(
            Some(1.0),
            Some(ratio_with_args(
                "test".chars(),
                "test".chars(),
                &Args::default()
            ))
        );
        assert_delta!(
            Some(1.0),
            Some(ratio_with_args(S8.chars(), S8.chars(), &Args::default()))
        );
        assert_delta!(
            Some(1.0),
            Some(ratio_with_args(S9.chars(), S9.chars(), &Args::default()))
        );
    }

    #[test]
    fn test_partial_ratio() {
        //assert_delta!(Some(1.0), partial_ratio(S1.chars(), S1.chars(), None, None));
        assert_delta!(
            Some(0.65),
            Some(ratio_with_args(S1.chars(), S3.chars(), &Args::default()))
        );
        //assert_delta!(Some(1.0), partial_ratio(S1.chars(), S3.chars(), None, None));
    }

    #[test]
    fn two_empty_strings() {
        assert_delta!(
            Some(1.0),
            Some(ratio_with_args("".chars(), "".chars(), &Args::default()))
        );
    }

    #[test]
    fn first_string_empty() {
        assert_delta!(
            Some(0.0),
            Some(ratio_with_args(
                "test".chars(),
                "".chars(),
                &Args::default()
            ))
        );
    }

    #[test]
    fn second_string_empty() {
        assert_delta!(
            Some(0.0),
            Some(ratio_with_args(
                "".chars(),
                "test".chars(),
                &Args::default()
            ))
        );
    }

    // https://github.com/rapidfuzz/RapidFuzz/issues/206
    #[test]
    fn issue206() {
        let str1 = "South Korea";
        let str2 = "North Korea";

        {
            let score = ratio(str1.chars(), str2.chars());

            assert_eq!(
                None,
                ratio_with_args(
                    str1.chars(),
                    str2.chars(),
                    &Args::default().score_cutoff(score + 0.0001)
                )
            );
            assert_delta!(
                Some(score),
                ratio_with_args(
                    str1.chars(),
                    str2.chars(),
                    &Args::default().score_cutoff(score - 0.0001)
                )
            );
        }
    }

    // https://github.com/rapidfuzz/RapidFuzz/issues/210
    #[test]
    fn issue210() {
        let str1 = "bc";
        let str2 = "bca";

        {
            let score = ratio(str1.chars(), str2.chars());

            assert_eq!(
                None,
                ratio_with_args(
                    str1.chars(),
                    str2.chars(),
                    &Args::default().score_cutoff(score + 0.0001)
                )
            );
            assert_delta!(
                Some(score),
                ratio_with_args(
                    str1.chars(),
                    str2.chars(),
                    &Args::default().score_cutoff(score - 0.0001)
                )
            );
        }
    }

    #[test]
    fn test_partial_ratio_short_needle_identical() {
        let s1 = "abcd";
        let s2 = "abcd";

        let result = partial_ratio_short_needle(
            s1.chars(),
            s1.chars().count(),
            s2.chars(),
            s2.chars().count(),
            0.0,
            None,
        );

        assert_eq!(result.score, 100.0);
        assert_eq!(result.src_start, 0);
        assert_eq!(result.src_end, 4);
        assert_eq!(result.dest_start, 0);
        assert_eq!(result.dest_end, 4);
    }

    #[test]
    fn test_partial_ratio_short_needle_substring() {
        let s1 = "bcd";
        let s2 = "abcde";

        let result = partial_ratio_short_needle(
            s1.chars(),
            s1.chars().count(),
            s2.chars(),
            s2.chars().count(),
            0.0,
            None,
        );

        assert_eq!(result.score, 100.0);
        assert_eq!(result.src_start, 0);
        assert_eq!(result.src_end, 3);
        assert_eq!(result.dest_start, 1);
        assert_eq!(result.dest_end, 4);
    }
}
