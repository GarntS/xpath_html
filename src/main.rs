/* file:    main.rs
 * author:  garnt
 * date:    09/10/2024
 * desc:    Entrypoint for the xpath_html library.
*/

mod xpath_object;
mod xpath_parser;

use xpath_object::XPath;

/// main() is the entrypoint
fn main() {
    let test_str = "//div[contains(@class, 'id-AuthorList') and last()]/descendant::*[contains(@class, 'id-Link') or position() <=2] | //ul/child::li";
    let xpath_object = XPath::parse_from_str(test_str).unwrap();
    println!("obj: {:#?}", &xpath_object);
}
