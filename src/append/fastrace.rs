// Copyright 2024 tison <wander4096@gmail.com>
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

use std::time::SystemTime;

use log::Record;

use crate::Append;
use crate::AppendImpl;
use crate::KvDisplay;

#[derive(Default, Debug, Clone)]
pub struct FastraceAppend;

impl Append for FastraceAppend {
    fn try_append(&self, record: &Record) -> anyhow::Result<()> {
        let mut message = format!(
            "{} {:>5} {}{}",
            humantime::format_rfc3339_micros(SystemTime::now()),
            record.level(),
            record.args(),
            KvDisplay::new(record.key_values()),
        );
        if message.contains('\n') {
            // Align multi-line log messages with the first line after `level``.
            message = message.replace('\n', "\n                                  ");
        }
        fastrace::Event::add_to_local_parent(message, || []);
        Ok(())
    }

    fn flush(&self) {}
}

impl From<FastraceAppend> for AppendImpl {
    fn from(append: FastraceAppend) -> Self {
        AppendImpl::Fastrace(append)
    }
}