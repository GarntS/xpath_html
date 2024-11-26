/* file:    string_types.rs
 * author:  garnt
 * date:    10/29/2024
 * desc:    XPath (and XML/HTML) string types.
*/

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
