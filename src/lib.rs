// Copyright 2014 The html5ever Project Developers. See the
// COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Look up the script for a character.
//!
//! ### Example
//!
//! ```
//! use unicode_script::{get_script, Script};
//!
//! assert_eq!(get_script('A'), Script::Latin);
//! assert_eq!(get_script('カ'), Script::Katakana);
//! ```

pub mod script;
pub mod tables;
pub use script::get_script;
pub use tables::Script;

/// The version of [Unicode](http://www.unicode.org/)
/// that this version of unicode-script is based on.
pub const UNICODE_VERSION: (u64, u64, u64) = (16, 0, 0);

#[cfg(test)]
mod test {
    use super::{get_script, Script};

    #[test]
    fn test_get_script() {
        assert_eq!(get_script('a'), Script::Latin);
        assert_eq!(get_script('.'), Script::Common);
        assert_eq!(get_script('カ'), Script::Katakana);
        assert_eq!(get_script('🦳'), Script::Common);
    }
}
