use crate::details::common::{norm_sim_to_norm_dist, HashableChar};

macro_rules! less_than_score_cutoff_similarity {
    ($score_cutoff:expr, f32) => {
        1.0
    };
    ($score_cutoff:expr, f64) => {
        1.0
    };
    ($score_cutoff:expr, $tp:ty) => {
        $score_cutoff + 1
    };
}

// todo maybe some of these could be traits instead?
macro_rules! build_normalized_metric_funcs
{
    ($impl_type:tt, $res_type:ty, $worst_similarity:expr, $worst_distance:expr $(, $v:ident: $t:ty)*) => {
        pub(crate) fn normalized_distance<Iter1, Iter2, Elem1, Elem2>(
            s1: Iter1,
            len1: usize,
            s2: Iter2,
            len2: usize,
            $($v: $t,)*
            score_cutoff: f64,
            score_hint: f64
        ) -> f64
        where
            Iter1: Iterator<Item = Elem1> + DoubleEndedIterator + Clone,
            Iter2: Iterator<Item = Elem2> + DoubleEndedIterator + Clone,
            Elem1: PartialEq<Elem2> + HashableChar + Copy,
            Elem2: PartialEq<Elem1> + HashableChar + Copy,
        {
            let maximum = $impl_type::maximum(len1, len2 $(,$v)*);

            let cutoff_distance = (maximum as f64 * score_cutoff).ceil() as $res_type;
            let hint_distance = (maximum as f64 * score_hint).ceil() as $res_type;

            let dist = $impl_type::distance(
                s1,
                len1,
                s2,
                len2,
                $($v,)*
                cutoff_distance,
                hint_distance
            );
            let norm_dist = if maximum != 0 as $res_type {
                dist as f64 / maximum as f64
            } else {
                0.0
            };
            if norm_dist <= score_cutoff {
                norm_dist
            } else {
                1.0
            }
        }

        pub(crate) fn normalized_similarity<Iter1, Iter2, Elem1, Elem2>(
            s1: Iter1,
            len1: usize,
            s2: Iter2,
            len2: usize,
            $($v: $t,)*
            score_cutoff: f64,
            score_hint: f64
        ) -> f64
        where
            Iter1: Iterator<Item = Elem1> + DoubleEndedIterator + Clone,
            Iter2: Iterator<Item = Elem2> + DoubleEndedIterator + Clone,
            Elem1: PartialEq<Elem2> + HashableChar + Copy,
            Elem2: PartialEq<Elem1> + HashableChar + Copy,
        {
            let cutoff_score = norm_sim_to_norm_dist(score_cutoff);
            let hint_score = norm_sim_to_norm_dist(score_hint);

            let norm_dist = $impl_type::normalized_distance(
                s1,
                len1,
                s2,
                len2,
                $($v,)*
                cutoff_score,
                hint_score
            );
            let norm_sim = 1.0 - norm_dist;

            if norm_sim >= score_cutoff {
                norm_sim
            } else {
                0.0
            }
        }
    };
}

macro_rules! build_similarity_metric_funcs
{
    ($impl_type:tt, $res_type:tt, $worst_similarity:expr, $worst_distance:expr $(, $v:ident: $t:ty)*) => {
        build_normalized_metric_funcs!($impl_type, $res_type, $worst_similarity, $worst_distance $(, $v: $t)*);


        pub(crate) fn distance<Iter1, Iter2, Elem1, Elem2>(
            s1: Iter1,
            len1: usize,
            s2: Iter2,
            len2: usize,
            $($v: $t,)*
            score_cutoff: $res_type,
            score_hint: $res_type,
        ) -> $res_type
        where
            Iter1: Iterator<Item = Elem1> + DoubleEndedIterator + Clone,
            Iter2: Iterator<Item = Elem2> + DoubleEndedIterator + Clone,
            Elem1: PartialEq<Elem2> + HashableChar + Copy,
            Elem2: PartialEq<Elem1> + HashableChar + Copy,
        {
            let maximum = $impl_type::maximum(len1, len2 $(,$v)*);

            let cutoff_similarity = if maximum >= score_cutoff {
                maximum - score_cutoff
            } else {
                $worst_similarity as $res_type
            };
            let hint_similarity = if maximum >= score_hint {
                maximum - score_hint
            } else {
                $worst_similarity as $res_type
            };

            let sim = $impl_type::similarity(s1, len1, s2, len2, $($v,)* cutoff_similarity, hint_similarity);
            let dist = maximum - sim;

            if dist <= score_cutoff {
                dist
            } else {
                less_than_score_cutoff_similarity!(score_cutoff, $res_type)
            }
        }
    };
}

