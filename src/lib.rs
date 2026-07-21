use std::error::Error;
use yap::{IntoTokens, Tokens};

const SEPS: [char; 5] = [' ', '\t', '\r', '\n', '['];

/// Formats a reference string as markdown.
fn format_ref(str: String) -> String {
    if str.contains("://") {
        format!("[{str}]({str})")
    } else {
        format!("[`{str}`]")
    }
}

/// Extracts the next word token.
fn take_word(toks: &mut impl Tokens<Item = char>) -> String {
    toks.take_while(|&c| !SEPS.into_iter().any(|s| c == s))
        .collect::<String>()
}

/// Skips whitespace tokens.
fn skip_whitespace(toks: &mut impl Tokens<Item = char>) {
    toks.skip_while(|c| c.is_ascii_whitespace());
}

/// Emits a section header if it's not already emitted.
fn emit_section_header(output: &mut Vec<String>, header: &str) {
    if !output.iter().any(|line| line.trim() == header) {

        // inspect previous token, transform single new line into two new lines
        if let Some(last) = output.last_mut() {
            if last == "\n" {
                last.push('\n');
            }
        }

        output.push(header.to_owned());
        output.push("\n\n".to_owned());
    }
}

/// Transforms Doxygen comments into markdown for Rustdoc.
pub fn transform(str: &str) -> Result<String, Box<dyn Error>> {
    let mut res: Vec<String> = vec![];
    let mut toks = str.into_tokens();

    skip_whitespace(&mut toks);
    while let Some(tok) = toks.next() {
        if "@\\".chars().any(|c| c == tok) {
            let tag = take_word(&mut toks);
            skip_whitespace(&mut toks);
            match tag.as_str() {
                "param" => {
                    emit_section_header(&mut res, "# Arguments");
                    let (mut argument, mut attributes) = (take_word(&mut toks), "".to_owned());
                    if argument.is_empty() {
                        if toks.next() != Some('[') {
                            return Err("Expected opening '[' inside attribute list".into());
                        }
                        attributes = toks.take_while(|&c| c != ']').collect::<String>();
                        if toks.next() != Some(']') {
                            return Err("Expected closing ']' inside attribute list".into());
                        }
                        // Escape brackets so rustdoc doesn't read `[in]` as a link.
                        attributes = format!(" \\[{}\\]", attributes);
                        skip_whitespace(&mut toks);
                        argument = take_word(&mut toks);
                    }
                    res.push(format!("* `{}`{} -", argument, attributes));
                }
                "c" | "p" => res.push(format!("`{}`", take_word(&mut toks))),
                "ref" => res.push(format_ref(take_word(&mut toks))),
                "see" | "sa" => {
                    emit_section_header(&mut res, "# See also");
                    res.push(format!("> {}", format_ref(take_word(&mut toks))));
                }
                "a" | "e" | "em" => res.push(format!("_{}_", take_word(&mut toks))),
                "b" => res.push(format!("**{}**", take_word(&mut toks))),
                "note" => res.push("> **Note** ".to_owned()),
                "since" => res.push("> **Since** ".to_owned()),
                "deprecated" => res.push("> **Deprecated** ".to_owned()),
                "remark" | "remarks" => res.push("> ".to_owned()),
                "li" => res.push("- ".to_owned()),
                "par" => res.push("# ".to_owned()),
                "returns" | "return" | "result" => emit_section_header(&mut res, "# Returns"),
                "{" => { /* group start, not implemented  */ }
                "}" => { /* group end, not implemented */ }
                "brief" | "short" => {}
                _ => res.push(format!("{tok}{tag} ")),
            }
        } else if tok == '\n' {
            skip_whitespace(&mut toks);
            res.push(format!("{tok}"));
        } else {
            res.push(format!("{tok}"));
        }
    }
    Ok(res.join(""))
}

#[cfg(test)]
mod tests {
    #[test]
    fn basic() {
        const S: &str = "The FILE_BASIC_INFORMATION structure contains timestamps and basic attributes of a file.\n \\li If you specify a value of zero for any of the XxxTime members, the file system keeps a file's current value for that time.\n \\li If you specify a value of -1 for any of the XxxTime members, time stamp updates are disabled for I/O operations preformed on the file handle.\n\\li If you specify a value of -2 for any of the XxxTime members, time stamp updates are enabled for I/O operations preformed on the file handle.\n\\remarks To set the members of this structure, the caller must have FILE_WRITE_ATTRIBUTES access to the file.";
        const S_: &str = "The FILE_BASIC_INFORMATION structure contains timestamps and basic attributes of a file.\n- If you specify a value of zero for any of the XxxTime members, the file system keeps a file's current value for that time.\n- If you specify a value of -1 for any of the XxxTime members, time stamp updates are disabled for I/O operations preformed on the file handle.\n- If you specify a value of -2 for any of the XxxTime members, time stamp updates are enabled for I/O operations preformed on the file handle.\n> To set the members of this structure, the caller must have FILE_WRITE_ATTRIBUTES access to the file.";
        assert_eq!(crate::transform(S).unwrap(), S_);
    }

