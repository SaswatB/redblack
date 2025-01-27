// region: 21
/**
 * Internally, we represent paths as strings with '/' as the directory separator.
 * When we make system calls (eg: LanguageServiceHost.getDirectory()),
 * we expect the host to correctly handle paths in our specified format.
 *
 * @internal
 */
pub const directorySeparator: &str = "/";
/** @internal */
pub const altDirectorySeparator: &str = "\\";
const urlSchemeSeparator: &str = "://";
const backslashRegExp: &str = "\\/g";

//// Path Tests

/**
 * Determines whether a charCode corresponds to `/` or `\`.
 *
 * @internal
 */
pub fn isAnyDirectorySeparator(charCode: u32) -> bool { return charCode == b'/' as u32 || charCode == b'\\' as u32; }

/**
 * Determines whether a path starts with a URL scheme (e.g. starts with `http://`, `ftp://`, `file://`, etc.).
 *
 * @internal
 */
pub fn isUrl(path: &str) -> bool {
    return match getEncodedRootLength(path) {
        RootLength::Url(_) => true,
        _ => false,
    };
}

/**
 * Determines whether a path is an absolute disk path (e.g. starts with `/`, or a dos path
 * like `c:`, `c:\` or `c:/`).
 *
 * @internal
 */
pub fn isRootedDiskPath(path: &str) -> bool {
    return match getEncodedRootLength(path) {
        RootLength::RootedDiskPath(_) => true,
        _ => false,
    };
}

/**
 * Determines whether a path consists only of a path root.
 *
 * @internal
 */
pub fn isDiskPathRoot(path: &str) -> bool {
    match getEncodedRootLength(path) {
        RootLength::RootedDiskPath(rootLength) => rootLength > 0 && rootLength == path.len(),
        _ => false,
    }
}

/**
 * Determines whether a path starts with an absolute path component (i.e. `/`, `c:/`, `file://`, etc.).
 *
 * ```ts
 * // POSIX
 * pathIsAbsolute("/path/to/file.ext") === true
 * // DOS
 * pathIsAbsolute("c:/path/to/file.ext") === true
 * // URL
 * pathIsAbsolute("file:///path/to/file.ext") === true
 * // Non-absolute
 * pathIsAbsolute("path/to/file.ext") === false
 * pathIsAbsolute("./path/to/file.ext") === false
 * ```
 *
 * @internal
 */
pub fn pathIsAbsolute(path: &str) -> bool {
    return match getEncodedRootLength(path) {
        RootLength::Unknown => false,
        _ => true,
    };
}

// endregion: 93

// region: 119
/** @internal */
pub fn fileExtensionIs(path: &str, extension: &str) -> bool { return path.len() > extension.len() && path.ends_with(extension); }

/** @internal */
pub fn fileExtensionIsOneOf(path: &str, extensions: Vec<&str>) -> bool {
    for extension in extensions {
        if fileExtensionIs(path, extension) {
            return true;
        }
    }

    return false;
}
/**
 * Determines whether a path has a trailing separator (`/` or `\\`).
 *
 * @internal
 */
pub fn hasTrailingDirectorySeparator(path: &str) -> bool { return path.len() > 0 && isAnyDirectorySeparator(path.chars().last().unwrap() as u32); }

//// Path Parsing

pub fn isVolumeCharacter(charCode: u32) -> bool { (charCode >= b'a' as u32 && charCode <= b'z' as u32) || (charCode >= b'A' as u32 && charCode <= b'Z' as u32) }

pub fn getFileUrlVolumeSeparatorEnd(url: &str, start: usize) -> Option<usize> {
    let ch0 = url.chars().nth(start).unwrap() as u32;
    if ch0 == b':' as u32 {
        return Some(start + 1);
    }
    if ch0 == b'%' as u32 && url.chars().nth(start + 1).map_or(false, |c| (c as u32) == b'3' as u32) {
        let ch2 = url.chars().nth(start + 2).unwrap() as u32;
        if ch2 == b'a' as u32 || ch2 == b'A' as u32 {
            return Some(start + 3);
        }
    }
    None
}

