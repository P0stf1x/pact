#![no_main]

use std::ffi::{CStr, CString};
use std::iter::Peekable;
use std::str::Chars;

static KEYWORDS: [&str; 59] = [
    "alignas",        // (C23)
    "alignof",        // (C23)
    "auto",
    "bool",           // (C23)
    "break",
    "case",
    "char",
    "const",
    "constexpr",      // (C23)
    "continue",
    "default",
    "do",
    "double",
    "else",
    "enum",
    "extern",
    "false",          // (C23)
    "float",
    "for",
    "goto",
    "if",
    "inline",         // (C99)
    "int",
    "long",
    "nullptr",        // (C23)
    "register",
    "restrict",       // (C99)
    "return",
    "short",
    "signed",
    "sizeof",
    "static",
    "static_assert",  // (C23)
    "struct",
    "switch",
    "thread_local",   // (C23)
    "true",           // (C23)
    "typedef",
    "typeof",         // (C23)
    "typeof_unqual",  // (C23)
    "union",
    "unsigned",
    "void",
    "volatile",
    "while",
    "_Alignas",       // (C11) (deprecated in C23)
    "_Alignof",       // (C11) (deprecated in C23)
    "_Atomic",        // (C11)
    "_BitInt",        // (C23)
    "_Bool",          // (C99) (deprecated in C23)
    "_Complex",       // (C99)
    "_Decimal128",    // (C23)
    "_Decimal32",     // (C23)
    "_Decimal64",     // (C23)
    "_Generic",       // (C11)
    "_Imaginary",     // (C99)
    "_Noreturn",      // (C11) (deprecated in C23)
    "_Static_assert", // (C11) (deprecated in C23)
    "_Thread_local",  // (C11) (deprecated in C23)
];

#[derive(Debug)]
enum Token {
    Identifier(CString),
    Keyword(CString),
    Number(CString),
    Float(CString),
    String(CString),
    Char(CString),
    Operator(CString),
    Punctuation(CString),
}

struct LexerTemporaryValue {
    pub found_token: String,
    pub characters_consumed: usize,
}

#[derive(Debug)]
struct LexerReturnValue {
    pub found_token: Token,
    pub characters_consumed: usize,
}

#[unsafe(no_mangle)]
pub extern "C" fn lex(cstring_pointer: *const i8, length: usize) -> usize {
    let cs = unsafe { CStr::from_ptr(cstring_pointer) };
    let rust_str = cs.to_str().unwrap();
    println!("passed string: `{}`", rust_str);
    let lex_test = lex_start(rust_str);
    println!("{:?}", lex_test);
    println!("passed length: `{}`", length);
    return length;
}

fn lex_start(input: &str) -> LexerReturnValue {
    let mut chars = input.chars().peekable();
    let next = chars.peek().unwrap();
    match next {
        // Identifier / Keyword
        'a'..='z' | 'A'..='Z' | '_' => {
            let result = lex_identifier(&mut chars);
            let token = if KEYWORDS.contains(&result.found_token.as_str()) {
                Token::Keyword(CString::new(result.found_token).unwrap())
            } else {
                Token::Identifier(CString::new(result.found_token).unwrap())
            };
            return LexerReturnValue {
                found_token: token,
                characters_consumed: result.characters_consumed,
            }
        },

        // Zero
        // Special case since it can be binary, octal, hex, float, decimal
        '0' => { return lex_zero(&mut chars) }

        // Other numbers
        '1'..='9' => { return lex_number_or_float(&mut chars) }

        // Strings
        '"' => { return lex_string(&mut chars) }

        // Chars
        '\'' => { return lex_char(&mut chars) }

        // Operator
        '+' | '=' | '*' | '/' | '^' | '%' | '!' | '-' | '|' | '&' => { return lex_operator(&mut chars) }

        // Punctuation
        '(' | ')' | ',' | '.' | ';' => { return lex_punctuation(&mut chars) }
        _ => panic!()
    };
}

fn lex_identifier(chars: &mut Peekable<Chars>) -> LexerTemporaryValue {
    let mut result = String::new();
    let mut consumed = 0;
    loop {
        match chars.peek() {
            Some('a'..='z' | 'A'..='Z' | '0'..='9' | '_') => {
                result.push(chars.next().unwrap());
                consumed += 1;
            },
            _ => break
        }
    }
    return LexerTemporaryValue {
        found_token: result,
        characters_consumed: consumed,
    };
}

fn lex_zero(mut chars: &mut Peekable<Chars>) -> LexerReturnValue {
    let second_next = chars.clone().nth(1); // cloned because there's no peek_nth
    let mut is_float = false;
    let result = match second_next {
        Some('x') => {
            lex_hexadecimal(&mut chars)
        },
        Some('o') => {
            lex_octal(&mut chars)
        },
        Some('b') => {
            lex_binary(&mut chars)
        },
        Some('.') => {
            is_float = true;
            lex_float(&mut chars, "0".to_string())
        },
        Some('0'..='9') => {
            return lex_number_or_float(&mut chars);
        }
        _ => return LexerReturnValue {
            found_token: Token::Number(CString::new("0").unwrap()),
            characters_consumed: 1,
        }
    };
    let token = if is_float {
        Token::Float(CString::new(result.found_token).unwrap())
    } else {
        Token::Number(CString::new(result.found_token).unwrap())
    };
    return LexerReturnValue {
        found_token: token,
        characters_consumed: result.characters_consumed,
    };
}

