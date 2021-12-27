use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

fn main() {
    let shared_map = Rc::new(RefCell::new(HashMap::new()));

    // using the `try` variants is safer since we now get errors at compile time rather than at
    // run time in case of Borrow Checker violations.
    if let Ok(mut mut_map) = shared_map.try_borrow_mut() {
        mut_map.insert("africa", 92388);
        mut_map.insert("kyoto", 12345);
        mut_map.insert("piccadilly", 90201);
        mut_map.insert("marbles", 43217);
    }

    for (k, v) in shared_map.borrow().iter() {
        println!("{:?} => {:?}", k, v);
    }
}
