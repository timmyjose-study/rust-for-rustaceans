fn main() {
    let mut l1 = line_number();
    for _ in 0..5 {
        l1();
    }

    let mut l2 = line_number();
    for _ in 0..5 {
        l2();
    }

    for _ in 0..5 {
        l1();
    }

    call_line_number(l1);
}

fn call_line_number(mut ln: impl FnMut() -> ()) {
    for _ in 0..10 {
        ln();
    }
}

fn line_number() -> impl FnMut() -> () {
    let mut line_no = 0;

    move || {
        line_no += 1;
        println!("Current line number = {}", line_no);
    }
}