fn lex_number_or_float(mut chars: &mut Peekable<Chars>) -> LexerReturnValue {
    let mut result = String::new();
    let mut consumed = 0;
    let mut is_float = false;
    loop {
        match chars.peek() {
            Some('0'..='9') => {
                result.push(chars.next().unwrap());
                consumed += 1;
            },
            Some('.') => {
                is_float = true;
                let float_lex_result = lex_float(&mut chars, result);
                result = float_lex_result.found_token;
                consumed = float_lex_result.characters_consumed;
                break;
            }
            _ => break
        }
    }
    let token = if is_float {
        Token::Float(CString::new(result).unwrap())
    } else {
        Token::Number(CString::new(result).unwrap())
    };
    return LexerReturnValue {
        found_token: token,
        characters_consumed: consumed,
    };
}

fn lex_hexadecimal(chars: &mut Peekable<Chars>) -> LexerTemporaryValue {
    let mut result = String::new();
    result.push(chars.next().unwrap()); // 0
    result.push(chars.next().unwrap()); // o
    let mut consumed = 2;
    loop {
        match chars.peek() {
            Some('a'..='f' | 'A'..='F' | '0'..='9') => {
                result.push(chars.next().unwrap());
                consumed += 1;
            },
            _ => break
        }
    }
    return LexerTemporaryValue {
        found_token: result,
        characters_consumed: consumed,
    };
}

fn lex_octal(chars: &mut Peekable<Chars>) -> LexerTemporaryValue {
    let mut result = String::new();
    result.push(chars.next().unwrap()); // 0
    result.push(chars.next().unwrap()); // x
    let mut consumed = 2;
    loop {
        match chars.peek() {
            Some('0'..='7') => {
                result.push(chars.next().unwrap());
                consumed += 1;
            },
            _ => break
        }
    }
    return LexerTemporaryValue {
        found_token: result,
        characters_consumed: consumed,
    };
}

fn lex_binary(chars: &mut Peekable<Chars>) -> LexerTemporaryValue {
    let mut result = String::new();
    result.push(chars.next().unwrap()); // 0
    result.push(chars.next().unwrap()); // b
    let mut consumed = 2;
    loop {
        match chars.peek() {
            Some('0'..='1') => {
                result.push(chars.next().unwrap());
                consumed += 1;
            },
            _ => break
        }
    }
    return LexerTemporaryValue {
        found_token: result,
        characters_consumed: consumed,
    };
}

fn lex_float(chars: &mut Peekable<Chars>, found: String) -> LexerTemporaryValue {
    let mut result = found;
    let mut consumed = 2;
    loop {
        match chars.peek() {
            Some('0'..='9' | '.') => {
                result.push(chars.next().unwrap());
                consumed += 1;
            },
            _ => break
        }
    }
    return LexerTemporaryValue {
        found_token: result,
        characters_consumed: consumed,
    };
}

fn lex_string(chars: &mut Peekable<Chars>) -> LexerReturnValue {
    let mut result = String::new();
    let mut previous_character_is_escape = false;
    result.push(chars.next().unwrap());
    let mut consumed = 1;
    loop {
        match chars.peek() {
            Some('"') => {
                result.push(chars.next().unwrap());
                consumed += 1;
                if !previous_character_is_escape {
                    break;
                }
            }
            Some(c) => {
                if *c == '\\' { // mark that previous char is escape char
                    previous_character_is_escape = !previous_character_is_escape; // but not if it's \\ (two escapes in a row)
                } else {
                    previous_character_is_escape = false;
                }
                result.push(chars.next().unwrap());
                consumed += 1;
            },
            None => {
                panic!("String not terminated");
            }
        }
    }
    return LexerReturnValue {
        found_token: Token::String(CString::new(result).unwrap()),
        characters_consumed: consumed,
    };
}

fn lex_char(chars: &mut Peekable<Chars>) -> LexerReturnValue {
    let mut result = String::new();
    let mut found_char = false;
    let mut previous_character_is_escape = false;
    result.push(chars.next().unwrap());
    let mut consumed = 1;
    loop {
        match chars.peek() {
            Some('\'') => {
                if found_char {
                    result.push(chars.next().unwrap());
                    consumed += 1;
                    break;
                } else {
                    if previous_character_is_escape {
                        result.push(chars.next().unwrap());
                        consumed += 1;
                        found_char = true;
                        continue;
                    }
                    // Empty chars are invalid
                    panic!("Empty chars are invalid");
                }
            }
            Some(c) => {
                if found_char {
                    panic!("Char cannot contain more than 1 char");
                }
                if *c == '\\' { // mark that previous char is backslash
                    if previous_character_is_escape {
                        found_char = true;
                    }
                    previous_character_is_escape = true;
                } else {
                    found_char = true; // just mark as found if not backslash
                }
                result.push(chars.next().unwrap());
                consumed += 1;
            },
            None => {
                panic!("Char not terminated");
            }
        }
    }
    return LexerReturnValue {
        found_token: Token::Char(CString::new(result).unwrap()),
        characters_consumed: consumed,
    };
}

fn lex_operator(chars: &mut Peekable<Chars>) -> LexerReturnValue {
    let mut result = String::new();
    let mut consumed = 0;
    loop {
        match chars.peek() {
            Some('+' | '=' | '*' | '/' | '^' | '%' | '!' | '-' | '|' | '&') => {
                result.push(chars.next().unwrap());
                consumed += 1;
            },
            _ => break
        }
    }
    return LexerReturnValue {
        found_token: Token::Operator(CString::new(result).unwrap()),
        characters_consumed: consumed,
    };
}

fn lex_punctuation(chars: &mut Peekable<Chars>) -> LexerReturnValue {
    let mut result = String::new();
    let mut consumed = 0;
    loop {
        match chars.peek() {
            Some('(' | ')' | ',' | '.' | ';') => {
                result.push(chars.next().unwrap());
                consumed += 1;
            },
            _ => break
        }
    }
    return LexerReturnValue {
        found_token: Token::Punctuation(CString::new(result).unwrap()),
        characters_consumed: consumed,
    };
}
