use crate::select::attribute_condition::AttributeCondition;
use crate::select::attribute_operator::AttributeOperator;

pub fn attribute_matches(condition: &AttributeCondition, actual: Option<&str>) -> bool {
    let Some(actual) = actual else {
        return false;
    };
    if condition.operator == AttributeOperator::Exists {
        return true;
    }
    let (actual, wanted) = if condition.ignores_case {
        (
            actual.to_ascii_lowercase(),
            condition.value.to_ascii_lowercase(),
        )
    } else {
        (actual.to_owned(), condition.value.to_string())
    };
    match condition.operator {
        AttributeOperator::Exists => true,
        AttributeOperator::Equals => actual == wanted,
        AttributeOperator::Includes => {
            !wanted.is_empty() && actual.split_whitespace().any(|item| item == wanted)
        }
        AttributeOperator::DashMatch => {
            actual == wanted || actual.starts_with(&format!("{wanted}-"))
        }
        AttributeOperator::Prefix => !wanted.is_empty() && actual.starts_with(&wanted),
        AttributeOperator::Suffix => !wanted.is_empty() && actual.ends_with(&wanted),
        AttributeOperator::Substring => !wanted.is_empty() && actual.contains(&wanted),
    }
}
