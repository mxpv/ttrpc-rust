// Copyright (c) 2019 Ant Financial
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

mod protocols;

use std::thread;
use std::env;

use nix::sys::socket::*;
use nix::unistd::close;

use ttrpc::client::Client;
use log::LevelFilter;

fn main() {
    //simple_logging::log_to_stderr(LevelFilter::Trace);

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Usage: {} unix_addr", args[0]);
    }

    let c = Client::from_unix_addr(&args[1]).unwrap();
    let hc = protocols::health_ttrpc::HealthClient::new(c.clone());
    let ac = protocols::agent_ttrpc::AgentServiceClient::new(c);

    let thc = hc.clone();
    let tac = ac.clone();
    let t = thread::spawn(move|| {
        let req = protocols::health::CheckRequest::new();

        println!("thread check: {:?}", thc.check(&req, 0));

        println!("thread version: {:?}", thc.version(&req, 0));

        let show = match tac.list_interfaces(&protocols::agent::ListInterfacesRequest::new(), 0) {
            Err(e) => {format!("{:?}", e)},
            Ok(s) => {format!("{:?}", s)},
        };
        println!("thread list_interfaces: {}", show);
    });

    println!("main check: {:?}", hc.check(&protocols::health::CheckRequest::new(), 0));

    let show = match ac.online_cpu_mem(&protocols::agent::OnlineCPUMemRequest::new(), 0) {
        Err(e) => {format!("{:?}", e)},
        Ok(s) => {format!("{:?}", s)},
    };
    println!("main online_cpu_mem: {}", show);

    t.join().unwrap();

    close(fd);
}
