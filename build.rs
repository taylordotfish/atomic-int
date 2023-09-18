/*
 * Copyright 2023 taylor.fish <contact@taylor.fish>
 *
 * This file is part of atomic-int.
 *
 * atomic-int is licensed under the Apache License, Version 2.0
 * (the "License"); you may not use atomic-int except in compliance
 * with the License. You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use std::env;
use std::io;
use std::path::PathBuf;
use std::process::{Command, Stdio};

fn has_atomic(name: &str) -> io::Result<bool> {
    let mut out = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    out.push("feature-test");
    Ok(Command::new(env::var_os("RUSTC").unwrap())
        .arg("has_atomic.rs")
        .arg("-o")
        .arg(out)
        .arg("--crate-type=lib")
        .arg("--target")
        .arg(env::var_os("TARGET").unwrap())
        .arg("--edition=2018")
        .arg("--cfg")
        .arg(format!("test_has_{name}_atomic"))
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()?
        .success())
}

macro_rules! test_atomic {
    ($name:literal) => {
        if cfg!(feature = $name) && has_atomic($name)? {
            println!(concat!("cargo:rustc-cfg=has_", $name, "_atomic"));
        }
    };
}

fn main() -> io::Result<()> {
    env::set_current_dir("feature-test")?;
    test_atomic!("c_char");
    test_atomic!("c_schar");
    test_atomic!("c_uchar");
    test_atomic!("c_short");
    test_atomic!("c_ushort");
    test_atomic!("c_int");
    test_atomic!("c_uint");
    test_atomic!("c_long");
    test_atomic!("c_ulong");
    test_atomic!("c_longlong");
    test_atomic!("c_ulonglong");
    println!("cargo:rerun-if-changed=feature-test");
    Ok(())
}
