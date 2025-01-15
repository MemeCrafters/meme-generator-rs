use std::collections::HashMap;

use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;

use meme_generator_core::meme::Meme;

pub fn search_memes(
    memes: &HashMap<String, Box<dyn Meme>>,
    query: &str,
    include_tags: bool,
) -> Vec<String> {
    let mut results = Vec::new();
    let matcher = SkimMatcherV2::default();

    fn max(a: Option<i64>, b: Option<i64>) -> Option<i64> {
        match (a, b) {
            (Some(x), Some(y)) => Some(x.max(y)),
            (Some(x), None) => Some(x),
            (None, Some(y)) => Some(y),
            (None, None) => None,
        }
    }

    for (key, meme) in memes {
        let info = meme.info();
        let mut score = matcher.fuzzy_match(key, &query);
        for keyword in info.keywords {
            score = max(score, matcher.fuzzy_match(&keyword, &query));
        }
        if include_tags {
            for tag in info.tags {
                score = max(score, matcher.fuzzy_match(&tag, &query));
            }
        }
        if let Some(score) = score {
            results.push((key.clone(), score));
        }
    }
    results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    results.into_iter().map(|(key, _)| key).collect()
}
