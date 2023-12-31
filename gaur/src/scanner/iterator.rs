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

use super::tokens::Token;
use std::io::{BufRead, BufReader, Error as IoError, Read};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TokenError {
    #[error("buffer read failed")]
    Io(#[from] IoError),
}

pub struct TokenStreamer<T>
where
    T: BufRead,
{
    input: T,
    status: Result<(), TokenError>,
}

impl<R> TokenStreamer<BufReader<R>>
where
    R: Read,
{
    pub fn from_reader(reader: R) -> Self {
        let input = BufReader::new(reader);
        TokenStreamer {
            input,
            status: Ok(()),
        }
    }
}

pub enum TokenOutput<'a> {
    Token(Token<'a>),
    Eof,
    Err(TokenError),
}

impl<T> TokenStreamer<T>
where
    T: BufRead,
{
    pub fn from_bufreader(buf_reader: T) -> Self {
        TokenStreamer {
            input: buf_reader,
            status: Ok(()),
        }
    }

    pub fn next_token<'a>(&'a mut self) -> TokenOutput<'a> {
        TokenOutput::Eof
    }
}
