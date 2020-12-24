#[macro_use]
extern crate action_make_struct_fields_public;

#[cfg(test)]
mod test {
    mod another_crate {

        #[make_public]
        #[derive(Default)]
        struct Foo {
            a: u32,
            b: i32,
        }

        // #[make_public]
        // ^^^^^^^^^^^^^^ `pub` not permitted here because it's implied
        #[derive(Debug)]
        pub enum Bar {
            A,
            B,
        }
    }

    #[test]
    fn case() {
        let foo = another_crate::Foo::default();
        println!("{}", foo.a);
        println!("{:?}", another_crate::Bar::A);
    }
}