#[derive(Debug, PartialEq, Eq)]
enum RootLength {
    RootedDiskPath(usize),
    Url(usize),
    Unknown,
}

/**
 * Returns length of the root part of a path or URL (i.e. length of "/", "x:/", "//server/share/, file:///user/files").
 * If the root is part of a URL, the twos-complement of the root length is returned.
 */
fn getEncodedRootLength(path: &str) -> RootLength {
    if path.is_empty() {
        return RootLength::Unknown;
    }
    let ch0 = path.chars().next().unwrap() as u32;
    // POSIX or UNC
    if ch0 == b'/' as u32 || ch0 == b'\\' as u32 {
        if path.chars().nth(1).map_or(false, |c| (c as u32) != ch0) {
            return RootLength::RootedDiskPath(1); // POSIX: "/" (or non-normalized "\")
        }

        let p1 = path[2..].find(if ch0 == b'/' as u32 { directorySeparator } else { altDirectorySeparator }).map(|i| i + 2);
        if p1.is_none() {
            return RootLength::RootedDiskPath(path.len()); // UNC: "//server" or "\\server"
        }

        return RootLength::RootedDiskPath(p1.unwrap() + 1); // UNC: "//server/" or "\\server\"
    }

    // DOS
    if isVolumeCharacter(ch0) && path.chars().nth(1).map_or(false, |c| (c as u32) == b':' as u32) {
        if let Some(ch2) = path.chars().nth(2) {
            if (ch2 as u32) == b'/' as u32 || (ch2 as u32) == b'\\' as u32 {
                return RootLength::RootedDiskPath(3); // DOS: "c:/" or "c:\"
            }
        }
        if path.len() == 2 {
            return RootLength::RootedDiskPath(2); // DOS: "c:" (but not "c:d")
        }
    }

    // URL
    if let Some(scheme_end) = path.find(urlSchemeSeparator) {
        let authority_start = scheme_end + urlSchemeSeparator.len();
        if let Some(authority_end) = path[authority_start..].find(directorySeparator).map(|i| i + authority_start) {
            // URL: "file:///", "file://server/", "file://server/path"
            // For local "file" URLs, include the leading DOS volume (if present).
            // Per https://www.ietf.org/rfc/rfc1738.txt, a host of "" or "localhost" is a
            // special case interpreted as "the machine from which the URL is being interpreted".
            let scheme = &path[..scheme_end];
            let authority = &path[authority_start..authority_end];
            if scheme == "file" && (authority.is_empty() || authority == "localhost") {
                if let Some(ch) = path.chars().nth(authority_end + 1) {
                    if isVolumeCharacter(ch as u32) {
                        if let Some(volume_separator_end) = getFileUrlVolumeSeparatorEnd(path, authority_end + 2) {
                            if path.chars().nth(volume_separator_end).map_or(false, |c| (c as u32) == b'/' as u32) {
                                // URL: "file:///c:/", "file://localhost/c:/", "file:///c%3a/", "file://localhost/c%3a/"
                                return RootLength::Url(volume_separator_end + 1);
                            }
                            if volume_separator_end == path.len() {
                                // URL: "file:///c:", "file://localhost/c:", "file:///c$3a", "file://localhost/c%3a"
                                // but not "file:///c:d" or "file:///c%3ad"
                                return RootLength::Url(volume_separator_end);
                            }
                        }
                    }
                }
            }
            return RootLength::Url(authority_end + 1); // URL: "file://server/", "http://server/"
        }
        return RootLength::Url(path.len()); // URL: "file://server", "http://server"
    }

    // relative
    RootLength::Unknown
}

