use std::collections::HashMap;
use std::ops;

#[derive(Debug, PartialEq)]
pub struct DiscreteDistribution<const N: usize> {
    entries: HashMap<String, f64>,
}

impl<const N: usize> ops::Index<&str> for DiscreteDistribution<N> {
    type Output = f64;
    fn index(&self, index: &str) -> &Self::Output {
        &self.entries[index]
    }
}

pub fn naive_bayes<'a, const N: usize, T>(
    dist_1: &'a DiscreteDistribution<N>,
    dist_2: &'a DiscreteDistribution<N>,
    prior_probability: f64,
    to_classify: T,
) -> &'a DiscreteDistribution<N>
where
    T: Iterator<Item = &'a str>,
{
    let mut prob_dist_1_given_tc = prior_probability;
    let mut prob_dist_2_given_tc = 1.0 - prior_probability;
    for ch in to_classify {
        prob_dist_1_given_tc *= dist_1[ch];
        prob_dist_2_given_tc *= dist_2[ch];
    }

    dbg!(prob_dist_2_given_tc);
    dbg!(prob_dist_1_given_tc);
    if prob_dist_2_given_tc > prob_dist_1_given_tc {
        dist_2
    } else {
        dist_1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let dd1_h = HashMap::from([
            ("Dear".to_string(), 0.47),
            ("Friend".to_string(), 0.29),
            ("Lunch".to_string(), 0.18),
            ("Money".to_string(), 0.06),
        ]);

        let dd2_h = HashMap::from([
            ("Dear".to_string(), 0.29),
            ("Friend".to_string(), 0.14),
            ("Lunch".to_string(), 0.00),
            ("Money".to_string(), 0.57),
        ]);

        let dd1 = DiscreteDistribution::<4> { entries: dd1_h };
        let dd2 = DiscreteDistribution::<4> { entries: dd2_h };
        let result = naive_bayes(
            &dd1,
            &dd2,
            0.67,
            vec!["Money", "Money", "Money"].into_iter(),
        );
        assert_eq!(result, &dd2);
    }
}
