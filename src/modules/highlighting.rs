//! # Syntax Highlighting Module
//! 
//! This module provides syntax highlighting functionality for the Ninja editor.
//! It supports multiple programming languages with customizable color schemes
//! and extensible highlighting rules.
//! 
//! ## Features
//! 
//! - **Multi-language Support**: Rust, C, Java, Python, Go, JavaScript, TypeScript, HTML, CSS, TOML
//! - **Real-time Highlighting**: Updates syntax highlighting as you type
//! - **Extensible System**: Easy to add new language support
//! - **Customizable Colors**: Configurable color schemes for different token types
//! - **Comment Support**: Single-line and multi-line comment highlighting
//! - **String Literals**: Proper highlighting of string and character literals
//! - **Keyword Recognition**: Language-specific keyword highlighting
//! 
//! ## Architecture
//! 
//! The highlighting system consists of:
//! - **`HighlightType`**: Enumeration of different token types
//! - **`SyntaxHighlight`**: Trait defining the interface for language highlighters
//! - **`syntax_struct!`**: Macro for easily creating new language highlighters
//! - **Language-specific structs**: Concrete implementations for each supported language
//! 
//! ## Usage
//! 
//! ```rust
//! use ninja::modules::highlighting::{SyntaxHighlight, RustHighlight, HighlightType};
//!
//! let highlighter = RustHighlight::new();
//! let color = highlighter.syntax_color(&HighlightType::String);
//! ```

use std::cmp;
use crossterm::queue;
use crossterm::style::{Color, SetForegroundColor};
use crate::screens::editor::Row;
use crate::screens::editor::EditorContents;

/// Represents different types of syntax highlighting tokens.
/// 
/// This enum defines all the different types of tokens that can be highlighted
/// in the editor, each with its own color scheme.
/// 
/// # Variants
/// 
/// - **`Normal`**: Default text color
/// - **`Number`**: Numeric literals (integers, floats)
/// - **`SearchMatch`**: Text that matches the current search term
/// - **`String`**: String literals (double-quoted)
/// - **`CharLiteral`**: Character literals (single-quoted)
/// - **`Comment`**: Single-line comments
/// - **`MultilineComment`**: Multi-line comments
/// - **`Selection`**: Currently selected text
/// - **`Other`**: Custom color for special cases
/// 
/// # Example
/// 
/// ```rust
/// use crossterm::style::Color;
/// use ninja::modules::highlighting::HighlightType;
///
/// let highlight = HighlightType::String;
/// let selection = HighlightType::Selection;
/// let custom = HighlightType::Other(Color::Red);
/// ```
#[derive(Copy, Clone)]
pub enum HighlightType {
    /// Default text color
    Normal,
    /// Numeric literals (integers, floats)
    Number,
    /// Text that matches the current search term
    SearchMatch,
    /// String literals (double-quoted)
    String,
    /// Character literals (single-quoted)
    CharLiteral,
    /// Single-line comments
    Comment,
    /// Multi-line comments
    MultilineComment,
    /// Currently selected text
    Selection,
    /// Custom color for special cases
    Other(Color),
}

/// Trait defining the interface for syntax highlighting implementations.
/// 
/// This trait provides the contract that all language-specific syntax
/// highlighters must implement. It defines methods for:
/// - Language identification and file extensions
/// - Comment syntax detection
/// - Color scheme definition
/// - Syntax analysis and highlighting
/// 
/// # Implementing Syntax Highlighting
/// 
/// To add support for a new language, implement this trait:
/// 
/// ```rust
/// use ninja::modules::highlighting::{SyntaxHighlight, HighlightType};
/// use crossterm::style::Color;/// 
///
/// use ninja::screens::editor::Row;
///
/// struct MyLanguageHighlight;
///
/// impl SyntaxHighlight for MyLanguageHighlight {
///     fn extensions(&self) -> &[&str] {
///         &["my", "mylang"]
///     }
///     
///     fn file_type(&self) -> &str {
///         "MyLanguage"
///     }
///     
///     fn comment_start(&self) -> &str {
///         "//"
///     }
///     
///     fn multiline_comment(&self) -> Option<(&str, &str)> {
///         Some(("/*", "*/"))
///     }
///     
///     fn syntax_color(&self, highlight_type: &HighlightType) -> Color {
///         match highlight_type {
///             HighlightType::String => Color::Green,
///             HighlightType::Comment => Color::DarkGrey,
///             _ => Color::Reset,
///         }
///     }
///     
///     fn update_syntax(&self, at: usize, editor_rows: &mut Vec<Row>) {
///         // Implement syntax analysis logic
///     }
/// }
/// ```
pub trait SyntaxHighlight {
    /// Returns the file extensions supported by this highlighter.
    /// 
    /// # Returns
    /// 
    /// Returns a slice of file extensions (without the dot) that this
    /// highlighter can handle.
    fn extensions(&self) -> &[&str];
    
