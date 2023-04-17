/*  Copyright © 2023 Seth Junot
 *
 *  This program is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation, version 3 of the License.
 *
 *  This program is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *  along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

//! OxForth - Command-line interface / binary entry point

use std::process::exit;

use arguments::{Behavior, Options};

/// Where can a user find this source code?
const SOURCE_LOCATION: &str = "https://github.com/xSetech/OxForth";

/// What version, tag, or commit identifies this code?
const SOURCE_VERSION: &str = "0.2";

pub mod arguments;
pub mod repl;

/// Entry point
fn main() {
    println!("OxForth {} - {}", SOURCE_VERSION, SOURCE_LOCATION);

    let arg_parse_result = arguments::parse();
    if let Err(err) = arg_parse_result {
        eprintln!("{}", err.msg);
        eprintln!("note: pass --help to have helpful information printed");
        exit(1);
    }

    let (behavior, options): (Behavior, Options) = arg_parse_result.unwrap();
    match behavior {
        Behavior::HELP => {
            arguments::help();
        },
        Behavior::REPL => {
            repl::repl(&options);
        }
    }

}
