#[allow(unused_imports)]
use std::sync::{Arc, Mutex, MutexGuard};

/// Generates a "manager" for the given "thing". The "thing" is of type `$field_type`. The
/// "manager" wraps it in a lock (`Mutex`), which is wrapped in an arc (`Arc`). One
/// constraint is that the field type has to be `Default`.
///
/// - `$struct_name` = The name of the generated struct (the "manager").
/// - `$field_name` = The name of the instance in the generated struct.
/// - `$field_type` = The type of the instance in the generated struct.
#[macro_export]
macro_rules! make_manager {
  ($struct_name: ident manages { $field_name: ident: $field_type: ty } ) => {
    #[derive(Debug)]
    struct $struct_name
    where
      $field_type: Default,
    {
      $field_name: Arc<Mutex<$field_type>>,
    }

    impl Default for $struct_name {
      fn default() -> Self {
        Self {
          $field_name: Arc::new(Mutex::new(Default::default())),
        }
      }
    }

    #[allow(dead_code)]
    impl $struct_name {
      pub fn get_arc_clone(&self) -> Arc<Mutex<$field_type>> {
        self.$field_name.clone()
      }

      pub fn set_field(
        &self,
        value: $field_type,
      ) {
        *self.$field_name.lock().unwrap() = value;
      }

      pub fn get_field(&self) -> MutexGuard<$field_type> {
        self.$field_name.lock().unwrap()
      }

      pub fn get_from_arc(my_arc: &Arc<Mutex<$field_type>>) -> MutexGuard<$field_type> {
        my_arc.lock().unwrap()
      }
    }
  };
}