    /// Returns the human-readable name of the language.
    /// 
    /// # Returns
    /// 
    /// Returns a string representing the language name.
    fn file_type(&self) -> &str;
    
    /// Returns the string that starts single-line comments.
    /// 
    /// # Returns
    /// 
    /// Returns the comment start sequence (e.g., "//", "#", "--").
    fn comment_start(&self) -> &str;
    
    /// Returns the start and end sequences for multi-line comments.
    /// 
    /// # Returns
    /// 
    /// Returns `Some((start, end))` if the language supports multi-line comments,
    /// or `None` if it doesn't.
    fn multiline_comment(&self) -> Option<(&str, &str)>;
    
    /// Returns the color for a given highlight type.
    /// 
    /// # Arguments
    /// 
    /// * `highlight_type` - The type of token to color
    /// 
    /// # Returns
    /// 
    /// Returns the color that should be used for the given highlight type.
    fn syntax_color(&self, highlight_type: &HighlightType) -> Color;
    
    /// Updates the syntax highlighting for a specific row.
    /// 
    /// This method analyzes the text in the specified row and updates
    /// the highlighting information. It should handle:
    /// - Keyword recognition
    /// - String and character literal detection
    /// - Comment detection (single and multi-line)
    /// - Number recognition
    /// - Continuation from previous rows (for multi-line constructs)
    /// 
    /// # Arguments
    /// 
    /// * `at` - The index of the row to update
    /// * `editor_rows` - The complete collection of editor rows
    fn update_syntax(&self, at: usize, editor_rows: &mut Vec<Row>);
    
    /// Renders a row with syntax highlighting colors.
    /// 
    /// This method applies the highlighting colors to the text and
    /// writes the colored output to the provided buffer.
    /// 
    /// # Arguments
    /// 
    /// * `render` - The text to render
    /// * `highlight` - The highlighting information for each character
    /// * `out` - The output buffer to write colored text to
    fn color_row(&self, render: &str, highlight: &[HighlightType], out: &mut EditorContents) {
        let mut current_color = self.syntax_color(&HighlightType::Normal);
        render.char_indices().enumerate().for_each(|(char_index, (_byte_index, c))| {
            let color = if char_index < highlight.len() {
                self.syntax_color(&highlight[char_index])
            } else {
                self.syntax_color(&HighlightType::Normal)
            };
            if current_color != color {
                current_color = color;
                let _ = queue!(out, SetForegroundColor(color));
            }
            out.push(c);
        });
        let _ = queue!(out, SetForegroundColor(Color::Reset));
    }
    
    /// Determines if a character is a separator (word boundary).
    /// 
    /// This method is used to identify word boundaries for keyword
    /// recognition and other syntax analysis.
    /// 
    /// # Arguments
    /// 
    /// * `c` - The character to check
    /// 
    /// # Returns
    /// 
    /// Returns `true` if the character is a separator, `false` otherwise.
    fn is_separator(&self, c: char) -> bool {
        c.is_whitespace()
            || [
            ',', '.', '[', ']', '(', ')', '+', '-', '/', '*', '=', '~', '%', '<', '>', '"',
            '\'', ';', '&',
        ]
            .contains(&c)
    }
}

