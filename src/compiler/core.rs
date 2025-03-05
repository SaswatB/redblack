// region: 218
/** @internal */
pub fn contains<T: PartialEq>(array: Option<&[T]>, value: &T, equalityComparer: Option<impl Fn(&T, &T) -> bool>) -> bool {
    if let Some(array) = array {
        for i in 0..array.len() {
            if equalityComparer.as_ref().map(|f| f(&array[i], value)).unwrap_or(equateValues(&array[i], value)) {
                return true;
            }
        }
    }
    false
}
// endregion: 230

// region: 996
/**
 * @return Whether the value was added.
 *
 * @internal
 */
pub fn pushIfUnique<T: Clone + PartialEq>(array: &mut Vec<T>, toAdd: T, equalityComparer: Option<impl Fn(&T, &T) -> bool>) -> bool {
    if contains(Some(array), &toAdd, equalityComparer) {
        return false;
    } else {
        array.push(toAdd);
        return true;
    }
}

/**
 * Unlike `pushIfUnique`, this can take `undefined` as an input, and returns a new array.
 *
 * @internal
 */
pub fn appendIfUnique<T: Clone + PartialEq>(array: Option<&mut Vec<T>>, toAdd: T, equalityComparer: Option<impl Fn(&T, &T) -> bool>) -> Vec<T> {
    if let Some(array) = array {
        pushIfUnique(array, toAdd, equalityComparer);
        array.clone()
    } else {
        vec![toAdd]
    }
}
// endregion: 1026

// region: 1934
/** @internal */
pub fn equateValues<T: PartialEq>(a: &T, b: &T) -> bool { a == b }

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
