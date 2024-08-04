use pest::iterators::{Pair, Pairs};

use crate::{
    ast::Input,
    grammar::Rule,
};

use super::{
    assertions::{assert_as_rule_eq, checked_take_last_some, require_some},
    parse_result::ParseResult,
};

pub fn next<T>(input: &Input, inner: &mut Pairs<Rule>, parse: impl Fn(&Input, Pair<Rule>) -> ParseResult<T>) -> ParseResult<T> {
    let pair = require_some!(inner.next());
    let model = parse(input, pair)?;
    Ok(model)
}

pub fn last<T>(input: &Input, inner: Pairs<Rule>, parse: impl Fn(&Input, Pair<Rule>) -> ParseResult<T>) -> ParseResult<T> {
    let pair = checked_take_last_some!(inner);
    let model = parse(input, pair)?;
    Ok(model)
}

pub fn homogeneous_remainder<T>(
    input: &Input,
    inner: Pairs<Rule>,
    parse: impl Fn(&Input, Pair<Rule>) -> ParseResult<T>,
    rule: Rule,
) -> ParseResult<Vec<T>> {
    let mut parsed = Vec::new();

    for pair in inner {
        assert_as_rule_eq!(pair, rule);
        let model = parse(input, pair)?;
        parsed.push(model);
    }

    Ok(parsed)
}

pub fn homogeneous_run<T>(
    input: &Input,
    inner: &mut Pairs<Rule>,
    parse: impl Fn(&Input, Pair<Rule>) -> ParseResult<T>,
    rule: Rule,
) -> ParseResult<Vec<T>> {
    let mut parsed = Vec::new();

    loop {
        match inner.peek() {
            Some(pair) => {
                if pair.as_rule() == rule {
                    let _ = inner.next();
                    let model = parse(input, pair)?;
                    parsed.push(model);
                } else {
                    break;
                }
            }
            None => {
                break;
            }
        }
    }

    Ok(parsed)
}