/// Macro for easily creating new syntax highlighting implementations.
/// 
/// This macro generates a complete syntax highlighting implementation
/// for a new language, including the struct definition, constructor,
/// and trait implementation.
/// 
/// # Syntax
/// 
/// ```rust
/// use ninja::syntax_struct;
/// use crossterm::style::Color;
///
/// syntax_struct! {
///     struct LanguageName {
///         extensions: ["ext1", "ext2"],
///         file_type: "Language Name",
///         comment_start: "//",
///         keywords: {
///             [Color::Yellow; "keyword1", "keyword2"],
///             [Color::Magenta; "type1", "type2"]
///         },
///         multiline_comment: Some(("/*", "*/"))
///     }
/// }
/// ```
/// 
/// # Parameters
/// 
/// - **`struct`**: The name of the struct to create
/// - **`extensions`**: Array of file extensions this language supports
/// - **`file_type`**: Human-readable language name
/// - **`comment_start`**: String that starts single-line comments
/// - **`keywords`**: Array of keyword groups with their colors
/// - **`multiline_comment`**: Optional tuple of multi-line comment delimiters
/// 
/// # Example
/// 
/// ```rust
/// use ninja::syntax_struct;
/// use crossterm::style::Color;
///
/// syntax_struct! {
///     struct MyLanguageHighlight {
///         extensions: ["ml", "mylang"],
///         file_type: "MyLanguage",
///         comment_start: "#",
///         keywords: {
///             [Color::Yellow; "if", "else", "while", "for"],
///             [Color::Magenta; "int", "string", "bool"]
///         },
///         multiline_comment: Some(("/*", "*/"))
///     }
/// }
/// ```
#[macro_export]
macro_rules! syntax_struct {
    (
        struct $Name:ident {
            extensions:$ext:expr,
            file_type:$type:expr,
            comment_start:$start:expr,
            keywords: {
                $([$color:expr; $($words:expr),*]),*
            },
            multiline_comment:$ml_comment:expr
        }
    ) => {
        pub struct $Name {
            extensions: &'static [&'static str],
            file_type: &'static str,
            comment_start:&'static str,
            multiline_comment:Option<(&'static str,&'static str)>
        }

        impl $Name {
            pub fn new() -> Self {
                Self {
                    extensions: &$ext,
                    file_type: $type,
                    comment_start:$start,
                    multiline_comment: $ml_comment
                }
            }
        }

        impl SyntaxHighlight for $Name {

            fn comment_start(&self) -> &str {
                self.comment_start
            }

            fn multiline_comment(&self) -> Option<(&str, &str)> {
                self.multiline_comment
            }

            fn extensions(&self) -> &[&str] {
                self.extensions
            }

            fn file_type(&self) -> &str {
                self.file_type
            }

            fn syntax_color(&self, highlight_type: &HighlightType) -> Color {
                match highlight_type {
                    HighlightType::Normal => Color::Reset,
                    HighlightType::Number => Color::Cyan,
                    HighlightType::SearchMatch => Color::Blue,
                    HighlightType::String => Color::Green,
                    HighlightType::CharLiteral => Color::DarkGreen,
                    HighlightType::Comment | HighlightType::MultilineComment => Color::DarkGrey,
                    HighlightType::Selection => Color::White,
                    HighlightType::Other(color) => *color
                }
            }

            fn update_syntax(&self, at: usize, editor_rows: &mut Vec<Row>) {
                let mut in_comment = at > 0 && editor_rows[at - 1].is_comment; // add line
                let current_row = &mut editor_rows[at];
                macro_rules! add {
                    ($h:expr) => {
                        current_row.highlight.push($h)
                    };
                }
                current_row.highlight = Vec::with_capacity(current_row.render.len());
                let render = current_row.render.as_bytes();
                let mut i = 0;
                let mut previous_separator = true;
                let mut in_string: Option<char> = None;
                let comment_start = self.comment_start().as_bytes();
                while i < render.len() {
                    let c = render[i] as char;
                    let previous_highlight = if i > 0 {
                        current_row.highlight[i - 1]
                    } else {
                        HighlightType::Normal
                    };
                    if in_string.is_none() && !comment_start.is_empty() && !in_comment { // modify
                        let end = i + comment_start.len();
                        if render[i..cmp::min(end, render.len())] == *comment_start {
                            (i..render.len()).for_each(|_| add!(HighlightType::Comment));
                            break;
                        }
                    }
                    if let Some(val) = $ml_comment {
                        if in_string.is_none() {
                            if in_comment {
                                add!(HighlightType::MultilineComment);
                                let end = i + val.1.len();
                                if render[i..cmp::min(render.len(),end)] == *val.1.as_bytes() {
                                    (0..val.1.len().saturating_sub(1)).for_each(|_| add!(HighlightType::MultilineComment));
                                    i = end;
                                    previous_separator = true;
                                    in_comment = false;
                                    continue
                                } else {
                                    i+=1;
                                    continue
                                }
                            } else {
                                let end = i + val.0.len();
                                if render[i..cmp::min(render.len(),end)] == *val.0.as_bytes() {
                                    (i..end).for_each(|_| add!(HighlightType::MultilineComment));
                                    i+= val.0.len();
                                    in_comment = true;
                                    continue
                                }
                            }
                        }
                    }
                    if let Some(val) = in_string {
                        add! {
                            if val == '"' { HighlightType::String } else { HighlightType::CharLiteral }
                        }
                        if c == '\\' && i + 1 < render.len() {
                            add! {
                                if val == '"' { HighlightType::String } else { HighlightType::CharLiteral }
                            }
                            i += 2;
                            continue
                        }
                        if val == c {
                            in_string = None;
                        }
                        i += 1;
                        previous_separator = true;
                        continue;
                    } else if c == '"' || c == '\'' {
                        in_string = Some(c);
                        add! {
                            if c == '"' { HighlightType::String } else { HighlightType::CharLiteral }
                        }
                        i += 1;
                        continue;
                    }
                    if (c.is_digit(10)
                        && (previous_separator
                            || matches!(previous_highlight, HighlightType::Number)))
                        || (c == '.' && matches!(previous_highlight, HighlightType::Number))
                    {
                        add!(HighlightType::Number);
                        i += 1;
                        previous_separator = false;
                        continue;
                    }
                    if previous_separator {
                        $(
                            $(
                                let end = i + $words.len();
                                let is_end_or_sep = render
                                    .get(end)
                                    .map(|c| self.is_separator(*c as char))
                                    .unwrap_or(end == render.len());
                                if is_end_or_sep && render[i..end] == *$words.as_bytes() {
                                    (i..end).for_each(|_| add!(HighlightType::Other($color)));
                                    i += $words.len();
                                    previous_separator = false;
                                    continue;
                                }
                            )*
                        )*
                    }
                    add!(HighlightType::Normal);
                    previous_separator = self.is_separator(c);
                    i += 1;
                }
                assert_eq!(current_row.render.len(), current_row.highlight.len());
                let changed = current_row.is_comment != in_comment;
                current_row.is_comment = in_comment;
                if (changed && at + 1 < editor_rows.len()) {
                    self.update_syntax(at+1,editor_rows)
                }
            }
        }
    };
}

