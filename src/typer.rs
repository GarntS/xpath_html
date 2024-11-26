/* file:    typer.rs
 * author:  garnt
 * date:    10/29/2024
 * desc:    XPath (and XML/HTML) types and typer traits.
*/

/// Represents a type within XPath's evaluation
pub enum XPathType {
    Boolean,
    Integer,
    Decimal,
    Float,
    Double,
    String,
    Sequence,
    None,
}

/// Trait allowing a struct to represent an output type
pub trait XPathTypedOutput {
    /// Returns the output type for this object
    fn get_output_type(&self) -> XPathType;
}

/// Struct representing an HTML/XML QName
//  spec here: https://www.w3.org/TR/REC-xml-names/#NT-QName
struct QName {
    pub prefix: String,
    pub local_part: String,
}

impl QName {
    /// Returns a new, empty, QName
    pub fn new() -> Self {
        QName {
            prefix: String::new(),
            local_part: String::new(),
        }
    }
}

/// Enum representing an XPath EQName, which is either a QName or a
/// URIQualifiedName, which we can just treat as a String.
enum EQName {
    QName(QName),
    URIQualifiedName(String),
}
