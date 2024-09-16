/* file:    xpath_parser.rs
 * author:  garnt
 * date:    09/10/2024
 * desc:    XPath parser implementation using Pest.
*/

use pest_derive::Parser;

/// XPath parser implemented with pest_derive
#[derive(Parser)]
#[grammar = "xpath_grammar.pest"]
pub struct XPathParser;

// pest Tests
#[cfg(test)]
use pest_test_gen::pest_tests;

#[pest_tests(crate::xpath_parser::XPathParser, crate::xpath_parser::Rule, "xpath")]
#[cfg(test)]
mod xpath_parser_tests {}
