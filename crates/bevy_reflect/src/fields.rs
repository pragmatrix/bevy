use crate::{Reflect, TypeInfo, Typed};
use std::any::{Any, TypeId};

/// The named field of a reflected struct.
#[derive(Clone, Debug)]
pub struct NamedField {
    name: &'static str,
    type_name: &'static str,
    type_id: TypeId,
    type_info: &'static TypeInfo,
    #[cfg(feature = "documentation")]
    docs: Option<&'static str>,
}

impl NamedField {
    /// Create a new [`NamedField`].
    pub fn new<T: Reflect + Typed>(name: &'static str) -> Self {
        Self {
            name,
            type_name: std::any::type_name::<T>(),
            type_id: TypeId::of::<T>(),
            type_info: T::type_info(),
            #[cfg(feature = "documentation")]
            docs: None,
        }
    }

    /// Sets the docstring for this field.
    #[cfg(feature = "documentation")]
    pub fn with_docs(self, docs: Option<&'static str>) -> Self {
        Self { docs, ..self }
    }

    /// The name of the field.
    pub fn name(&self) -> &'static str {
        self.name
    }

    /// The [type name] of the field.
    ///
    /// [type name]: std::any::type_name
    pub fn type_name(&self) -> &'static str {
        self.type_name
    }

    /// The [`TypeId`] of the field.
    pub fn type_id(&self) -> TypeId {
        self.type_id
    }

    /// Check if the given type matches the field type.
    pub fn is<T: Any>(&self) -> bool {
        TypeId::of::<T>() == self.type_id
    }

    /// The [`TypeInfo`] of the field.
    pub fn type_info(&self) -> &'static TypeInfo {
        self.type_info
    }

    /// The docstring of this field, if any.
    #[cfg(feature = "documentation")]
    pub fn docs(&self) -> Option<&'static str> {
        self.docs
    }
}

/// The unnamed field of a reflected tuple or tuple struct.
#[derive(Clone, Debug)]
pub struct UnnamedField {
    index: usize,
    type_name: &'static str,
    type_id: TypeId,
    // Indirection to break infinite `type_info()` invocation cycles when Unnamed field is created
    // from within a `type_info()` call.
    type_info: fn() -> &'static TypeInfo,
    #[cfg(feature = "documentation")]
    docs: Option<&'static str>,
}

impl UnnamedField {
    pub fn new<T: Reflect + Typed>(index: usize) -> Self {
        Self {
            index,
            type_name: std::any::type_name::<T>(),
            type_id: TypeId::of::<T>(),
            type_info: || T::type_info(),
            #[cfg(feature = "documentation")]
            docs: None,
        }
    }

    /// Sets the docstring for this field.
    #[cfg(feature = "documentation")]
    pub fn with_docs(self, docs: Option<&'static str>) -> Self {
        Self { docs, ..self }
    }

    /// Returns the index of the field.
    pub fn index(&self) -> usize {
        self.index
    }

    /// The [type name] of the field.
    ///
    /// [type name]: std::any::type_name
    pub fn type_name(&self) -> &'static str {
        self.type_name
    }

    /// The [`TypeId`] of the field.
    pub fn type_id(&self) -> TypeId {
        self.type_id
    }

    /// Check if the given type matches the field type.
    pub fn is<T: Any>(&self) -> bool {
        TypeId::of::<T>() == self.type_id
    }

    /// The [`TypeInfo`] of the field.
    pub fn type_info(&self) -> &'static TypeInfo {
        (self.type_info)()
    }

    /// The docstring of this field, if any.
    #[cfg(feature = "documentation")]
    pub fn docs(&self) -> Option<&'static str> {
        self.docs
    }
}