syntax_struct! {
    struct RustHighlight {
        extensions:["rs"],
        file_type:"Rust",
        comment_start:"//",
        keywords : {
            [Color::Yellow;
                "mod","unsafe","extern","crate","use","type","struct","enum","union","const","static",
                "mut","let","if","else","impl","trait","for","fn","self","Self", "while", "true","false",
                "in","continue","break","loop","match"
            ],
            [Color::Magenta; "isize","i8","i16","i32","i64","usize","u8","u16","u32","u64","f32","f64",
                "char","str","bool"
            ]
        },
        multiline_comment: Some(("/*", "*/"))
    }
}

syntax_struct! {
    struct CHighlight {
        extensions:["c","h"],
        file_type:"C",
        comment_start:"//",
        keywords : {
            [Color::Yellow;
                "auto","break","case","char","const","continue","default","do","double","else",
                "enum","extern","float","for","goto","if", "include", "int","long","register","return",
                "short","signed","sizeof","static","struct","switch","typedef", "union", "unsigned",
                "void", "volatile", "while"
            ],
            [Color::Magenta; "bool", "true", "false"]
        },
        multiline_comment: Some(("/*", "*/"))
    }
}

syntax_struct! {
    struct JavaHighlight {
        extensions:["java"],
        file_type:"Java",
        comment_start:"//",
        keywords : {
            [Color::Yellow;
                "abstract","assert","boolean","break","byte","case","catch","char","class",
                "const","continue","default","do","double","else","enum","extends","final",
                "finally","float","for","goto","if", "implements", "import", "instanceof",
                "int", "interface", "long", "native", "new", "null", "package", "private",
                "protected", "public", "return", "short", "static", "strictfp", "super",
                "switch", "synchronized", "this", "throw", "throws", "transient", "try",
                "void", "volatile", "while"
            ],
            [Color::Magenta;  ]
        },
        multiline_comment: Some(("/*", "*/"))
    }
}

