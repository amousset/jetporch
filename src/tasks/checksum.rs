// Jetporch
// Copyright (C) 2023 - Michael DeHaan <michael@michaeldehaan.net> + contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// at your option) any later version.
// 
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
// 
// You should have received a copy of the GNU General Public License
// long with this program.  If not, see <http://www.gnu.org/licenses/>.

use rs_sha512::{Sha512State,HasherContext};
use std::hash::BuildHasher;
use std::hash::Hasher;

pub fn sha512(data: &String) -> String {
    let mut sha512hasher = Sha512State::default().build_hasher();
    let bytes = data.as_bytes();
    sha512hasher.write(bytes);
    let _u64result = sha512hasher.finish();
    let bytes_result = HasherContext::finish(&mut sha512hasher);
    return format!("{bytes_result:02x}")
}