    #[test]
    fn with_sections() {
        const S: &str = " The NtDelayExecution routine suspends the current thread until the specified condition is met.\n\n @param Alertable The function returns when either the time-out period has elapsed or when the APC function is called.\n @param DelayInterval The time interval for which execution is to be suspended, in milliseconds.\n - A value of zero causes the thread to relinquish the remainder of its time slice to any other thread that is ready to run.\n - If there are no other threads ready to run, the function returns immediately, and the thread continues execution.\n - A value of INFINITE indicates that the suspension should not time out.\n @return NTSTATUS Successful or errant status. The return value is STATUS_USER_APC when Alertable is TRUE, and the function returned due to one or more I/O completion callback functions.\n @remarks Note that a ready thread is not guaranteed to run immediately. Consequently, the thread will not run until some arbitrary time after the sleep interval elapses,\n based upon the system \"tick\" frequency and the load factor from other processes.\n @see https://learn.microsoft.com/en-us/windows/win32/api/synchapi/nf-synchapi-sleepex";
        const S_: &str = "The NtDelayExecution routine suspends the current thread until the specified condition is met.\n\n# Arguments\n\n* `Alertable` - The function returns when either the time-out period has elapsed or when the APC function is called.\n* `DelayInterval` - The time interval for which execution is to be suspended, in milliseconds.\n- A value of zero causes the thread to relinquish the remainder of its time slice to any other thread that is ready to run.\n- If there are no other threads ready to run, the function returns immediately, and the thread continues execution.\n- A value of INFINITE indicates that the suspension should not time out.\n\n# Returns\n\nNTSTATUS Successful or errant status. The return value is STATUS_USER_APC when Alertable is TRUE, and the function returned due to one or more I/O completion callback functions.\n> Note that a ready thread is not guaranteed to run immediately. Consequently, the thread will not run until some arbitrary time after the sleep interval elapses,\nbased upon the system \"tick\" frequency and the load factor from other processes.\n\n# See also\n\n> [https://learn.microsoft.com/en-us/windows/win32/api/synchapi/nf-synchapi-sleepex](https://learn.microsoft.com/en-us/windows/win32/api/synchapi/nf-synchapi-sleepex)";
        assert_eq!(crate::transform(S).unwrap(), S_);
    }

    #[test]
    fn with_attributes() {
        const S: &str = "Creates a new registry key or opens an existing one, and it associates the key with a transaction.\n\n@param[out] KeyHandle A pointer to a handle that receives the key handle.\n @param[in] DesiredAccess The access mask that specifies the desired access rights.\n@param[in] ObjectAttributes A pointer to an OBJECT_ATTRIBUTES structure that specifies the object attributes.\n@param[in] TitleIndex Reserved.\n@param[in, optional] Class A pointer to a UNICODE_STRING structure that specifies the class of the key.\n @param[in] CreateOptions The options to use when creating the key.\n@param[in] TransactionHandle A handle to the transaction.\n @param[out, optional] Disposition A pointer to a variable that receives the disposition value.\n@return NTSTATUS Successful or errant status.\n";
        const S_: &str = "Creates a new registry key or opens an existing one, and it associates the key with a transaction.\n\n# Arguments\n\n* `KeyHandle` \\[out\\] - A pointer to a handle that receives the key handle.\n* `DesiredAccess` \\[in\\] - The access mask that specifies the desired access rights.\n* `ObjectAttributes` \\[in\\] - A pointer to an OBJECT_ATTRIBUTES structure that specifies the object attributes.\n* `TitleIndex` \\[in\\] - Reserved.\n* `Class` \\[in, optional\\] - A pointer to a UNICODE_STRING structure that specifies the class of the key.\n* `CreateOptions` \\[in\\] - The options to use when creating the key.\n* `TransactionHandle` \\[in\\] - A handle to the transaction.\n* `Disposition` \\[out, optional\\] - A pointer to a variable that receives the disposition value.\n\n# Returns\n\nNTSTATUS Successful or errant status.\n";
        assert_eq!(crate::transform(S).unwrap(), S_);
    }

    #[test]
    fn new_paragraph_after_html() {
        const S: &str =  "Set encoding parameters to default values:\n<ul>\n<li>Lossless</li>\n<li>1 tile\n</li>\n<li>etc...</li>\n</ul>\n@param parameters Compression parameters";
        const S_: &str = "Set encoding parameters to default values:\n<ul>\n<li>Lossless</li>\n<li>1 tile\n</li>\n<li>etc...</li>\n</ul>\n\n# Arguments\n\n* `parameters` - Compression parameters";
        assert_eq!(crate::transform(S).unwrap(), S_);
    }

    // issue #4
    #[test]
    fn param_attributes_are_escaped() {
        const S: &str = "@param[in] Foo The foo.";
        const S_: &str = "# Arguments\n\n* `Foo` \\[in\\] - The foo.";
        assert_eq!(crate::transform(S).unwrap(), S_);
    }
}
