#[cfg(test)]
mod tests;

use std::collections::HashMap;

/// Return the end position of the match
pub type Filter = Box<dyn Fn(&Rules, &str) -> Option<usize>>;
pub struct Rules(pub HashMap<i32, Filter>);

/// number of lines parsed + rules
fn parse_rules(inp: &str) -> (usize, Rules) {
    let mut rules = Rules(HashMap::new());

    let mut count = 0;
    for line in inp.lines() {
        count += 1;
        if line.is_empty() {
            break;
        }

        let mut kv = line.split(":");
        let key: i32 = kv.next().unwrap().parse().expect("not a number");

        let val = kv.next().unwrap();
        let or_filters = val
            .split("|")
            .map(|f| {
                let and_filters = f
                    .split(" ")
                    .filter_map(|token| {
                        // one,single rule
                        if let Some(num) = token.parse::<i32>().ok() {
                            // rule is a number, indicating that we want to call another rule
                            let filter = Box::new(move |rules: &Rules, line: &str| {
                                let res = rules.0[&num](rules, line);
                                res
                            });
                            Some(filter as Filter)
                        } else {
                            if let Some(ind) = token.find('"') {
                                let substr = &token[ind + 1..];
                                let substr = substr.chars().next().unwrap();
                                let filter = move |_: &Rules, line: &str| {
                                    if line.starts_with(substr) {
                                        Some(1usize)
                                    } else {
                                        None
                                    }
                                };
                                let filter = Box::new(filter);
                                Some(filter as Filter)
                            } else {
                                None
                            }
                        }
                    })
                    .collect::<Vec<_>>();

                move |rules: &Rules, line: &str| {
                    let mut i = 0;
                    for (_rule_id, rule) in and_filters.iter().enumerate() {
                        let line = &line[i..];
                        if let Some(j) = rule(rules, line) {
                            i += j
                        } else {
                            return None;
                        }
                    }
                    Some(i)
                }
            })
            .collect::<Vec<_>>();

        let filter: Filter =
            Box::new(move |rules, line| or_filters.iter().find_map(|f| f(rules, line)));

        rules.0.insert(key, filter);
    }

    (count, rules)
}

fn run<'a>(rules: &Rules, lines: impl Iterator<Item = &'a str>) -> usize {
    lines
        .filter(|line| {
            rules.0[&0](rules, line)
                .and_then(|i| if i == line.len() { Some(i) } else { None })
                .is_some()
        })
        .map(|line| {
            dbg!(line);
        })
        .count()
}

fn main() {
    let mut input = String::new();
    std::io::Read::read_to_string(&mut std::io::stdin(), &mut input).unwrap();

    let (i, rules) = parse_rules(input.as_str());

    let res = run(&rules, input.lines().skip(i));
    println!("{}", res);
}
