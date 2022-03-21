use r3bl_rs_utils::make_mutex_manager;
use std::sync::{Arc, Mutex, MutexGuard};

#[test]
fn test_make_manager_struct() {
  // Generate the struct.
  make_mutex_manager! { ThingManager manages i32 };

  // Create an manager_instance of the "manager" struct.
  let manager_instance = ThingManager::default();

  // 🔒 Each of the locked objects need to be wrapped in a block, or call `drop()` so the
  // mutex guard can be dropped and the tests won't deadlock.

  // 1. Test that `wrapped_thing` is created.
  {
    let locked_thing = manager_instance.wrapped_thing.lock().unwrap();
    assert_eq!(*locked_thing, 0);
  }

  // 2. Test that `get_locked_thing()` works.
  {
    let locked_thing = manager_instance.get_locked_thing();
    assert_eq!(*locked_thing, 0);
  }

  // 3. Test that `set_value_of_wrapped_thing()` works.
  {
    manager_instance.set_value_of_wrapped_thing(12);
    let locked_thing = manager_instance.get_locked_thing();
    assert_eq!(*locked_thing, 12);
  }

  // 4. Test that `get_arc()` => `with_arc_get_locked_thing()` &
  //    `with_arc_set_value_of_wrapped_thing()` works. Watch out for deadlock.
  {
    let arc_clone = manager_instance.get_arc();
    let locked_thing = ThingManager::with_arc_get_locked_thing(&arc_clone);
    assert_eq!(*locked_thing, 12);
    drop(locked_thing); // 🔒 Prevents deadlock below.

    ThingManager::with_arc_set_value_of_wrapped_thing(&arc_clone, 13);
    assert_eq!(*ThingManager::with_arc_get_locked_thing(&arc_clone), 13);
  }
}
