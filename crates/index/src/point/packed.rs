use core::fmt;
use std::num::NonZeroU16;

pub(super) struct PackedVec<T>(Vec<T>);

impl<T> PackedVec<T> {
    // TODO fail insert if too large

    pub(super) fn next_id(&self) -> Option<PackedId> {
        PackedId::try_from(self.0.len()).ok()
    }

    pub(super) fn get(&self, id: PackedId) -> Option<&T> {
        self.0.get(usize::from(id))
    }

    pub(super) fn get_mut(&mut self, id: PackedId) -> Option<&mut T> {
        self.0.get_mut(usize::from(id))
    }

    // TODO document panics
    pub(super) fn push(&mut self, item: T) -> PackedId {
        let id = PackedId::try_from(self.0.len()).unwrap();
        self.0.push(item);
        id
    }

    // TODO document panics
    pub(super) fn remove(&mut self, id: PackedId) -> Removed<T> {
        let item = self.0.swap_remove(id.into());
        let old = PackedId::try_from(self.0.len()).unwrap();
        let swap = if old == id {
            None
        } else {
            Some(Swap { old, new: id })
        };

        Removed { item, swap }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct PackedId(NonZeroU16);

impl PackedId {
    pub(super) const FIRST: Self = PackedId(NonZeroU16::MIN);
}

impl fmt::Display for PackedId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} - 1", self.0)
    }
}

impl TryFrom<usize> for PackedId {
    type Error = &'static str;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        let value = u16::try_from(value).map_err(|_| "value is too large")?;
        let Some(value) = value.checked_add(1) else {
            return Err("value is too large");
        };
        Ok(Self(NonZeroU16::new(value).unwrap()))
    }
}

impl From<PackedId> for usize {
    fn from(id: PackedId) -> Self {
        usize::from(u16::from(id.0))
    }
}

pub(super) struct Removed<T> {
    pub(super) item: T,
    pub(super) swap: Option<Swap>,
}

pub(super) struct Swap {
    pub(super) old: PackedId,
    pub(super) new: PackedId,
}
