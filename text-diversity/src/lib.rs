use std::collections::{HashMap, HashSet};

use itertools::Itertools;

pub fn jaccard(s1: &str, s2: &str) -> f64 {
    let set1: HashSet<&str> = s1.split_whitespace().collect();
    let set2: HashSet<&str> = s2.split_whitespace().collect();

    let intersection = set1.intersection(&set2).count();
    let union = set1.union(&set2).count();

    if union == 0 {
        return 1.0;
    }

    intersection as f64 / union as f64
}

pub fn distinct_n(responses: &[&str], n: usize) -> f64 {
    let mut unique_ngrams = HashSet::new();
    let mut total_count = 0;

    for response in responses {
        let words: Vec<&str> = response.split_whitespace().collect();
        for ngram in words.windows(n) {
            unique_ngrams.insert(ngram.join(" "));
            total_count += 1;
        }
    }

    if total_count == 0 {
        return 0.0;
    }

    unique_ngrams.len() as f64 / total_count as f64
}

fn ngram_counts(words: &[&str], n: usize) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    for ngram in words.windows(n) {
        *counts.entry(ngram.join(" ")).or_insert(0) += 1;
    }
    counts
}

fn modified_precision(candidate_words: &[&str], references: &[Vec<&str>], n: usize) -> f64 {
    let candidate_counts = ngram_counts(candidate_words, n);
    if candidate_counts.is_empty() {
        return 0.0;
    }

    let mut clipped_count = 0;
    let mut total_count = 0;

    for (ngram, count) in &candidate_counts {
        let max_ref_count = references
            .iter()
            .map(|ref_words| ngram_counts(ref_words, n).get(ngram).copied().unwrap_or(0))
            .max()
            .unwrap_or(0);

        clipped_count += (*count).min(max_ref_count);
        total_count += count;
    }

    clipped_count as f64 / total_count as f64
}

fn brevity_penalty(candidate_len: usize, references: &[Vec<&str>]) -> f64 {
    if candidate_len == 0 {
        return 0.0;
    }

    let closest_ref_len = references
        .iter()
        .map(|r| r.len())
        .min_by_key(|&len| (len as isize - candidate_len as isize).abs())
        .unwrap_or(0);

    if candidate_len >= closest_ref_len {
        1.0
    } else {
        (1.0 - closest_ref_len as f64 / candidate_len as f64).exp()
    }
}

pub fn bleu(candidate: &str, references: &[&str], max_n: usize) -> f64 {
    let candidate_words: Vec<&str> = candidate.split_whitespace().collect();
    let ref_words: Vec<Vec<&str>> = references
        .iter()
        .map(|r| r.split_whitespace().collect())
        .collect();

    if candidate_words.is_empty() || ref_words.is_empty() {
        return 0.0;
    }

    let precisions: Vec<f64> = (1..=max_n)
        .map(|n| modified_precision(&candidate_words, &ref_words, n))
        .collect();

    if precisions.contains(&0.0) {
        return 0.0;
    }

    let log_avg = precisions.iter().map(|p| p.ln()).sum::<f64>() / max_n as f64;
    let bp = brevity_penalty(candidate_words.len(), &ref_words);

    bp * log_avg.exp()
}

pub fn self_bleu(responses: &[&str], max_n: usize) -> f64 {
    if responses.len() < 2 {
        return 0.0;
    }

    let sum: f64 = responses
        .iter()
        .enumerate()
        .map(|(i, candidate)| {
            let references: Vec<&str> = responses
                .iter()
                .enumerate()
                .filter(|(j, _)| *j != i)
                .map(|(_, r)| *r)
                .collect();
            bleu(candidate, &references, max_n)
        })
        .sum();

    sum / responses.len() as f64
}

pub fn self_bleu_diversity(responses: &[&str], max_n: usize) -> f64 {
    1.0 - self_bleu(responses, max_n)
}

