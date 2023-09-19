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

use std::io::{BufRead, BufReader, Read};

pub struct Scanner<T>
where
    T: BufRead,
{
    input: T,
}

impl<R> Scanner<BufReader<R>>
where
    R: Read,
{
    pub fn from_reader(reader: R) -> Self {
        let input = BufReader::new(reader);
        Scanner { input }
    }
}

impl<T> Scanner<T>
where
    T: BufRead,
{
    pub fn from_bufreader(buf_reader: T) -> Self {
        Scanner { input: buf_reader }
    }
}
