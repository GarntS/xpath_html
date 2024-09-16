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
pub struct XPath<'a> {
    /// The type that this XPath will evaluate to
    output_type: ExpressionType,
    /// A Vec containing all the individual expressions in the XPath, in the event there's more than one
    expressions: Vec<XPathExpression<'a>>,
}

/// A single XPath expression
#[derive(Debug)]
struct XPathExpression<'a> {
    /// A Vec containing pairs of (Axis, Step), in order they should be evaluated
    axis_step_pairs: Vec<(Axis, Step<'a>)>,
}

/// Struct containing all the relevant info about an XPath axis
#[derive(Debug)]
struct Axis {
    /// The AxisType for this axis
    axis_type: AxisType,
    //context: EvalContext,
}

/// Struct containing all the relevant info about an XPath step
#[derive(Debug)]
struct Step<'a> {
    tag_is_wildcard: bool,
    tag_identifier: &'a str,
    selectors: Vec<Selector>,
}

/// Struct containing all the relevant info about an XPath step
#[derive(Debug)]
struct Selector {
    clauses: Vec<SelectorClause>,
}

/// Enum representing a single parsed literal from the XPath
#[derive(Debug, PartialEq)]
enum XPathLiteral<'a> {
    String(&'a str),
    UInt(u64),
}

// implementation for XPath
impl XPath<'_> {
    /// Returns a new, empty, XPath
    fn new() -> Self {
        XPath {
            output_type: ExpressionType::InvalidExpressionType,
            expressions: Vec::new(),
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
        for sub_pair in base_pair.into_inner() {
            if sub_pair.as_rule() == xpath_parser::Rule::xpath_expression {
                out_xpath
                    .expressions
                    .push(XPathExpression::parse_from_pair(sub_pair)?);
            }
        }

        // return the parsed XPath
        Ok(out_xpath)
    }
}

// implementation for Expression
impl XPathExpression<'_> {
    /// Returns a new, empty, XPathExpression
    fn new() -> Self {
        XPathExpression {
            axis_step_pairs: Vec::new(),
        }
    }

    /// Parses an XPathExpression from the provided pest::iterators::Pair object
    fn parse_from_pair(
        expr_pair: pest::iterators::Pair<xpath_parser::Rule>,
    ) -> Result<Self, Box<dyn Error>> {
        /// List of valid states for the XPathExpression-parsing state machine
        #[derive(Debug, PartialEq)]
        enum ParserState {
            Start,
            PrevWasAxis,
            PrevWasStep,
        }

        // check we're parsing an xpath_expression
        if expr_pair.as_rule() != xpath_parser::Rule::xpath_expression {
            return Err("Pair isn't an XPathExpression!".into());
        }

        // track the parse state machine's current state
        let mut out_expr: Self = Self::new();
        let mut cur_state: ParserState = ParserState::Start;
        let mut prev_axis: Option<Axis> = None;

        // iterate over all the child nodes of expr_pair
        for sub_pair in expr_pair.into_inner() {
            match sub_pair.as_rule() {
                // if the child is an axis
                xpath_parser::Rule::axis => {
                    // if we got 2 axes in a row, return an error
                    if cur_state == ParserState::PrevWasAxis {
                        return Err("Unexpected matched rule!".into());
                    }

                    // update the state
                    cur_state = ParserState::PrevWasAxis;

                    // actually parse the Axis from this pair, set prev_axis
                    prev_axis = Some(Axis::parse_from_pair(sub_pair)?);
                }
                // if the child is a step
                xpath_parser::Rule::step => {
                    // if we got 2 steps in a row, or this step was first, return an error
                    if cur_state == ParserState::Start || cur_state == ParserState::PrevWasStep {
                        return Err("Unexpected matched rule!".into());
                    }

                    // update the state
                    cur_state = ParserState::PrevWasStep;

                    // actually parse the Step from this pair
                    let step: Step = Step::parse_from_pair(sub_pair)?;

                    // grab the prev_axis
                    let prev_axis: Option<Axis> = std::mem::take(&mut prev_axis);
                    let prev_axis: Axis =
                        prev_axis.expect("prev_axis should always be set when parsing a step!");

                    // insert our new pair into the
                    out_expr.axis_step_pairs.push((prev_axis, step));
                }
                // if the child is anything else, it's unexpected
                _ => return Err("Unexpected matched rule!".into()),
            }
        }

        // return the parsed XPathExpression
        Ok(out_expr)
    }
}

// implementation for Axis
impl Axis {
    /// Returns a new, empty, Axis
    fn new() -> Self {
        Axis {
            axis_type: AxisType::InvalidAxisType,
        }
    }

    /// Parses an Axis from the provided pest::iterators::Pair object
    fn parse_from_pair(
        axis_pair: pest::iterators::Pair<xpath_parser::Rule>,
    ) -> Result<Self, Box<dyn Error>> {
        // check we're parsing an Axis
        if axis_pair.as_rule() != xpath_parser::Rule::axis {
            return Err("Pair isn't an Axis!".into());
        }

        // track the parse state machine's current state
        let mut out_axis: Self = Self::new();
        let axis_str: &str = axis_pair.as_str();

        // check if this pair has a descendent matching the axis_name rule
        let axis_name: Option<&str> = axis_pair
            .into_inner()
            .find(|inner_pair| inner_pair.as_rule() == xpath_parser::Rule::axis_name)
            .map(|pair| pair.as_str());

        // actually set the axis type for this axis
        out_axis.axis_type = if let Some(axis_name) = axis_name {
            AxisType::from_str(axis_name).or(Err("Failed to parse AxisType!"))?
        } else {
            // if there's no explicit axis type, use the implied one
            match axis_str {
                "//" => AxisType::DescendantOrSelf,
                "/" => AxisType::Child,
                _ => return Err("Unexpected axis_pair value!".into()),
            }
        };

        // return the successfully-parsed axis
        Ok(out_axis)
    }
}

// implementation for Step
impl Step<'_> {
    /// Returns a new, empty, Step
    fn new() -> Self {
        Step {
            tag_is_wildcard: false,
            tag_identifier: "",
            selectors: Vec::new(),
        }
    }

    /// Parses a Step from the provided pest::iterators::Pair object
    fn parse_from_pair(
        step_pair: pest::iterators::Pair<xpath_parser::Rule>,
    ) -> Result<Self, Box<dyn Error>> {
        /// List of valid states for the Step-parsing state machine
        #[derive(Debug, PartialEq)]
        enum ParserState {
            Start,
        }

        // check we're parsing a step
        if step_pair.as_rule() != xpath_parser::Rule::step {
            return Err("Pair isn't a Step!".into());
        }

        // track the parse state machine's current state
        let mut out_step: Self = Self::new();
        let mut _cur_state: ParserState = ParserState::Start;

        // TODO(garnt): actually implement this
        //out_step.content = step_pair.as_str().to_owned();
        Ok(out_step)
    }
}
