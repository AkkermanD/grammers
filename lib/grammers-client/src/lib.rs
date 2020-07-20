// Copyright 2020 - developers of the `grammers` project.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
mod client;
pub mod types;

pub use client::{Client, Config, SignInError};
use futures::channel::mpsc;
pub use grammers_mtsender::{AuthorizationError, InvocationError};

pub type UpdateStream = mpsc::Receiver<grammers_tl_types::enums::Updates>;
