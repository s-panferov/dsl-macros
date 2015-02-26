#![feature(plugin)]
#![plugin(dsl_macros)]

struct Builder {
    counter: u32
}

impl Builder {
    pub fn build<F>(build: F) -> Builder where F: FnOnce(&mut Builder) {
        let mut builder = Builder { counter: 0u32 };
        build(&mut builder);
        builder
    }

    pub fn build_two_args<F>(build: F) -> Builder where F: FnOnce(&mut Builder, u32) {
        let mut builder = Builder { counter: 0u32 };
        build(&mut builder, 0u32);
        builder
    }

    pub fn inc(&mut self) {
        self.counter += 1;
    }

    pub fn add(&mut self, n: u32) {
        self.counter += n;
    }

    pub fn dec<T>(&mut self) {
        self.counter -= 1;
    }
}

#[test]
fn test_semi() {
    let builder = Builder::build(dsl!(|builder| {
        inc();
    }));

    assert_eq!(builder.counter, 1);
}

#[test]
fn test_expr() {
    let builder = Builder::build(dsl!{|builder| {
        inc();
    }});

    assert_eq!(builder.counter, 1);
}

#[test]
fn test_doesnt_rewrite_normal_calls() {
    let builder = Builder::build(dsl!(|builder| {
        builder.inc();
        inc();
    }));

    assert_eq!(builder.counter, 2);
}

#[test]
fn test_doesnt_rewrite_other_stuff() {
    let builder = Builder::build(dsl!(|builder| {
        let a = 10u32;
        add(a);
    }));

    assert_eq!(builder.counter, 10);
}

#[test]
fn test_two_args() {
    let builder = Builder::build_two_args(dsl!(|builder, _other| {
        builder.inc();
        inc();
    }));

    assert_eq!(builder.counter, 2);
}

#[test]
fn test_type_args() {
    let builder = Builder::build(dsl!(|builder| {
        inc();
        dec::<u32>();
    }));

    assert_eq!(builder.counter, 0);
}

#[test]
fn test_nested() {
    let builder = Builder::build(dsl!(|builder| {
        Builder::build(dsl!(|builder| {
            inc()
        }));

        inc()
    }));

    assert_eq!(builder.counter, 1);
}