/**
 * Returns length of the root part of a path or URL (i.e. length of "/", "x:/", "//server/share/, file:///user/files").
 *
 * For example:
 * ```ts
 * getRootLength("a") === 0                   // ""
 * getRootLength("/") === 1                   // "/"
 * getRootLength("c:") === 2                  // "c:"
 * getRootLength("c:d") === 0                 // ""
 * getRootLength("c:/") === 3                 // "c:/"
 * getRootLength("c:\\") === 3                // "c:\\"
 * getRootLength("//server") === 7            // "//server"
 * getRootLength("//server/share") === 8      // "//server/"
 * getRootLength("\\\\server") === 7          // "\\\\server"
 * getRootLength("\\\\server\\share") === 8   // "\\\\server\\"
 * getRootLength("file:///path") === 8        // "file:///"
 * getRootLength("file:///c:") === 10         // "file:///c:"
 * getRootLength("file:///c:d") === 8         // "file:///"
 * getRootLength("file:///c:/path") === 11    // "file:///c:/"
 * getRootLength("file://server") === 13      // "file://server"
 * getRootLength("file://server/path") === 14 // "file://server/"
 * getRootLength("http://server") === 13      // "http://server"
 * getRootLength("http://server/path") === 14 // "http://server/"
 * ```
 *
 * @internal
 */
pub fn getRootLength(path: &str) -> usize {
    match getEncodedRootLength(path) {
        RootLength::RootedDiskPath(length) => length,
        RootLength::Url(length) => length,
        RootLength::Unknown => 0,
    }
}

/**
 * Returns the path except for its basename. Semantics align with NodeJS's `path.dirname`
 * except that we support URLs as well.
 *
 * ```ts
 * // POSIX
 * getDirectoryPath("/path/to/file.ext") === "/path/to"
 * getDirectoryPath("/path/to/") === "/path"
 * getDirectoryPath("/") === "/"
 * // DOS
 * getDirectoryPath("c:/path/to/file.ext") === "c:/path/to"
 * getDirectoryPath("c:/path/to/") === "c:/path"
 * getDirectoryPath("c:/") === "c:/"
 * getDirectoryPath("c:") === "c:"
 * // URL
 * getDirectoryPath("http://typescriptlang.org/path/to/file.ext") === "http://typescriptlang.org/path/to"
 * getDirectoryPath("http://typescriptlang.org/path/to") === "http://typescriptlang.org/path"
 * getDirectoryPath("http://typescriptlang.org/") === "http://typescriptlang.org/"
 * getDirectoryPath("http://typescriptlang.org") === "http://typescriptlang.org"
 * ```
 *
 * @internal
 */
// export function getDirectoryPath(path: Path): Path;
/**
 * Returns the path except for its basename. Semantics align with NodeJS's `path.dirname`
 * except that we support URLs as well.
 *
 * ```ts
 * // POSIX
 * getDirectoryPath("/path/to/file.ext") === "/path/to"
 * getDirectoryPath("/path/to/") === "/path"
 * getDirectoryPath("/") === "/"
 * // DOS
 * getDirectoryPath("c:/path/to/file.ext") === "c:/path/to"
 * getDirectoryPath("c:/path/to/") === "c:/path"
 * getDirectoryPath("c:/") === "c:/"
 * getDirectoryPath("c:") === "c:"
 * // URL
 * getDirectoryPath("http://typescriptlang.org/path/to/file.ext") === "http://typescriptlang.org/path/to"
 * getDirectoryPath("http://typescriptlang.org/path/to") === "http://typescriptlang.org/path"
 * getDirectoryPath("http://typescriptlang.org/") === "http://typescriptlang.org/"
 * getDirectoryPath("http://typescriptlang.org") === "http://typescriptlang.org"
 * getDirectoryPath("file://server/path/to/file.ext") === "file://server/path/to"
 * getDirectoryPath("file://server/path/to") === "file://server/path"
 * getDirectoryPath("file://server/") === "file://server/"
 * getDirectoryPath("file://server") === "file://server"
 * getDirectoryPath("file:///path/to/file.ext") === "file:///path/to"
 * getDirectoryPath("file:///path/to") === "file:///path"
 * getDirectoryPath("file:///") === "file:///"
 * getDirectoryPath("file://") === "file://"
 * ```
 *
 * @internal
 */
pub fn getDirectoryPath(path_arg: &str) -> String {
    let path = normalizeSlashes(path_arg);

    // If the path provided is itself the root, then return it.
    let root_length = getRootLength(&path);
    if root_length == path.len() {
        return path;
    }

    // return the leading portion of the path up to the last (non-terminal) directory separator
    // but not including any trailing directory separator.
    let path = removeTrailingDirectorySeparator(&path);
    let last_sep = path[root_length..].rfind(directorySeparator).map(|i| i + root_length);
    match last_sep {
        Some(i) => path[..i].to_string(),
        None => path[..root_length].to_string(),
    }
}
// endregion: 323

