use r3bl_rs_utils::make_manager;
use std::sync::{Arc, Mutex, MutexGuard};

#[test]
fn test_make_manager_struct() {
  // Generate the struct.
  make_manager! {
    ThingManager manages { field_1: i32 }
  };

  // Create an instance of the "manager" struct.
  let instance = ThingManager::default();

  // 🔒 Each of these locked blocks need to be wrapped in a block, so the mutex guard can
  // be dropped and the tests won't deadlock.

  // 1. Test that `field_1` is created.
  {
    let field_1 = instance.field_1.lock().unwrap();
    assert_eq!(*field_1, 0);
  }

  // 2. Test that `get_field()` works.
  {
    let field_1 = instance.get_field();
    assert_eq!(*field_1, 0);
  }

  // 3. Test that `set_field()` works.
  {
    instance.set_field(12);
    let field_1 = instance.get_field();
    assert_eq!(*field_1, 12);
  }

  // 4. Test that `get_arc_clone()` & `get_from_arc()` works.
  {
    let arc_clone = instance.get_arc_clone();
    let field_1 = ThingManager::get_from_arc(&arc_clone);
    assert_eq!(*field_1, 12);
  }
}
