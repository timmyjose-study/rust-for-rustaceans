mod registers {
    static mut R1: Register = Register {
        name: "r1",
        state: RegisterState::Off,
    };

    static mut R2: Register = Register {
        name: "r2",
        state: RegisterState::Off,
    };

    struct Register {
        name: &'static str,
        state: RegisterState,
    }

    #[derive(PartialEq, Eq)]
    enum RegisterState {
        On,
        Off,
    }

    pub fn set_off(name: &'static str) {
        match name {
            "r1" => unsafe {
                if R1.state == RegisterState::On {
                    println!("Switching r1 off");
                    R1.state = RegisterState::Off;
                }
            },
            "r2" => unsafe {
                if R2.state == RegisterState::On {
                    println!("Switching r2 off");
                    R2.state = RegisterState::Off;
                }
            },
            _ => panic!("register not found"),
        }
    }

    pub fn set_on(name: &'static str) {
        match name {
            "r1" => unsafe {
                if R1.state == RegisterState::Off {
                    println!("Switching r1 on");
                    R1.state = RegisterState::On;
                }
            },
            "r2" => unsafe {
                if R2.state == RegisterState::Off {
                    println!("Switching r2 on");
                    R2.state = RegisterState::On;
                }
            },
            _ => panic!("register not found"),
        }
    }
}

use std::marker::PhantomData;

pub struct On;
pub struct Off;
pub struct Pair<R1, R2>(PhantomData<(R1, R2)>);

impl Pair<Off, Off> {
    pub fn get() -> Option<Self> {
        // this ensures a singleton pair of registers
        static mut PAIR_TAKEN: bool = false;

        if unsafe { PAIR_TAKEN } {
            None
        } else {
            registers::set_off("r1");
            registers::set_off("r2");
            unsafe { PAIR_TAKEN = true };
            Some(Pair(PhantomData))
        }
    }

    pub fn first_on(self) -> Pair<On, Off> {
        registers::set_on("r1");
        Pair(PhantomData)
    }

    pub fn second_on(self) -> Pair<Off, On> {
        registers::set_on("r2");
        Pair(PhantomData)
    }
}

impl Pair<On, Off> {
    pub fn off(self) -> Pair<Off, Off> {
        registers::set_off("r1");
        Pair(PhantomData)
    }
}

impl Pair<Off, On> {
    pub fn off(self) -> Pair<Off, Off> {
        registers::set_off("r2");
        Pair(PhantomData)
    }
}

fn main() {
    let p = Pair::get();

    if let Some(p) = p {
        // returns Pair<On, Off>
        let p = p.first_on();
        // this method does not exist in Pair<On, Off>
        //p.second_on();

        // but this one does, and returns a Pair<Off, Off>
        let p = p.off();

        // now we can turn the second one on
        // which returns a Pair<Off, On>
        let p = p.second_on();

        // as before, this method does not exist
        // in Pair<Off, On>
        //let p = p.first_on();

        // but this one does
        let p = p.off();
    }
}
