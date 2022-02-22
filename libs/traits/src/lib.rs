pub trait TaggedEnum {
	type Tag;
	fn tag(&self) -> Self::Tag;
}
