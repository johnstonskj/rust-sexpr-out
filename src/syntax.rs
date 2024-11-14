/*!
Constants used in the writing of s-expressions.
 */

// ------------------------------------------------------------------------------------------------
// Public Constants
// ------------------------------------------------------------------------------------------------

pub const CHAR_LIST_OPEN: &[u8] = b"(";
pub const CHAR_LIST_CLOSE: &[u8] = b")";
pub const CHAR_SPACE: &[u8] = b" ";
pub const CHAR_NEWLINE: &[u8] = b"\n";

pub const CHAR_QUOTE: &[u8] = b"'";
pub const CHAR_QUASI_QUOTE: &[u8] = b"`";
pub const CHAR_UNQUOTE: &[u8] = b",";