pub(crate) use build_normalized_metric_funcs;
pub(crate) use build_similarity_metric_funcs;
pub(crate) use less_than_score_cutoff_similarity;

pub(crate) trait DistanceMetricUsize {
    fn maximum(&self, len1: usize, len2: usize) -> usize;

    fn _distance<Iter1, Iter2, Elem1, Elem2>(
        &self,
        s1: Iter1,
        len1: usize,
        s2: Iter2,
        len2: usize,
        score_cutoff: usize,
        score_hint: usize,
    ) -> usize
    where
        Iter1: Iterator<Item = Elem1> + DoubleEndedIterator + Clone,
        Iter2: Iterator<Item = Elem2> + DoubleEndedIterator + Clone,
        Elem1: PartialEq<Elem2> + HashableChar + Copy,
        Elem2: PartialEq<Elem1> + HashableChar + Copy;

    fn _similarity<Iter1, Iter2, Elem1, Elem2>(
        &self,
        s1: Iter1,
        len1: usize,
        s2: Iter2,
        len2: usize,
        score_cutoff: usize,
        mut score_hint: usize,
    ) -> usize
    where
        Iter1: Iterator<Item = Elem1> + DoubleEndedIterator + Clone,
        Iter2: Iterator<Item = Elem2> + DoubleEndedIterator + Clone,
        Elem1: PartialEq<Elem2> + HashableChar + Copy,
        Elem2: PartialEq<Elem1> + HashableChar + Copy,
    {
        let maximum = self.maximum(len1, len2);
        if score_cutoff > maximum {
            return 0;
        }

        score_hint = score_hint.min(score_cutoff);
        let cutoff_distance = maximum - score_cutoff;
        let hint_distance = maximum - score_hint;
        let dist = self._distance(s1, len1, s2, len2, cutoff_distance, hint_distance);
        let sim = maximum - dist;
        if sim >= score_cutoff {
            sim
        } else {
            0
        }
    }
}

pub(crate) trait SimilarityMetricUsize {
    fn maximum(&self, len1: usize, len2: usize) -> usize;

    fn _distance<Iter1, Iter2, Elem1, Elem2>(
        &self,
        s1: Iter1,
        len1: usize,
        s2: Iter2,
        len2: usize,
        score_cutoff: usize,
        score_hint: usize,
    ) -> usize
    where
        Iter1: Iterator<Item = Elem1> + DoubleEndedIterator + Clone,
        Iter2: Iterator<Item = Elem2> + DoubleEndedIterator + Clone,
        Elem1: PartialEq<Elem2> + HashableChar + Copy,
        Elem2: PartialEq<Elem1> + HashableChar + Copy,
    {
        let maximum = self.maximum(len1, len2);

        let cutoff_similarity = if maximum >= score_cutoff {
            maximum - score_cutoff
        } else {
            0
        };
        let hint_similarity = if maximum >= score_hint {
            maximum - score_hint
        } else {
            0
        };

        let sim = self._similarity(s1, len1, s2, len2, cutoff_similarity, hint_similarity);
        let dist = maximum - sim;

        if dist <= score_cutoff {
            dist
        } else {
            score_cutoff + 1
        }
    }

    fn _similarity<Iter1, Iter2, Elem1, Elem2>(
        &self,
        s1: Iter1,
        len1: usize,
        s2: Iter2,
        len2: usize,
        score_cutoff: usize,
        mut score_hint: usize,
    ) -> usize
    where
        Iter1: Iterator<Item = Elem1> + DoubleEndedIterator + Clone,
        Iter2: Iterator<Item = Elem2> + DoubleEndedIterator + Clone,
        Elem1: PartialEq<Elem2> + HashableChar + Copy,
        Elem2: PartialEq<Elem1> + HashableChar + Copy,
    {
        let maximum = self.maximum(len1, len2);
        if score_cutoff > maximum {
            return 0;
        }

        score_hint = score_hint.min(score_cutoff);
        let cutoff_distance = maximum - score_cutoff;
        let hint_distance = maximum - score_hint;
        let dist = self._distance(s1, len1, s2, len2, cutoff_distance, hint_distance);
        let sim = maximum - dist;
        if sim >= score_cutoff {
            sim
        } else {
            0
        }
    }
}

