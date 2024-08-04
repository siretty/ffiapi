pub(super) const BUG_PREFIX: &'static str = "bug: invalid parse result: check grammar and caller for correctness";

macro_rules! assert_as_rule_eq {
    ($pair:expr, $rule:expr) => {
        assert_eq!(
            $pair.as_rule(), $rule,
            "{}", $crate::parse::assertions::BUG_PREFIX
        )
    };
}

pub(super) use assert_as_rule_eq;

macro_rules! assert_other_rule {
    ($rule:expr) => {
        panic!(
            "{}: rule {:?} could not be matched",
            $crate::parse::assertions::BUG_PREFIX, $rule
        )
    }
}

pub(super) use assert_other_rule;

macro_rules! assert_invalid_identifier {
    ($string:expr) => {
        panic!(
            "{}: {:?} is not a valid identifier",
            $crate::parse::assertions::BUG_PREFIX, $string
        )
    }
}

pub(super) use assert_invalid_identifier;

macro_rules! require_some {
    ($option:expr) => {
        $option.expect(format!("{}", $crate::parse::assertions::BUG_PREFIX).as_str())
    };
}

pub(super) use require_some;

macro_rules! checked_take_last_some {
    ($pairs:expr) => {
        {
            let mut pairs = $pairs;
            let only = require_some!(pairs.next());
            assert_eq!(
                pairs.peek(), None,
                "{}", $crate::parse::assertions::BUG_PREFIX
            );
            only
        }
    }
}

pub(super) use checked_take_last_some;
