/// A trait that is automatically implemented for any enum type with the attribute `#[derive(Tag)]`.
pub trait TaggedEnum {
	type Tag;
	fn tag(&self) -> Self::Tag;
}
