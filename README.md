# enum-tags
A Derive-Macro library that generates a companion tag-enum for any enum so that variants can be referred to without specifying fields. 

# Usage
Add this to your Cargo.toml:
```
[dependencies]
enum-tags = "0.1.0"
```

Then derive [`Tag`] for any enum you want to generate a companion tag-enum for.
```rust
#[derive(Tag)]
enum MyEnum {
	A,
	B = 1024,
	C(char),
	D { x: i32, y: i32 },
}
```

The generated enum will look like this:
```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum MyEnumTag {
	A,
	B = 1024,
	C,
	D,
}
```

An `impl` for the [`TaggedEnum`] trait will also be generated to allow conversion from your enum type to the tag-enum.
The generated `impl` will look like this:
```rust
impl ::enum_tags_traits::TaggedEnum for MyEnum {
	type Tag = MyEnumTag;
	fn tag(&self) -> Self::Tag {
		match *self {
			Self::A => Self::Tag::A,
			Self::B => Self::Tag::B,
			Self::C(_) => Self::Tag::C,
			Self::D { x: _, y: _ } => Self::Tag::D,
		}
	}
}
```
