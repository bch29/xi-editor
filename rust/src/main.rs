// Copyright 2016 Google Inc. All rights reserved.
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

extern crate serde;
extern crate serde_json;
extern crate time;

extern crate xi_rope;
extern crate xi_unicode;

use std::io;
use std::io::BufRead;

#[macro_use]
mod macros;

mod tabs;
mod editor;
mod view;
mod linewrap;
mod rpc;

use tabs::Tabs;
use rpc::Request;

pub fn handle_req(request: Request, tabs: &mut Tabs) {
    match request {
        Request::TabCommand { id, tab_command } => {
            if let Some(result) = tabs.do_rpc(tab_command) {
                rpc::respond(&result, id);
            } else if let Some(id) = id {
                print_err!("RPC with id={:?} not responded", id);
            }
        }
    }
}


fn main() {
    let stdin = io::stdin();
    let mut stdin_handle = stdin.lock();
    let mut buf = String::new();
    let mut tabs = Tabs::new();

    while stdin_handle.read_line(&mut buf).is_ok() {
        if buf.is_empty() {
            break;
        }

        print_err!("to core: {:?}", buf);
        if let Ok(req) = serde_json::from_slice::<Request>(buf.as_bytes()) {
            handle_req(req, &mut tabs);
        }

        buf.clear();
    }
}