pub(crate) trait NormalizedMetricUsize {
    fn _normalized_distance<Iter1, Iter2, Elem1, Elem2>(
        &self,
        s1: Iter1,
        len1: usize,
        s2: Iter2,
        len2: usize,
        score_cutoff: f64,
        score_hint: f64,
    ) -> f64
    where
        Iter1: Iterator<Item = Elem1> + DoubleEndedIterator + Clone,
        Iter2: Iterator<Item = Elem2> + DoubleEndedIterator + Clone,
        Elem1: PartialEq<Elem2> + HashableChar + Copy,
        Elem2: PartialEq<Elem1> + HashableChar + Copy;
    fn _normalized_similarity<Iter1, Iter2, Elem1, Elem2>(
        &self,
        s1: Iter1,
        len1: usize,
        s2: Iter2,
        len2: usize,
        score_cutoff: f64,
        score_hint: f64,
    ) -> f64
    where
        Iter1: Iterator<Item = Elem1> + DoubleEndedIterator + Clone,
        Iter2: Iterator<Item = Elem2> + DoubleEndedIterator + Clone,
        Elem1: PartialEq<Elem2> + HashableChar + Copy,
        Elem2: PartialEq<Elem1> + HashableChar + Copy;
}

impl<T: DistanceMetricUsize> NormalizedMetricUsize for T {
    fn _normalized_distance<Iter1, Iter2, Elem1, Elem2>(
        &self,
        s1: Iter1,
        len1: usize,
        s2: Iter2,
        len2: usize,
        score_cutoff: f64,
        score_hint: f64,
    ) -> f64
    where
        Iter1: Iterator<Item = Elem1> + DoubleEndedIterator + Clone,
        Iter2: Iterator<Item = Elem2> + DoubleEndedIterator + Clone,
        Elem1: PartialEq<Elem2> + HashableChar + Copy,
        Elem2: PartialEq<Elem1> + HashableChar + Copy,
    {
        let maximum = self.maximum(len1, len2);

        let cutoff_distance = (maximum as f64 * score_cutoff).ceil() as usize;
        let hint_distance = (maximum as f64 * score_hint).ceil() as usize;

        let dist = self._distance(s1, len1, s2, len2, cutoff_distance, hint_distance);
        let norm_dist = if maximum != 0 {
            dist as f64 / maximum as f64
        } else {
            0.0
        };
        if norm_dist <= score_cutoff {
            norm_dist
        } else {
            1.0
        }
    }

    fn _normalized_similarity<Iter1, Iter2, Elem1, Elem2>(
        &self,
        s1: Iter1,
        len1: usize,
        s2: Iter2,
        len2: usize,
        score_cutoff: f64,
        score_hint: f64,
    ) -> f64
    where
        Iter1: Iterator<Item = Elem1> + DoubleEndedIterator + Clone,
        Iter2: Iterator<Item = Elem2> + DoubleEndedIterator + Clone,
        Elem1: PartialEq<Elem2> + HashableChar + Copy,
        Elem2: PartialEq<Elem1> + HashableChar + Copy,
    {
        let cutoff_score = norm_sim_to_norm_dist(score_cutoff);
        let hint_score = norm_sim_to_norm_dist(score_hint);

        let norm_dist = self._normalized_distance(s1, len1, s2, len2, cutoff_score, hint_score);
        let norm_sim = 1.0 - norm_dist;

        if norm_sim >= score_cutoff {
            norm_sim
        } else {
            0.0
        }
    }
}

// todo how not to duplicate this?
pub(crate) trait NormalizedMetricUsize2 {
    fn _normalized_distance<Iter1, Iter2, Elem1, Elem2>(
        &self,
        s1: Iter1,
        len1: usize,
        s2: Iter2,
        len2: usize,
        score_cutoff: f64,
        score_hint: f64,
    ) -> f64
    where
        Iter1: Iterator<Item = Elem1> + DoubleEndedIterator + Clone,
        Iter2: Iterator<Item = Elem2> + DoubleEndedIterator + Clone,
        Elem1: PartialEq<Elem2> + HashableChar + Copy,
        Elem2: PartialEq<Elem1> + HashableChar + Copy;
    fn _normalized_similarity<Iter1, Iter2, Elem1, Elem2>(
        &self,
        s1: Iter1,
        len1: usize,
        s2: Iter2,
        len2: usize,
        score_cutoff: f64,
        score_hint: f64,
    ) -> f64
    where
        Iter1: Iterator<Item = Elem1> + DoubleEndedIterator + Clone,
        Iter2: Iterator<Item = Elem2> + DoubleEndedIterator + Clone,
        Elem1: PartialEq<Elem2> + HashableChar + Copy,
        Elem2: PartialEq<Elem1> + HashableChar + Copy;
}

