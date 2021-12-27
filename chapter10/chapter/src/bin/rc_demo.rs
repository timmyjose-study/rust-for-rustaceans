use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Owner {
    name: String,
    // this must be Weak so that dropping a Gadget does not affect the dropability of Owner
    children: RefCell<Vec<Weak<Gadget>>>,
}

#[derive(Debug)]
struct Gadget {
    id: usize,
    // this must be an Rc so that dropping the owner (when ref count reaches 0) drops this gadget
    owner: Rc<Owner>,
}

fn main() {
    let owner = Rc::new(Owner {
        name: String::from("Bob"),
        children: RefCell::new(Vec::new()),
    });

    let gadget1 = Rc::new(Gadget {
        id: 1,
        owner: Rc::clone(&owner),
    });

    owner.children.borrow_mut().push(Rc::downgrade(&gadget1));

    println!(
        "Strong count = {}, weak_count = {}",
        Rc::strong_count(&owner),
        Rc::weak_count(&owner)
    );

    let gadget2 = Rc::new(Gadget {
        id: 2,
        owner: Rc::clone(&owner),
    });

    println!(
        "Strong count = {}, weak count = {}",
        Rc::strong_count(&owner),
        Rc::weak_count(&owner)
    );

    owner.children.borrow_mut().push(Rc::downgrade(&gadget2));

    println!("Here are the children for owner {:?}...", owner.name);

    for child in owner.children.borrow().iter() {
        if let Some(child) = child.upgrade() {
            println!("id {}, weak count = {}", child.id, Rc::weak_count(&child));
        }
    }

    println!("Dropping owner...");
    drop(owner);

    println!(
        "Strong count = {}, weak count = {}",
        Rc::strong_count(&gadget1.owner),
        Rc::weak_count(&gadget2.owner)
    );

    println!("gagdet1 = {:?}, gadget2 = {:?}", gadget1, gadget2);
}
