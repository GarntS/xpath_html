/* file:    lib.rs
 * author:  garnt
 * date:    09/10/2024
 * desc:    Entrypoint for the xpath library.
*/

pub mod typer;
pub mod xpath_object;
mod xpath_parser;

// TODO(garnt): implement better tests once things stabilize
/// brief, high-level functional tests
#[cfg(test)]
mod test {
    use crate::xpath_object::XPath;

    #[test]
    fn basic_functional_test() {
        let test_str = "div[contains(@class, 'id-AuthorList') or @somethin]";
        let xpath_object = XPath::parse_from_str(test_str).unwrap();
        println!("obj: {:#?}", &xpath_object);

        /*let torture_test_str = r#"//meta[@name="PUD"]/@content"#;
        let torture_xpath_object = XPath::parse_from_str(torture_test_str).unwrap();
        println!("torture_obj: {:#?}", &torture_xpath_object);*/
    }
}
