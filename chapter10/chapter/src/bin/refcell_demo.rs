use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

fn main() {
    let shared_map = Rc::new(RefCell::new(HashMap::new()));

    {
        // this scoping is necessary since without it, the program would fail with an "already
        // mutably borrowed" error, but at *runtime*, *not compile time* due to the nature of
        // Refcells.
        let mut mut_map = shared_map.borrow_mut();
        mut_map.insert("africa", 92388);
        mut_map.insert("kyoto", 12345);
        mut_map.insert("piccadilly", 90201);
        mut_map.insert("marbles", 43217);
    }

    for (k, v) in shared_map.borrow().iter() {
        println!("{:?} => {:?}", k, v);
    }
}
