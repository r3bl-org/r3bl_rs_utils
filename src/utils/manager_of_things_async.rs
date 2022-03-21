#![allow(unused_imports)]
use tokio::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::sync::Arc;

/// Generates a "manager" for the given "thing". The "thing" is of type `$thing_type`. The
/// "manager" wraps it in a lock (`RwLock`), which is wrapped in an arc (`Arc`). One
/// constraint is that the field type has to be `Default`.
///
/// Nomenclature:
/// - `$struct_name` = The name of the generated struct (the "manager").
/// - `$thing_type` = The type of the instance in the generated struct.
/// - `wrapped_thing` = The name of the property managed by the generated struct.
/// - `locked_thing` = 🔒 Accessor gets `MutexGuard` to the "thing" (remember to drop it).
#[macro_export]
macro_rules! make_rwlock_manager_async {
  ($struct_name: ident manages $thing_type: ty ) => {
    #[derive(Debug)]
    struct $struct_name
    where
      $thing_type: Default + Send + Sync + 'static,
    {
      wrapped_thing: Arc<RwLock<$thing_type>>,
    }

    impl Default for $struct_name {
      fn default() -> Self {
        Self {
          wrapped_thing: Arc::new(RwLock::new(Default::default())),
        }
      }
    }

    impl $struct_name {
      /// Directly mutate `wrapped_thing`.
      pub async fn set_value_of_wrapped_thing(
        &self,
        value: $thing_type,
      ) {
        *self.wrapped_thing.write().await = value;
      }

      /// 🔒 Directly access `wrapped_thing` as write-locked.
      ///
      /// Make sure to drop the `MutexGuard` that is returned when you're done w/ it to
      /// prevent deadlock.
      // pub async fn get_locked_thing(
      //   &'static self
      // ) -> RwLockWriteGuard<'static, $thing_type> {
      //   self.wrapped_thing.write().await
      // }

      /// 🔒 Directly access `wrapped_thing` as read-locked.
      ///
      /// Make sure to drop the `MutexGuard` that is returned when you're done w/ it to
      /// prevent deadlock.
      // pub async fn get_locked_thing_r(&'_ self) -> RwLockReadGuard<'_, $thing_type> {
      //   self.wrapped_thing.read().await
      // }

      /// Get a clone of the arc. This can be passed around safely, instead of passing the
      /// manager instance itself.
      pub fn get_arc(&self) -> Arc<RwLock<$thing_type>> {
        self.wrapped_thing.clone()
      }

      /// 🔒 Static method that allow you to indirectly access the wrapped_thing via `Arc`
      /// produced by `get_arc()`.
      ///
      /// Make sure to drop the `MutexGuard` that is returned when you're done w/ it to
      /// prevent deadlock.
      pub async fn with_arc_get_locked_thing(
        my_arc: &'_ Arc<RwLock<$thing_type>>
      ) -> RwLockWriteGuard<'_, $thing_type> {
        my_arc.write().await
      }

      /// 🔒 Static method that allow you to indirectly access the wrapped_thing via `Arc`
      /// produced by `get_arc()`.
      ///
      /// Make sure to drop the `MutexGuard` that is returned when you're done w/ it to
      /// prevent deadlock.
      pub async fn with_arc_get_locked_thing_r(
        my_arc: &'_ Arc<RwLock<$thing_type>>
      ) -> RwLockReadGuard<'_, $thing_type> {
        my_arc.read().await
      }

      /// Static method that allow you to indirectly mutate the wrapped_thing via `Arc`
      /// produced by `get_arc()`.
      pub async fn with_arc_set_value_of_wrapped_thing(
        my_arc: &Arc<RwLock<$thing_type>>,
        value: $thing_type,
      ) {
        *my_arc.write().await = value;
      }
    }
  };
}