// region: 519
//// Path Normalization

/**
 * Normalize path separators, converting `\` into `/`.
 *
 * @internal
 */
pub fn normalizeSlashes(path: &str) -> String {
    if path.contains("\\") {
        path.replace("\\", directorySeparator)
    } else {
        path.to_string()
    }
}
// endregion: 530

// region: 559
/**
 * Combines paths. If a path is absolute, it replaces any previous path. Relative paths are not simplified.
 *
 * ```ts
 * // Non-rooted
 * combinePaths("path", "to", "file.ext") === "path/to/file.ext"
 * combinePaths("path", "dir", "..", "to", "file.ext") === "path/dir/../to/file.ext"
 * // POSIX
 * combinePaths("/path", "to", "file.ext") === "/path/to/file.ext"
 * combinePaths("/path", "/to", "file.ext") === "/to/file.ext"
 * // DOS
 * combinePaths("c:/path", "to", "file.ext") === "c:/path/to/file.ext"
 * combinePaths("c:/path", "c:/to", "file.ext") === "c:/to/file.ext"
 * // URL
 * combinePaths("file:///path", "to", "file.ext") === "file:///path/to/file.ext"
 * combinePaths("file:///path", "file:///to", "file.ext") === "file:///to/file.ext"
 * ```
 *
 * @internal
 */
pub fn combinePaths(path: &str, paths: &[Option<&str>]) -> String {
    let mut result = if !path.is_empty() { normalizeSlashes(path) } else { String::new() };

    for relative_path in paths {
        if let Some(relative_path) = relative_path {
            let normalized_relative = normalizeSlashes(relative_path);
            if result.is_empty() || getRootLength(&normalized_relative) != 0 {
                result = normalized_relative;
            } else {
                result = format!("{}{}", ensureTrailingDirectorySeparator(&result), normalized_relative);
            }
        }
    }

    result
}
// endregion: 592

// region: 669
//// Path Mutation

/**
 * Removes a trailing directory separator from a path, if it does not already have one.
 *
 * ```ts
 * removeTrailingDirectorySeparator("/path/to/file.ext") === "/path/to/file.ext"
 * removeTrailingDirectorySeparator("/path/to/file.ext/") === "/path/to/file.ext"
 * ```
 *
 * @internal
 */
pub fn removeTrailingDirectorySeparator(path: &str) -> String {
    if hasTrailingDirectorySeparator(path) {
        return path[..path.len() - 1].to_string();
    }

    path.to_string()
}

/**
 * Adds a trailing directory separator to a path, if it does not already have one.
 *
 * ```ts
 * ensureTrailingDirectorySeparator("/path/to/file.ext") === "/path/to/file.ext/"
 * ensureTrailingDirectorySeparator("/path/to/file.ext/") === "/path/to/file.ext/"
 * ```
 *
 * @internal
 */
pub fn ensureTrailingDirectorySeparator(path: &str) -> String {
    if !hasTrailingDirectorySeparator(path) {
        return format!("{}{}", path, directorySeparator);
    }

    path.to_string()
}
// endregion: 713

// region: 985

//// Path Traversal
/**
 * Calls `callback` on `directory` and every ancestor directory it has, returning the first defined result.
 *
 * @internal
 */
pub fn forEachAncestorDirectory<T>(directory: &str, mut callback: impl FnMut(&str) -> Option<T>) -> Option<T> {
    let mut current_dir = directory.to_string();
    loop {
        if let Some(result) = callback(&current_dir) {
            return Some(result);
        }

        let parent_path = getDirectoryPath(&current_dir);
        if parent_path == current_dir {
            return None;
        }

        current_dir = parent_path;
    }
}

/** @internal */
pub fn isNodeModulesDirectory(dir_path: &str) -> bool { dir_path.ends_with("/node_modules") }

// endregion: eof
