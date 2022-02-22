use enum_tags::*;
use std::fmt::Debug;

#[derive(Tag)]
enum NoFields {
	A,
	B,
	C = 5,
}

#[derive(Tag)]
enum WithUnnamedFields {
	A(u8),
	B(bool),
	C(char),
}

#[derive(Tag)]
enum WithNamedFields {
	A { byte: u8 },
	B { boolean: bool },
	C { character: char },
}

#[derive(Tag)]
enum WithGenerics<T>
where
	T: ?Sized,
{
	A(Box<T>),
	B,
}

#[derive(Tag)]
pub enum PubAndEmpty {}

#[test]
fn no_fields() {
	assert_eq!(NoFields::A as u8, NoFieldsTag::A as u8);
	assert_eq!(NoFields::B as u8, NoFieldsTag::B as u8);
	assert_eq!(NoFields::C as u8, NoFieldsTag::C as u8);
}

#[test]
fn with_fields() {
	assert_eq!(WithUnnamedFieldsTag::A as u8, 0);
	assert_eq!(WithUnnamedFieldsTag::B as u8, 1);
	assert_eq!(WithUnnamedFieldsTag::C as u8, 2);
}

#[test]
fn tagged_enum() {
	assert_eq!(NoFields::A as u8, <NoFields as TaggedEnum>::Tag::A as u8);
	assert_eq!(NoFields::B as u8, <NoFields as TaggedEnum>::Tag::B as u8);
	assert_eq!(NoFields::C as u8, <NoFields as TaggedEnum>::Tag::C as u8);
}

#[test]
fn tag() {
	let no_fields_tag = NoFields::A.tag();
	assert_eq!(no_fields_tag, NoFieldsTag::A);

	let with_unnamed_fields_tag = WithUnnamedFields::A(0).tag();
	assert_eq!(with_unnamed_fields_tag, WithUnnamedFieldsTag::A);

	let with_named_fields_tag = WithNamedFields::A { byte: 0 }.tag();
	assert_eq!(with_named_fields_tag, WithNamedFieldsTag::A);

	let with_generics_tag = WithGenerics::<i32>::B.tag();
	assert_eq!(with_generics_tag, WithGenericsTag::B);
}

fn do_something_with_tag<E>(tag: E::Tag)
where
	E: TaggedEnum,
	E::Tag: Debug,
{
	println!("{:?}", tag);
}

#[test]
fn generic_function() {
	do_something_with_tag::<NoFields>(NoFieldsTag::A);
	do_something_with_tag::<WithUnnamedFields>(WithUnnamedFieldsTag::A);
	do_something_with_tag::<WithGenerics<i32>>(WithGenericsTag::A);
}
