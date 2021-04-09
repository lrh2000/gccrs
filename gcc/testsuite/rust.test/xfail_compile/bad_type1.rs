fn foo() -> i32 { // { dg-error "expected .i32. got .bool." }
    let logical: bool = 123; // { dg-error "expected .bool. got .<integer>." }
    // { dg-error "failure in setting up let stmt type" "" { target { *-*-* } } .-1 }
    logical
}

fn main() {
    let a: bool = foo(); // { dg-error "expected .bool. got .i32." }
    // { dg-error "failure in setting up let stmt type" "" { target { *-*-* } } .-1 }
}
