use crate::prelude::*;

#[derive(Debug, Clone)]
pub enum Rule {
    Atomic(char),
    Composite(Vec<Vec<i64>>),
}

pub fn parse(input: &str) -> (HashMap<i64, Rule>, Vec<String>) {
    let (rules, strings) = input.split2("\n\n");

    let rules = rules
        .lines()
        .map(|line| {
            let (id, body) = line.split2(": ");
            let id = id.parse().unwrap();
            let body = if body.starts_with('"') {
                Rule::Atomic(body.chars().skip(1).next().unwrap())
            } else {
                Rule::Composite(
                    body.split(" | ")
                        .map(|option| {
                            option
                                .split(" ")
                                .map(|sub_rule| sub_rule.parse().unwrap())
                                .collect()
                        })
                        .collect(),
                )
            };
            (id, body)
        })
        .collect();

    let strings = strings.lines().map(|s| s.to_string()).collect();

    (rules, strings)
}

fn expand_helper(
    rules: &HashMap<i64, Rule>,
    id: i64,
    seen: &mut HashMap<i64, String>,
) {
    let expanded = match &rules[&id] {
        &Rule::Atomic(c) => c.to_string(),
        Rule::Composite(options) => {
            let expanded = options
                .iter()
                .map(|option| {
                    option
                        .iter()
                        // TODO: Use itertools' dedup_with_count when it's available
                        .chain(std::iter::once(&-1))
                        .scan((0, option[0]), |prev, &id| {
                            if prev.1 != id {
                                let tmp = prev.clone();
                                *prev = (1, id);
                                Some(Some(tmp))
                            } else {
                                prev.0 += 1;
                                Some(None)
                            }
                        })
                        .filter_map(|x| x)
                        .map(|(count, id)| {
                            if !seen.contains_key(&id) {
                                expand_helper(rules, id, seen);
                            }
                            let s = seen[&id].clone();
                            if count >= 2 {
                                let len = s.chars().count();
                                let full_len = count * len;
                                let paren_needed = len >= 2 && !s.starts_with('(');
                                let abbr_len = len + 3 + if paren_needed { 2 } else { 0 };
                                if abbr_len < full_len {
                                    if paren_needed {
                                        format!("({}){{{}}}", s, count)
                                    } else {
                                        format!("{}{{{}}}", s, count)
                                    }
                                } else {
                                    std::iter::repeat(s).take(count).join("")
                                }
                            } else {
                                s
                            }
                        })
                        .join("")
                })
                .join("|");

            if options.len() > 1 {
                format!("({})", expanded)
            } else {
                expanded
            }
        }
    };
    seen.insert(id, expanded);
}

fn expand(rules: &HashMap<i64, Rule>, id: i64) -> String {
    let mut seen = hashmap! {};
    expand_helper(rules, id, &mut seen);
    seen.remove(&id).unwrap()
}

pub fn part1(input: &(HashMap<i64, Rule>, Vec<String>)) -> impl Debug {
    let (rules, strings) = input;

    let re = Regex::new(&format!("^{}$", expand(rules, 0))).unwrap();
    strings.iter().filter(|s| re.is_match(s)).count()
}

pub fn part2(input: &(HashMap<i64, Rule>, Vec<String>)) -> impl Debug {
    let (rules, strings) = input;
    let mut rules = rules.clone();
    let max_recursion_depth = 5;

    let mut options = vec![];
    for depth in 0..max_recursion_depth + 1 {
        options.push(vec![42; depth + 1]);
    }
    rules.insert(8, Rule::Composite(options));

    options = vec![];
    for depth in 0..max_recursion_depth + 1 {
        let mut seq = vec![42; depth + 1];
        seq.extend(vec![31; depth + 1]);
        options.push(seq);
    }
    rules.insert(11, Rule::Composite(options));

    let re = Regex::new(&format!("^{}$", expand(&rules, 0))).unwrap();
    strings.iter().filter(|s| re.is_match(s)).count()
}
