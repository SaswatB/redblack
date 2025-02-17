use super::{diagnostic_information_map_generated::Diagnostics, types::{CharacterCodes, DiagnosticMessage}, utilities::positionIsSynthesized};
use lazy_static::lazy_static;

// region: 549
pub fn isWhiteSpaceLike(ch: char) -> bool {
    isWhiteSpaceSingleLine(ch) || isLineBreak(ch)
}

/** Does not include line breaks. For that, see isWhiteSpaceLike. */
pub fn isWhiteSpaceSingleLine(ch: char) -> bool {
    // Note: nextLine is in the Zs space, and should be considered to be a whitespace.
    // It is explicitly not a line-break as it isn't in the exact set specified by EcmaScript.
    ch == CharacterCodes::space ||
        ch == CharacterCodes::tab ||
        ch == CharacterCodes::verticalTab ||
        ch == CharacterCodes::formFeed ||
        ch == CharacterCodes::nonBreakingSpace ||
        ch == CharacterCodes::nextLine ||
        ch == CharacterCodes::ogham ||
        ch as u32 >= CharacterCodes::enQuad as u32 && ch as u32 <= CharacterCodes::zeroWidthSpace as u32 ||
        ch == CharacterCodes::narrowNoBreakSpace ||
        ch == CharacterCodes::mathematicalSpace ||
        ch == CharacterCodes::ideographicSpace ||
        ch == CharacterCodes::byteOrderMark
}

pub fn isLineBreak(ch: char) -> bool {
    // ES5 7.3:
    // The ECMAScript line terminator characters are listed in Table 3.
    //     Table 3: Line Terminator Characters
    //     Code Unit Value     Name                    Formal Name
    //     \u000A              Line Feed               <LF>
    //     \u000D              Carriage Return         <CR>
    //     \u2028              Line separator          <LS>
    //     \u2029              Paragraph separator     <PS>
    // Only the characters in Table 3 are treated as line terminators. Other new line or line
    // breaking characters are treated as white space but not as line terminators.

    ch == CharacterCodes::lineFeed ||
        ch == CharacterCodes::carriageReturn ||
        ch == CharacterCodes::lineSeparator ||
        ch == CharacterCodes::paragraphSeparator
}
// endregion: 588

// region: 638
/** @internal */
pub fn skipTrivia(text: &str, mut pos: u32, stop_after_line_break: Option<bool>, stop_at_comments: Option<bool>, in_jsdoc: Option<bool>) -> u32 {
    if positionIsSynthesized(pos) {
        return pos;
    }

    let mut can_consume_star = false;
    // Keep in sync with couldStartTrivia
    loop {
        let ch = text.chars().nth(pos as usize).unwrap() as u32;
        match ch {
            // CharacterCodes::carriageReturn
            13 | 
            // CharacterCodes::lineFeed
            10 => {
                if ch == CharacterCodes::carriageReturn {
                  if text.chars().nth((pos + 1) as usize).unwrap() == CharacterCodes::lineFeed {
                      pos += 1;
                  }
                }
                pos += 1;
                if stop_after_line_break.unwrap_or(false) {
                    return pos;
                }
                can_consume_star = in_jsdoc.unwrap_or(false);
                continue;
            }
            // CharacterCodes::tab
            9 | 
            // CharacterCodes::verticalTab
            11 | 
            // CharacterCodes::formFeed
            12 | 
            // CharacterCodes::space
            32 => {
                pos += 1;
                continue;
            }
            // CharacterCodes::slash
            47 => {
                if stop_at_comments.unwrap_or(false) {
                  break;
                }
                if text.chars().nth((pos + 1) as usize).unwrap() == CharacterCodes::slash {
                    pos += 2;
                    while (pos as usize) < text.len() {
                        if isLineBreak(text.chars().nth(pos as usize).unwrap()) {
                            break;
                        }
                        pos += 1;
                    }
                    can_consume_star = false;
                    continue;
                }
                if text.chars().nth((pos + 1) as usize).unwrap() == CharacterCodes::asterisk {
                    pos += 2;
                    while (pos as usize) < text.len() {
                        if text.chars().nth(pos as usize).unwrap() == CharacterCodes::asterisk && text.chars().nth((pos + 1) as usize).unwrap() == CharacterCodes::slash {
                            pos += 2;
                            break;
                        }
                        pos += 1;
                    }
                    can_consume_star = false;
                    continue;
                }
                break;
            }
            // CharacterCodes::lessThan
            60 | 
            // CharacterCodes::bar
            124 |
             // CharacterCodes::equals
            61 | 
            // CharacterCodes::greaterThan
            62 => {
                if isConflictMarkerTrivia(text, pos) {
                    pos = scanConflictMarkerTrivia(text, pos, None);
                    can_consume_star = false;
                    continue;
                }
                break;
            }
            // CharacterCodes::hash
            35 => {
                if pos == 0 && isShebangTrivia(text, pos) {
                    pos = scanShebangTrivia(text, pos);
                    can_consume_star = false;
                    continue;
                }
                break;
            }
            // CharacterCodes::asterisk
            42 => {
                if can_consume_star {
                    pos += 1;
                    can_consume_star = false;
                    continue;
                }
                break;
            }
            _ => {
                if ch > CharacterCodes::maxAsciiCharacter as u32 && isWhiteSpaceLike(ch as u8 as char) {
                    pos += 1;
                    continue;
                }
                break;
            }
        }
    }
    return pos;
}

