
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Material {
    pub entry_point: &'static str,
    pub name: &'static str,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct MaterialHandle(pub usize);
impl MaterialHandle {
    pub const NONE: Self = Self(0);
}
