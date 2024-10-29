/* file:    xpath_object.rs
 * author:  garnt
 * date:    09/10/2024
 * desc:    XPath object representation.
*/

use crate::xpath_parser;
use pest::Parser;
use std::error::Error;
use std::str::FromStr;

/// Pretty-prints a pest::Pair
fn pretty_print_pair<R: pest::RuleType>(pair: &pest::iterators::Pair<R>, depth: Option<usize>) {
    let depth: usize = depth.unwrap_or_default();
    println!(
        "{}{:?}: {}",
        "\t".repeat(depth),
        pair.as_rule(),
        pair.as_str()
    );
    for sub_pair in pair.clone().into_inner() {
        pretty_print_pair(&sub_pair, Some(depth + 1));
    }
}

/// The type of data returned by evaluating an expression
#[derive(Debug, PartialEq)]
pub enum ExpressionType {
    String,
    Elements,
    InvalidExpressionType,
}

/// Represents the different axes that can be evaluated against
#[derive(Debug, PartialEq)]
pub enum AxisType {
    Ancestor,
    AncestorOrSelf,
    Attribute,
    Child,
    Descendant,
    DescendantOrSelf,
    Namespace,
    SelfAxis,
    Parent,
    Following,
    FollowingSibling,
    Preceding,
    PrecedingSibling,
    InvalidAxisType,
}

/// std::str::FromStr implementation for AxisType
impl FromStr for AxisType {
    type Err = ();

    fn from_str(input: &str) -> Result<AxisType, Self::Err> {
        match input {
            "ancestor" => Ok(AxisType::Ancestor),
            "ancestor-or-self" => Ok(AxisType::AncestorOrSelf),
            "attribute" => Ok(AxisType::Attribute),
            "child" => Ok(AxisType::Child),
            "descendant" => Ok(AxisType::Descendant),
            "descendant-or-self" => Ok(AxisType::DescendantOrSelf),
            "namespace" => Ok(AxisType::Namespace),
            "self" => Ok(AxisType::SelfAxis),
            "parent" => Ok(AxisType::Parent),
            "following" => Ok(AxisType::Following),
            "following-sibling" => Ok(AxisType::FollowingSibling),
            "preceding" => Ok(AxisType::Preceding),
            "preceding-sibling" => Ok(AxisType::PrecedingSibling),
            _ => Err(()),
        }
    }
}

/// Struct containing a parsed/evaluated xpath object
#[derive(Debug)]
pub struct XPath {
    // TODO(garnt): implement this properly
    /// The type that this XPath will evaluate to
    output_type: ExpressionType,
}

// implementation for XPath
impl XPath {
    /// Returns a new, empty, XPath
    fn new() -> Self {
        XPath {
            output_type: ExpressionType::InvalidExpressionType,
        }
    }

    /// Parses an XPath object from the provided string
    pub fn parse_from_str(xpath_str: &str) -> Result<Self, Box<dyn Error>> {
        let mut parsed_pairs =
            xpath_parser::XPathParser::parse(xpath_parser::Rule::xpath, xpath_str)?;
        let base_pair = parsed_pairs
            .next()
            .expect("Parsed pairs contained no parsed XPaths!");

        // print our pair
        pretty_print_pair(&base_pair, None);

        // check we're parsing an xpath
        if base_pair.as_rule() != xpath_parser::Rule::xpath {
            return Err("Pair isn't an XPath!".into());
        }

        let mut out_xpath = XPath::new();

        // iterate over all the child nodes of expr_pair
        /*
        for sub_pair in base_pair.into_inner() {
            if sub_pair.as_rule() == xpath_parser::Rule::xpath_expression {
                out_xpath
                    .expressions
                    .push(XPathExpression::parse_from_pair(sub_pair)?);
            }
        }*/

        // return the parsed XPath
        Ok(out_xpath)
    }
}
