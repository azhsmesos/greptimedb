// Copyright 2023 Greptime Team
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::any::Any;

use common_error::prelude::*;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum Error {
    #[snafu(display("Unsupported expr type: {}", name))]
    UnsupportedExpr { name: String, backtrace: Backtrace },

    #[snafu(display(
        "Illegal range: offset {}, length {}, array len {}",
        offset,
        length,
        len
    ))]
    IllegalRange {
        offset: u32,
        length: u32,
        len: usize,
        backtrace: Backtrace,
    },

    #[snafu(display("Empty range is not expected"))]
    EmptyRange { backtrace: Backtrace },
}

impl ErrorExt for Error {
    fn status_code(&self) -> StatusCode {
        use Error::*;
        match self {
            UnsupportedExpr { .. } => StatusCode::InvalidArguments,
            IllegalRange { .. } | EmptyRange { .. } => StatusCode::Internal,
        }
    }
    fn backtrace_opt(&self) -> Option<&Backtrace> {
        ErrorCompat::backtrace(self)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub type Result<T> = std::result::Result<T, Error>;
