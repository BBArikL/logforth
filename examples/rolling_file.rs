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

use log::LevelFilter;
use logforth::append;
use logforth::append::NonBlockingBuilder;
use logforth::append::RollingFileWriter;
use logforth::append::Rotation;
use logforth::filter;
use logforth::layout;
use logforth::logger::Dispatch;
use logforth::logger::Logger;

fn main() {
    let rolling = RollingFileWriter::builder()
        .rotation(Rotation::Minutely)
        .filename_prefix("example")
        .filename_suffix("log")
        .max_log_files(2)
        .build("logs")
        .unwrap();
    let (writer, _guard) = NonBlockingBuilder::default().finish(rolling);

    Logger::new()
        .dispatch(
            Dispatch::new()
                .filter(filter::LogLevel::new(LevelFilter::Trace))
                .layout(layout::SimpleJson)
                .append(append::RollingFile::new(writer)),
        )
        .apply()
        .unwrap();

    let repeat = 1;

    for i in 0..repeat {
        log::error!("Hello error!");
        log::warn!("Hello warn!");
        log::info!("Hello info!");
        log::debug!("Hello debug!");
        log::trace!("Hello trace!");

        if i + 1 < repeat {
            std::thread::sleep(std::time::Duration::from_secs(10));
        }
    }
}