syntax_struct! {
    struct PythonHighlight {
        extensions:["py"],
        file_type:"Python",
        comment_start:"#",
        keywords : {
            [Color::Yellow;
                "and","as","assert","async","await","break","class","continue","def","del",
                "elif","else","except","finally","for","from", "global", "if", "import",
                "in", "is", "lambda", "nonlocal", "not", "or", "pass", "raise", "return",
                "try", "while", "with", "yield"
            ],
            [Color::Magenta; ]
        },
        multiline_comment: Some(("", ""))
    }
}

syntax_struct! {
    struct GoHighlight {
        extensions:["go", "mod"],
        file_type:"Go",
        comment_start:"//",
        keywords : {
            [Color::Yellow;
                "break","case","chan","const","continue","default","defer","else","fallthrough",
                "for","func","go","goto","if", "import", "interface", "map", "module", "package", "range",
                "return", "select", "struct", "switch", "type", "var"
            ],
            [Color::Magenta; ]
        },
        multiline_comment: Some(("/*", "*/"))
    }
}

syntax_struct! {
    struct JavaScriptHighlight {
        extensions:["js","jsx"],
        file_type:"JavaScript",
        comment_start:"//",
        keywords : {
            [Color::Yellow;
                "break","case","catch","class","const","continue","debugger","default","delete",
                "do","else","export","extends","finally","for", "function", "if", "import",
                "in", "instanceof", "let", "new", "return", "super", "switch", "this",
                "throw", "try", "typeof", "var", "void", "while", "with", "yield"
            ],
            [Color::Magenta; ]
        },
        multiline_comment: Some(("/*", "*/"))
    }
}

syntax_struct! {
    struct TypeScriptHighlight {
        extensions:["ts","tsx"],
        file_type:"TypeScript",
        comment_start:"//",
        keywords : {
            [Color::Yellow;
                "abstract","as","async","await","break","case","catch","class","const",
                "continue","debugger","default","delete","do","else", "enum", "export",
                "extends", "finally", "for", "function", "if", "implements", "import",
                "in", "instanceof", "interface", "let", "new", "return", "super",
                "switch", "this", "throw", "try", "type", "typeof", "var", "void",
                "while", "with", "yield"
            ],
            [Color::Magenta; ]
        },
        multiline_comment: Some(("/*", "*/"))
    }
}

/*syntax_struct! {
    struct CSharpHighlight {
        extensions:["cs"],
        file_type:"csharp",
        comment_start:"//",
        keywords : {
            [Color::Yellow;
                "abstract","as","base","break","case","catch","class","const","continue",
                "default","delegate","do","else","enum","event","explicit","extern",
                "finally", "fixed", "for", "foreach", "goto", "if", "implicit", "in",
                "interface", "internal", "is", "lock", "namespace", "new", "null",
                "operator", "out", "override", "params", "private", "protected",
                "public", "readonly", "ref", "return", "sealed", "sizeof",
                "stackalloc", "static", "struct", "switch", "this", "throw",
                "try", "typeof", "unchecked", "unsafe", "using", "virtual",
                "void", "volatile", "while"
            ],
            [Color::Magenta; ]
        },
        multiline_comment: Some(("/*", "*/"))
    }
}*/

syntax_struct! {
    struct RHighlight {
        extensions:["r"],
        file_type:"R",
        comment_start:"#",
        keywords : {
            [Color::Yellow;
                "if", "else", "for", "while", "function", "return", "break", "next",
                "repeat", "switch", "case", "default", "try", "catch", "finally",
                "library", "require"
            ],
            [Color::Magenta; ]
        },
        multiline_comment: Some(("", "")) // R does not have multiline comments
    }
}

syntax_struct! {
    struct PHPHighlight {
        extensions:["php"],
        file_type:"PHP",
        comment_start:"//",
        keywords : {
            [Color::Yellow;
                "abstract","and","array","as","break","case","catch","class","clone",
                "const","continue","declare","default","die","do","echo", "else", "elseif",
                "empty", "enddeclare", "endfor", "endforeach", "endif", "endswitch",
                "endwhile", "eval", "exit", "extends", "final", "finally", "fn",
                "for", "foreach", "function", "global", "goto", "if", "implements",
                "include", "include_once", "instanceof", "insteadof", "interface",
                "isset", "list", "namespace", "new", "or", "print", "private",
                "protected", "public", "require", "require_once", "return",
                "static", "switch", "throw", "trait",  "try",
                "unset",  "use" ,"var" ,"while"
            ],
            [Color::Magenta; ]
        },
        multiline_comment: Some(("/*", "*/"))
    }
}

