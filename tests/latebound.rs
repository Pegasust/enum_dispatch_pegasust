#![deny(late_bound_lifetime_arguments)]

use enum_dispatch::enum_dispatch;

#[enum_dispatch]
pub trait FooAPI {
    fn foo_warn<'a>(&mut self) -> &'a u32 {
        unimplemented!()
    }

    fn foo_err<'a, 'b>(&'a self, x: &'b u32) -> &'b u32 {
        x
    }
}

#[enum_dispatch(FooAPI)]
pub enum Foo {
    Something,
}

pub struct Something(u32);

impl FooAPI for Something {}

#[test]
fn main() {
    assert_eq!(*Something(2).foo_err(&8), 8);
}