impl<T: SimilarityMetricUsize> NormalizedMetricUsize2 for T {
    fn _normalized_distance<Iter1, Iter2, Elem1, Elem2>(
        &self,
        s1: Iter1,
        len1: usize,
        s2: Iter2,
        len2: usize,
        score_cutoff: f64,
        score_hint: f64,
    ) -> f64
    where
        Iter1: Iterator<Item = Elem1> + DoubleEndedIterator + Clone,
        Iter2: Iterator<Item = Elem2> + DoubleEndedIterator + Clone,
        Elem1: PartialEq<Elem2> + HashableChar + Copy,
        Elem2: PartialEq<Elem1> + HashableChar + Copy,
    {
        let maximum = self.maximum(len1, len2);

        let cutoff_distance = (maximum as f64 * score_cutoff).ceil() as usize;
        let hint_distance = (maximum as f64 * score_hint).ceil() as usize;

        let dist = self._distance(s1, len1, s2, len2, cutoff_distance, hint_distance);
        let norm_dist = if maximum != 0 {
            dist as f64 / maximum as f64
        } else {
            0.0
        };
        if norm_dist <= score_cutoff {
            norm_dist
        } else {
            1.0
        }
    }

    fn _normalized_similarity<Iter1, Iter2, Elem1, Elem2>(
        &self,
        s1: Iter1,
        len1: usize,
        s2: Iter2,
        len2: usize,
        score_cutoff: f64,
        score_hint: f64,
    ) -> f64
    where
        Iter1: Iterator<Item = Elem1> + DoubleEndedIterator + Clone,
        Iter2: Iterator<Item = Elem2> + DoubleEndedIterator + Clone,
        Elem1: PartialEq<Elem2> + HashableChar + Copy,
        Elem2: PartialEq<Elem1> + HashableChar + Copy,
    {
        let cutoff_score = norm_sim_to_norm_dist(score_cutoff);
        let hint_score = norm_sim_to_norm_dist(score_hint);

        let norm_dist = self._normalized_distance(s1, len1, s2, len2, cutoff_score, hint_score);
        let norm_sim = 1.0 - norm_dist;

        if norm_sim >= score_cutoff {
            norm_sim
        } else {
            0.0
        }
    }
}

macro_rules! build_cached_normalized_metric_funcs {
    ($impl_type:tt, $res_type:ty, $worst_similarity:expr, $worst_distance:expr) => {
        #[allow(dead_code)]
        pub fn normalized_distance<Iter2, Elem2, ScoreCutoff, ScoreHint>(
            &self,
            s2: Iter2,
            score_cutoff: ScoreCutoff,
            score_hint: ScoreHint,
        ) -> f64
        where
            Iter2: IntoIterator<Item = Elem2>,
            Iter2::IntoIter: DoubleEndedIterator + Clone,
            Elem1: PartialEq<Elem2> + HashableChar + Copy,
            Elem2: PartialEq<Elem1> + HashableChar + Copy,
            ScoreCutoff: Into<Option<f64>>,
            ScoreHint: Into<Option<f64>>,
        {
            let s2_iter = s2.into_iter();
            let len2 = s2_iter.clone().count();
            self._normalized_distance(
                s2_iter,
                len2,
                score_cutoff.into().unwrap_or(1.0),
                score_hint.into().unwrap_or(1.0),
            )
        }

        pub(crate) fn _normalized_distance<Iter2, Elem2>(
            &self,
            s2: Iter2,
            len2: usize,
            score_cutoff: f64,
            score_hint: f64,
        ) -> f64
        where
            Iter2: Iterator<Item = Elem2> + DoubleEndedIterator + Clone,
            Elem1: PartialEq<Elem2> + HashableChar + Copy,
            Elem2: PartialEq<Elem1> + HashableChar + Copy,
        {
            let maximum = self.maximum(len2);

            let cutoff_distance = (maximum as f64 * score_cutoff).ceil() as $res_type;
            let hint_distance = (maximum as f64 * score_hint).ceil() as $res_type;

            let dist = self._distance(s2, len2, cutoff_distance, hint_distance);
            let norm_dist = if maximum != 0 as $res_type {
                dist as f64 / maximum as f64
            } else {
                0.0
            };
            if norm_dist <= score_cutoff {
                norm_dist
            } else {
                1.0
            }
        }

        #[allow(dead_code)]
        pub fn normalized_similarity<Iter2, Elem2, ScoreCutoff, ScoreHint>(
            &self,
            s2: Iter2,
            score_cutoff: ScoreCutoff,
            score_hint: ScoreHint,
        ) -> f64
        where
            Iter2: IntoIterator<Item = Elem2>,
            Iter2::IntoIter: DoubleEndedIterator + Clone,
            Elem1: PartialEq<Elem2> + HashableChar + Copy,
            Elem2: PartialEq<Elem1> + HashableChar + Copy,
            ScoreCutoff: Into<Option<f64>>,
            ScoreHint: Into<Option<f64>>,
        {
            let s2_iter = s2.into_iter();
            let len2 = s2_iter.clone().count();
            self._normalized_similarity(
                s2_iter,
                len2,
                score_cutoff.into().unwrap_or(0.0),
                score_hint.into().unwrap_or(0.0),
            )
        }

        pub(crate) fn _normalized_similarity<Iter2, Elem2>(
            &self,
            s2: Iter2,
            len2: usize,
            score_cutoff: f64,
            score_hint: f64,
        ) -> f64
        where
            Iter2: Iterator<Item = Elem2> + DoubleEndedIterator + Clone,
            Elem1: PartialEq<Elem2> + HashableChar + Copy,
            Elem2: PartialEq<Elem1> + HashableChar + Copy,
        {
            let cutoff_score = norm_sim_to_norm_dist(score_cutoff);
            let hint_score = norm_sim_to_norm_dist(score_hint);

            let norm_dist = self._normalized_distance(s2, len2, cutoff_score, hint_score);
            let norm_sim = 1.0 - norm_dist;

            if norm_sim >= score_cutoff {
                norm_sim
            } else {
                0.0
            }
        }
    };
}

