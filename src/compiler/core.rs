// region: 1939
/**
 * Compare the equality of two strings using a case-sensitive ordinal comparison.
 *
 * Case-sensitive comparisons compare both strings one code-point at a time using the integer
 * value of each code-point after applying `toUpperCase` to each string. We always map both
 * strings to their upper-case form as some unicode characters do not properly round-trip to
 * lowercase (such as `ẞ` (German sharp capital s)).
 *
 * @internal
 */
pub fn equateStringsCaseInsensitive(a: &str, b: &str) -> bool { a == b || (!a.is_empty() && !b.is_empty() && a.to_uppercase() == b.to_uppercase()) }
// endregion: 1956

// region: 2425
/** @internal */
pub fn startsWith(str: &str, prefix: &str, ignoreCase: Option<bool>) -> bool {
    match ignoreCase {
        Some(true) => equateStringsCaseInsensitive(&str[0..prefix.len().min(str.len())], prefix),
        _ => str.starts_with(prefix),
    }
}
// endregion: 2432
