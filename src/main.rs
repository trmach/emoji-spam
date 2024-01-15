use std::env;
use std::collections::HashMap;
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;
use emojis;

fn main() {
    let matcher = SkimMatcherV2::default();
    let input: String;
    if env::args().len() == 1 {
        input = r#"According to all known laws of aviation, there is no way 
a bee should be able to fly. Its wings are too small to get its 
fat little body off the ground. The bee, of course, flies anyway 
because bees don't care what humans think is impossible."#.to_string();
    } else {
        input = env::args().skip(1).take(1).collect();
    }

    let mut map = HashMap::new();

    // Make a dictionary of emojis.
    for e in emojis::iter().map(|e| e.as_str()).collect::<Vec<_>>() {
        let item = emojis::get(e).unwrap();
        map.insert(item.name(), item);
    }

    let mut output: String = Default::default();
    for word in input.split(' ') {
        output += format!("{} ", word).as_str();
        let mut options: Vec<(i64, String)> = vec![];

        // Fuzzy matching
        for (k, v) in &map {
            let option = matcher.fuzzy_match(k, word);
            if option.is_some() {
                options.push((option.unwrap(), v.to_string()));
            }
        }

        // sort and reverse to get the best matches
        options.sort_by_key(|k| k.0);
        options.reverse();
        if options.len() > 0 {
            output += format!("{} ", options[0].1).as_str();
        }
    }
    println!("{}", output);
}
