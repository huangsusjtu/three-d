use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::fmt;
use std::hash::{BuildHasherDefault, Hasher};
use std::ops::Deref;
use std::sync::Arc;

/// A hasher for `TypeId`s that takes advantage of its known characteristics.
///
/// Author of `anymap` crate has done research on the topic:
/// https://github.com/chris-morgan/anymap/blob/2e9a5704/src/lib.rs#L599
#[derive(Debug, Default)]
struct NoOpHasher(u64);

impl Hasher for NoOpHasher {
    fn write(&mut self, _bytes: &[u8]) {
        unimplemented!("This NoOpHasher can only handle u64s")
    }

    fn write_u64(&mut self, i: u64) {
        self.0 = i;
    }

    fn finish(&self) -> u64 {
        self.0
    }
}

/// A type map for request extensions.
///
/// All entries into this map must be owned types (or static references).
#[derive(Default)]
pub struct App {
    /// Use AHasher with a std HashMap with for faster lookups on the small `TypeId` keys.
    map: HashMap<TypeId, Box<dyn Any>, BuildHasherDefault<NoOpHasher>>,
}

impl App {
    /// Creates an empty `Extensions`.
    #[inline]
    pub fn new() -> App {
        App {
            map: HashMap::default(),
        }
    }

    /// Insert an item into the map.
    ///
    /// If an item of this type was already stored, it will be replaced and returned.
    pub fn insert<T: 'static>(&mut self, val: T) -> Option<T> {
        self.map
            .insert(TypeId::of::<T>(), Box::new(val))
            .and_then(downcast_owned)
    }

    /// Check if map contains an item of a given type.
    pub fn contains<T: 'static>(&self) -> bool {
        self.map.contains_key(&TypeId::of::<T>())
    }

    /// Get a reference to an item of a given type.
    pub fn get<T: 'static>(&self) -> Option<&T> {
        self.map
            .get(&TypeId::of::<T>())
            .and_then(|boxed| boxed.downcast_ref())
    }

    /// Get a mutable reference to an item of a given type.
    pub fn get_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.map
            .get_mut(&TypeId::of::<T>())
            .and_then(|boxed| boxed.downcast_mut())
    }

    /// Remove an item from the map of a given type.
    ///
    /// If an item of this type was already stored, it will be returned.
    pub fn remove<T: 'static>(&mut self) -> Option<T> {
        self.map.remove(&TypeId::of::<T>()).and_then(downcast_owned)
    }

    /// Clear the `Extensions` of all inserted extensions.
    #[inline]
    pub fn clear(&mut self) {
        self.map.clear();
    }

    /// Extends self with the items from another `Extensions`.
    pub fn extend(&mut self, other: App) {
        self.map.extend(other.map);
    }
}

impl fmt::Debug for App {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Extensions").finish()
    }
}

fn downcast_owned<T: 'static>(boxed: Box<dyn Any>) -> Option<T> {
    boxed.downcast().ok().map(|boxed| *boxed)
}

//
// #[doc(alias = "state")]
// #[derive(Debug)]
// pub struct Data<T: ?Sized>(Arc<T>);
//
// impl<T> Data<T> {
//     /// Create new `Data` instance.
//     pub fn new(state: T) -> Data<T> {
//         Data(Arc::new(state))
//     }
// }
//
// impl<T: ?Sized> Data<T> {
//     /// Returns reference to inner `T`.
//     pub fn get_ref(&self) -> &T {
//         self.0.as_ref()
//     }
//
//     /// Unwraps to the internal `Arc<T>`
//     pub fn into_inner(self) -> Arc<T> {
//         self.0
//     }
// }
//
// impl<T: ?Sized> Deref for Data<T> {
//     type Target = Arc<T>;
//
//     fn deref(&self) -> &Arc<T> {
//         &self.0
//     }
// }
//
// impl<T: ?Sized> Clone for Data<T> {
//     fn clone(&self) -> Data<T> {
//         Data(Arc::clone(&self.0))
//     }
// }
//
// impl<T: ?Sized> From<Arc<T>> for Data<T> {
//     fn from(arc: Arc<T>) -> Self {
//         Data(arc)
//     }
// }
//
// impl<T: Default> Default for Data<T> {
//     fn default() -> Self {
//         Data::new(T::default())
//     }
// }