// All conflict markers consist of the same character repeated seven times.  If it is
// a <<<<<<< or >>>>>>> marker then it is also followed by a space.
const MERGE_CONFLICT_MARKER_LENGTH: u32 = 7; // Length of "<<<<<<<" 

fn isConflictMarkerTrivia(text: &str, pos: u32) -> bool {
    debug_assert!(pos >= 0);

    // Conflict markers must be at the start of a line.
    if pos == 0 || isLineBreak(text.chars().nth((pos - 1) as usize).unwrap() as char) {
        let ch = text.chars().nth(pos as usize).unwrap() as u32;

        if (pos + MERGE_CONFLICT_MARKER_LENGTH) < text.len() as u32 {
            for i in 0..MERGE_CONFLICT_MARKER_LENGTH {
                if text.chars().nth((pos + i) as usize).unwrap() as u32 != ch {
                    return false;
                }
            }

            return ch == CharacterCodes::equals as u32 ||
                text.chars().nth((pos + MERGE_CONFLICT_MARKER_LENGTH) as usize).unwrap() as u32 == CharacterCodes::space as u32;
        }
    }

    false
}

fn scanConflictMarkerTrivia(text: &str, pos: u32, error: Option<fn(DiagnosticMessage, u32, u32)>) -> u32 {
    if let Some(error_fn) = error {
        error_fn(Diagnostics::Merge_conflict_marker_encountered(), pos, MERGE_CONFLICT_MARKER_LENGTH);
    }

    let ch = text.chars().nth(pos as usize).unwrap() as u32;
    let len = text.len() as u32;
    let mut pos = pos;

    if ch == CharacterCodes::lessThan as u32 || ch == CharacterCodes::greaterThan as u32 {
        while pos < len && !isLineBreak(text.chars().nth(pos as usize).unwrap() as char) {
            pos += 1;
        }
        pos
    } else {
        debug_assert!(ch == CharacterCodes::bar as u32 || ch == CharacterCodes::equals as u32);
        // Consume everything from the start of a ||||||| or ======= marker to the start
        // of the next ======= or >>>>>>> marker.
        let mut pos = pos;
        while pos < len {
            let current_char = text.chars().nth(pos as usize).unwrap() as u32;
            if (current_char == CharacterCodes::equals as u32 || current_char == CharacterCodes::greaterThan as u32) && current_char != ch && isConflictMarkerTrivia(text, pos) {
                break;
            }

            pos += 1;
        }
        pos
    }
}


lazy_static! {
    static ref SHEBANG_TRIVIA_REGEX: regex::Regex = regex::Regex::new(r"^#!.*").unwrap();
}

fn isShebangTrivia(text: &str, pos: u32) -> bool {
    // Shebangs check must only be done at the start of the file
    debug_assert!(pos == 0);
    SHEBANG_TRIVIA_REGEX.is_match(text)
}

fn scanShebangTrivia(text: &str, pos: u32) -> u32 {
    let shebang = SHEBANG_TRIVIA_REGEX.find(text).unwrap().as_str();
    pos + shebang.len() as u32
}
// endregion: 803
