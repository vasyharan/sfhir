/// Any combination of letters, numerals, "-" and ".", with a length limit of 64
/// characters.  (This might be an integer, an unprefixed OID, UUID or any other
/// identifier pattern that meets these constraints.)  Ids are case-insensitive.
#[derive(Debug, Clone, PartialEq)]
pub struct Id(String);