syntax_struct! {
    struct ObjectiveCHighlight {
        extensions:["m","mm"],
        file_type:"OBJ-C",
        comment_start:"//",
        keywords : {
            [Color::Yellow;
                "auto","break","case","char","const","continue","default","do","double",
                "else","enum","extern","float","for","goto","if", "include", "int",
                "long","register","return","short","signed","sizeof","static",
                "struct", "switch", "typedef", "union", "unsigned", "void", "volatile",
                "while"
            ],
            [Color::Magenta; ]
        },
        multiline_comment: Some(("/*", "*/"))
    }
}

syntax_struct! {
    struct SwiftHightlight {
        extensions:["swift"],
        file_type:"Swift",
        comment_start:"//",
        keywords : {
            [Color::Yellow;
                "associatedtype", "class", "deinit", "enum", "extension", "fileprivate",
                "func", "import", "init", "inout", "internal", "let", "open", "operator",
                "private", "protocol", "public", "static", "struct", "subscript",
                "typealias", "var"
            ],
            [Color::Magenta; ]
        },
        multiline_comment: Some(("/*", "*/"))
    }
}

syntax_struct! {
    struct KotlinHighlight {
        extensions:["kt","kts"],
        file_type:"Kotlin",
        comment_start:"//",
        keywords : {
            [Color::Yellow;
                "abstract","as","break","class","continue","do","else","false","final",
                "for","fun","if", "import", "in", "interface", "is", "null", "object",
                "package", "return", "super", "this", "throw", "true", "try", "typealias",
                "val", "var", "when", "while"
            ],
            [Color::Magenta; ]
        },
        multiline_comment: Some(("/*", "*/"))
    }
}

syntax_struct! {
    struct DartHighlight {
        extensions:["dart"],
        file_type:"Dart",
        comment_start:"//",
        keywords : {
            [Color::Yellow;
                "abstract","as","assert","async","await","break","case","catch","class",
                "const","continue","default","do","else", "enum", "extends", "export",
                "extends", "extension", "final", "finally", "for", "if", "implements",
                "import", "in", "interface", "is", "library", "new", "null", "part",
                "rethrow", "return", "set", "static", "super", "switch", "this",
                "throw", "try", "typedef", "var", "void", "while"
            ],
            [Color::Magenta; ]
        },
        multiline_comment: Some(("/*", "*/"))
    }
}

syntax_struct! {
    struct RubyHighlight {
        extensions:["rb"],
        file_type:"Ruby",
        comment_start:"#",
        keywords : {
            [Color::Yellow;
                "alias","and","begin","break","case","class","def","defined?","do",
                "else", "elsif", "end", "ensure", "false", "for", "if", "in",
                "module", "next", "nil", "not", "or", "redo", "rescue", "retry",
                "return", "self", "super", "then", "true", "undef", "unless",
                "until", "when", "while"
            ],
            [Color::Magenta; ]
        },
        multiline_comment: Some(("", "")) // Ruby does not have multiline comments
    }
}

syntax_struct! {
    struct HTMLHighlight {
        extensions:["html","htm"],
        file_type:"HTML",
        comment_start:"<!--",
        keywords : {
            [Color::Yellow;
                "html", "head", "body", "title", "meta", "link", "script", "style", "div",
                "span", "p", "a", "img", "ul", "ol", "li", "table", "tr", "td", "th",
                "form", "input", "button"
            ],
            [Color::Magenta; ]
        },
        multiline_comment: Some(("<!--", "-->"))
    }
}

syntax_struct! {
    struct CSSHighlight {
        extensions:["css"],
        file_type:"CSS",
        comment_start:"/*",
        keywords : {
            [Color::Yellow;
                "background", "border", "color", "display", "font", "height", "margin",
                "padding", "position", "text", "width"
            ],
            [Color::Magenta; ]
        },
        multiline_comment: Some(("/*", "*/"))
    }
}

syntax_struct! {
    struct TOMLHighlight {
        extensions:["toml"],
        file_type:"TOML",
        comment_start:"#",
        keywords : {
            [Color::Yellow; 
                "true", "false", "null", "nil"
            ],
            [Color::Magenta; ]
        },
        multiline_comment: Some(("#", "#"))
    }
}