pub fn pairwise_jaccard_diversity(responses: &[&str]) -> f64 {
    if responses.len() < 2 {
        return 1.0;
    }

    let pairs: Vec<_> = responses.iter().combinations(2).collect();
    let total_pairs = pairs.len() as f64;

    let sum_similarity: f64 = pairs
        .into_iter()
        .map(|pair| jaccard(pair[0], pair[1]))
        .sum();

    1.0 - sum_similarity / total_pairs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn jaccard_identical() {
        assert_eq!(jaccard("猫 追 狗", "猫 追 狗"), 1.0);
    }

    #[test]
    fn jaccard_no_overlap() {
        assert_eq!(jaccard("猫 追 狗", "鸟 飞 天"), 0.0);
    }

    #[test]
    fn jaccard_partial_overlap() {
        assert_eq!(jaccard("猫 追 狗", "猫 追 鼠"), 0.5);
    }

    #[test]
    fn jaccard_empty_strings() {
        assert_eq!(jaccard("", ""), 1.0);
    }

    #[test]
    fn diversity_identical_responses() {
        let responses = vec!["猫 追 狗", "猫 追 狗", "猫 追 狗"];
        assert_eq!(pairwise_jaccard_diversity(&responses), 0.0);
    }

    #[test]
    fn diversity_no_overlap() {
        let responses = vec!["猫 追 狗", "鸟 飞 天", "鱼 游 水"];
        assert_eq!(pairwise_jaccard_diversity(&responses), 1.0);
    }

    #[test]
    fn diversity_single_response() {
        assert_eq!(pairwise_jaccard_diversity(&["猫 追 狗"]), 1.0);
    }

    #[test]
    fn diversity_empty() {
        assert_eq!(pairwise_jaccard_diversity(&[]), 1.0);
    }

    #[test]
    fn distinct_n_all_same() {
        let responses = vec!["我 喜欢 苹果", "我 喜欢 苹果", "我 喜欢 苹果"];
        assert_eq!(distinct_n(&responses, 2), 2.0 / 6.0);
    }

    #[test]
    fn distinct_n_all_different() {
        let responses = vec!["我 喜欢 苹果", "猫 追 狗"];
        assert_eq!(distinct_n(&responses, 2), 1.0);
    }

    #[test]
    fn distinct_n_empty() {
        assert_eq!(distinct_n(&[], 2), 0.0);
    }

    #[test]
    fn distinct_n_short_responses() {
        let responses = vec!["单词"];
        assert_eq!(distinct_n(&responses, 2), 0.0);
    }

    #[test]
    fn bleu_identical() {
        assert_eq!(bleu("猫 坐在 垫子 上", &["猫 坐在 垫子 上"], 4), 1.0);
    }

    #[test]
    fn bleu_no_overlap() {
        assert_eq!(bleu("猫 坐在 垫子 上", &["鸟 飞 在 天"], 4), 0.0);
    }

    #[test]
    fn bleu_partial_overlap() {
        let score = bleu("猫 坐在 地板 上", &["猫 坐在 垫子 上"], 2);
        assert!(score > 0.0 && score < 1.0);
    }

    #[test]
    fn self_bleu_identical() {
        let responses = vec!["猫 坐在 垫子 上", "猫 坐在 垫子 上", "猫 坐在 垫子 上"];
        assert_eq!(self_bleu(&responses, 4), 1.0);
    }

    #[test]
    fn self_bleu_no_overlap() {
        let responses = vec!["猫 坐在 垫子 上", "鸟 飞 在 天 空", "鱼 游 在 水 里"];
        assert_eq!(self_bleu(&responses, 4), 0.0);
    }

    #[test]
    fn self_bleu_diversity_identical() {
        let responses = vec!["猫 坐在 垫子 上", "猫 坐在 垫子 上"];
        assert_eq!(self_bleu_diversity(&responses, 4), 0.0);
    }

    #[test]
    fn self_bleu_diversity_no_overlap() {
        let responses = vec!["猫 坐在 垫子 上", "鸟 飞 在 天 空"];
        assert_eq!(self_bleu_diversity(&responses, 4), 1.0);
    }
}
