use crate::prelude::*;

use std::ops::RangeInclusive;

type Input = (
    HashMap<String, Vec<RangeInclusive<i64>>>,
    Vec<i64>,
    Vec<Vec<i64>>,
);

pub fn parse(input: &str) -> Input {
    let (fields, input) = input.split2("\n\nyour ticket:\n");
    let (ticket, nearby_tickets) = input.split2("\n\nnearby tickets:\n");
    let fields = fields
        .lines()
        .map(|line| {
            let (key, value) = line.split2(": ");
            (
                key,
                value
                    .split(" or ")
                    .map(|range| {
                        let (lo, hi) = range.split2("-");
                        lo.parse().unwrap()..=hi.parse().unwrap()
                    })
                    .collect(),
            )
        })
        .collect();
    let ticket = ticket
        .trim()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();
    let nearby_tickets = nearby_tickets
        .lines()
        .map(|line| {
            line.trim().split(',').map(|x| x.parse().unwrap()).collect()
        })
        .collect();

    (fields, ticket, nearby_tickets)
}

pub fn part1(input: &Input) -> impl Debug {
    let (fields, _, nearby_tickets) = input;

    nearby_tickets
        .iter()
        .map(|ticket| {
            ticket
                .iter()
                .filter(|x| {
                    fields
                        .values()
                        .all(|rule| rule.iter().all(|range| !range.contains(x)))
                })
                .sum::<i64>()
        })
        .sum::<i64>()
}

pub fn part2(input: &Input) -> impl Debug {
    let (fields, ticket, nearby_tickets) = input;

    let valid_tickets: Vec<_> = nearby_tickets
        .iter()
        .filter(|ticket| {
            ticket.iter().all(|x| {
                fields
                    .values()
                    .any(|rule| rule.iter().any(|range| range.contains(x)))
            })
        })
        .collect();

    let mut potential_columns: Vec<(_, Vec<_>)> = fields
        .iter()
        .map(|(name, rule)| {
            let options = (0..fields.len())
                .filter(|&i| {
                    valid_tickets.iter().all(|ticket| {
                        rule.iter().any(|range| range.contains(&ticket[i]))
                    })
                })
                .collect();
            (name, options)
        })
        .collect();
    potential_columns.sort_unstable_by_key(|(_, options)| options.len());

    let mut columns = hashmap! {};
    let mut seen = hashset! {};
    for (name, mut options) in potential_columns.drain(..) {
        options.retain(|option| !seen.contains(option));
        assert!(options.len() == 1);

        let column = options.iter().next().copied().unwrap();
        columns.insert(name.to_string(), column);
        seen.insert(column);
    }

    columns
        .iter()
        .filter(|(name, _)| name.starts_with("departure"))
        .map(|(_, &column)| ticket[column])
        .product::<i64>()
}
