/// A string which has at least one character and no leading or trailing whitespace
/// and where there is no whitespace other than single spaces in the contents
#[derive(Debug, Clone, PartialEq)]
pub struct Code(String);