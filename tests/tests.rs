#![feature(plugin)]
#![plugin(rust_dsl)]

struct Builder {
    counter: u32
}

impl Builder {
    pub fn build<F>(build: F) -> Builder where F: FnOnce(&mut Builder) {
        let mut builder = Builder { counter: 0u32 };
        build(&mut builder);
        builder
    }

    pub fn dsl_method(&mut self) {
        self.counter += 1;
    }
}

#[test]
fn test() {
    let builder = Builder::build(dsl!{|builder| {
        dsl_method();
        builder.dsl_method()
    }});

    assert_eq!(builder.counter, 2);
}

// #[test]
// fn test() {
//     dsl!(1 + 1)
// }