macro_rules! build_cached_similarity_metric_funcs {
    ($impl_type:tt, $res_type:tt, $worst_similarity:expr, $worst_distance:expr) => {
        build_cached_normalized_metric_funcs!(
            $impl_type,
            $res_type,
            $worst_similarity,
            $worst_distance
        );

        #[allow(dead_code)]
        pub fn distance<Iter2, Elem2, ScoreCutoff, ScoreHint>(
            &self,
            s2: Iter2,
            score_cutoff: ScoreCutoff,
            score_hint: ScoreHint,
        ) -> $res_type
        where
            Iter2: IntoIterator<Item = Elem2>,
            Iter2::IntoIter: DoubleEndedIterator + Clone,
            Elem1: PartialEq<Elem2> + HashableChar + Copy,
            Elem2: PartialEq<Elem1> + HashableChar + Copy,
            ScoreCutoff: Into<Option<$res_type>>,
            ScoreHint: Into<Option<$res_type>>,
        {
            let s2_iter = s2.into_iter();
            let len2 = s2_iter.clone().count();
            self._distance(
                s2_iter,
                len2,
                score_cutoff.into().unwrap_or($worst_distance),
                score_hint.into().unwrap_or($worst_distance),
            )
        }

        #[allow(dead_code)]
        pub fn similarity<Iter2, Elem2, ScoreCutoff, ScoreHint>(
            &self,
            s2: Iter2,
            score_cutoff: ScoreCutoff,
            score_hint: ScoreHint,
        ) -> $res_type
        where
            Iter2: IntoIterator<Item = Elem2>,
            Iter2::IntoIter: DoubleEndedIterator + Clone,
            Elem1: PartialEq<Elem2> + HashableChar + Copy,
            Elem2: PartialEq<Elem1> + HashableChar + Copy,
            ScoreCutoff: Into<Option<$res_type>>,
            ScoreHint: Into<Option<$res_type>>,
        {
            let s2_iter = s2.into_iter();
            let len2 = s2_iter.clone().count();
            self._similarity(
                s2_iter,
                len2,
                score_cutoff.into().unwrap_or($worst_similarity),
                score_hint.into().unwrap_or($worst_similarity),
            )
        }

        pub(crate) fn _distance<Iter2, Elem2>(
            &self,
            s2: Iter2,
            len2: usize,
            score_cutoff: $res_type,
            score_hint: $res_type,
        ) -> $res_type
        where
            Iter2: Iterator<Item = Elem2> + DoubleEndedIterator + Clone,
            Elem1: PartialEq<Elem2> + HashableChar + Copy,
            Elem2: PartialEq<Elem1> + HashableChar + Copy,
        {
            let maximum = self.maximum(len2);

            let cutoff_similarity = if maximum >= score_cutoff {
                maximum - score_cutoff
            } else {
                $worst_similarity as $res_type
            };
            let hint_similarity = if maximum >= score_hint {
                maximum - score_hint
            } else {
                $worst_similarity as $res_type
            };

            let sim = self._similarity(s2, len2, cutoff_similarity, hint_similarity);
            let dist = maximum - sim;

            if dist <= score_cutoff {
                dist
            } else {
                less_than_score_cutoff_similarity!(score_cutoff, $res_type)
            }
        }
    };
}

pub(crate) use build_cached_normalized_metric_funcs;
pub(crate) use build_cached_similarity_metric_funcs;
