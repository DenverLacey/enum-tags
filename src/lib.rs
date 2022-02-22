pub use enum_tags_macros::*;
pub use enum_tags_traits::*;

mod fail_tests {
	//! # Struct Test
	//! ```compile_fail
	//! use enum_tags::Tag;
	//! #[derive(Tag)]
	//! struct Bad {}
	//! ```

	//! # Union Test
	//! ```compile_fail
	//! use enum_tags::Tag;
	//! #[derive(Tag)]
	//! union Bad {}
	//! ```
}
