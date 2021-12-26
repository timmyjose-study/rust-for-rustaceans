#[repr(packed)]
struct Packed {
    f1: u8,
    f2: u16,
}

fn main() {
    let packed = Packed { f1: 1, f2: 3 };
    let raw_f2 = std::ptr::addr_of!(packed.f2);
    assert_eq!(unsafe { raw_f2.read_unaligned() }, 3);

    // the following is undefined behaviour
    let raw_f2_ptr = &packed.f2;
    println!("{}", *raw_f2_ptr);
}
