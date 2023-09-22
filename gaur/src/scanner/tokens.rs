/*
 * Copyright 2023 Rishvic Pushpakaran
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use strum::EnumIter;

#[derive(Debug, Clone, Copy, EnumIter)]
pub enum TokenType {
    Token,
    Left,
    Right,
    NonAssoc,
    Type,
    Start,
    Union,
    Prec,

    PercentPercent,
    Colon,
    Semi,
    Pipe,

    PercentBlock,
    CodeBlock,
    UnionTag,

    Identifier,
    Number,
    CharConst,

    Comment,
    Whitespace,
}

impl TokenType {
    pub fn pattern(&self) -> &'static str {
        const ID_RE: &'static str = r"[_.a-zA-Z][_.0-9a-zA-Z]*";

        const NUM_RE: &'static str = r"(?x:
            # Hex constant.
            (0[xX][0-9a-fA-F]*)  |
            # Octal constant.
            (0[0-7]*)  |
            # Decimal constant.
            ([1-9][0-9]*)
        )";

        const CHAR_RE: &'static str = r#"(?x:
            # Universal character names.
            ('\\U[0-9a-fA-F]{8}')  |
            ('\\u[0-9a-fA-F]{4}')  |
            # Numeric escape sequences.
            ('\\x[0-9a-fA-F]+')  |
            ('\\[0-7]{1,3}')     |
            # Simple escape sequences.
            ('\\[?'"\\abfnrtv]')  |
            # Simple character constant.
            ('[^'\\\n\r]')
        )"#;

        match self {
            TokenType::Token => r"%token",
            TokenType::Left => r"%left",
            TokenType::Right => r"%right",
            TokenType::NonAssoc => r"%nonassoc",
            TokenType::Type => r"%type",
            TokenType::Start => r"%start",
            TokenType::Union => r"%union",
            TokenType::Prec => r"%prec",

            TokenType::PercentPercent => r"%%",
            TokenType::Colon => r":",
            TokenType::Semi => r";",
            TokenType::Pipe => r"\|",

            TokenType::PercentBlock => r"%\{",
            TokenType::CodeBlock => r"\{",
            TokenType::UnionTag => r"<",

            TokenType::Identifier => ID_RE,
            TokenType::Number => NUM_RE,
            TokenType::CharConst => CHAR_RE,

            TokenType::Comment => r"(?s:/\*.*?\*/)",
            TokenType::Whitespace => r"[ \f\n\r\t]+",
        }
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    token_type: TokenType,
    text: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use regex_automata::{meta::Regex, Anchored, Input};
    use strum::IntoEnumIterator;

    #[test]
    fn token_type_iterator_same_as_usize() {
        for (i, token_type) in TokenType::iter().enumerate() {
            assert_eq!(i, token_type as usize);
        }
    }

    #[test]
    fn no_token_matches_empty_string() {
        let patterns: &[&'static str] = &TokenType::iter()
            .map(|token_type| token_type.pattern())
            .collect::<Vec<&'static str>>()[..];

        let patt_re = Regex::new_many(patterns).unwrap();
        let input = Input::new("").anchored(Anchored::Yes);
        assert!(!patt_re.is_match(input));
    }
}
