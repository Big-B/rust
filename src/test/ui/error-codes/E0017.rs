static X: i32 = 1;
const C: i32 = 2;
static mut M: i32 = 3;

const CR: &'static mut i32 = &mut C; //~ ERROR E0658
static STATIC_REF: &'static mut i32 = &mut X; //~ ERROR E0658
                                              //~| ERROR E0019
                                              //~| ERROR cannot borrow
static CONST_REF: &'static mut i32 = &mut C; //~ ERROR E0658
static STATIC_MUT_REF: &'static mut i32 = unsafe { &mut M }; //~ ERROR E0658
fn main() {}
