// Copyright 2021 Datafuse Labs.
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

use common_exception::Result;
use common_io::prelude::CpBufferReader;
use serde_json::Value;

use crate::prelude::*;

mod boolean;
mod date;
mod null;
mod nullable;
mod number;
mod string;
mod timestamp;
mod variant;

pub use boolean::*;
pub use date::*;
pub use null::*;
pub use nullable::*;
pub use number::*;
pub use string::*;
pub use timestamp::*;
pub use variant::*;

pub trait TypeDeserializer: Send + Sync {
    fn de_binary(&mut self, reader: &mut &[u8]) -> Result<()>;

    fn de_default(&mut self);

    fn de_fixed_binary_batch(&mut self, reader: &[u8], step: usize, rows: usize) -> Result<()>;

    fn de_json(&mut self, reader: &Value) -> Result<()>;

    fn de_null(&mut self) -> bool {
        false
    }

    fn de_whole_text(&mut self, reader: &[u8]) -> Result<()>;

    fn de_text(&mut self, reader: &mut CpBufferReader) -> Result<()>;

    fn de_text_csv(&mut self, reader: &mut CpBufferReader) -> Result<()> {
        self.de_text(reader)
    }

    fn de_text_json(&mut self, reader: &mut CpBufferReader) -> Result<()> {
        self.de_text(reader)
    }

    fn de_text_quoted(&mut self, reader: &mut CpBufferReader) -> Result<()> {
        self.de_text(reader)
    }

    fn append_data_value(&mut self, value: DataValue) -> Result<()>;

    fn pop_data_value(&mut self) -> Result<DataValue>;

    fn finish_to_column(&mut self) -> ColumnRef;
}
