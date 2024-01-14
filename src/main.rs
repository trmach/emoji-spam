use std::env;
use std::collections::HashMap;
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;
use emojis;

fn main() {
    let matcher = SkimMatcherV2::default();
//  assert_eq!(None, matcher.fuzzy_match("abc", "abx"));
//  assert!(matcher.fuzzy_match("axbycz", "abc").is_some());
//  assert!(matcher.fuzzy_match("axbycz", "xyz").is_some());

//  let (score, indices) = matcher.fuzzy_indices("axbycz", "abc").unwrap();
//  assert_eq!(indices, [0, 2, 4]);

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

    for e in emojis::iter().map(|e| e.as_str()).collect::<Vec<_>>() {
        let item = emojis::get(e).unwrap();
        map.insert(item.name(), item);
    }

    let mut output: String = Default::default();
    for word in input.split(' ') {
        output += format!("{} ", word).as_str();
        for k in map.keys() {
            if matcher.fuzzy_match(k, word).is_some() {
                println!("{} -> {}", k, map.get(k).unwrap().as_str());
                output += format!("{} ", map.get(k).expect("Map returned none.").to_string()).as_str();
                break;
            }
        }
    }
    println!("{}", output);
}
