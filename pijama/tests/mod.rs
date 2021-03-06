macro_rules! check {
    ($name:ident) => {
        #[test]
        fn $name() {
            let config = pijama_interface::Config {
                path: concat!("tests/", stringify!($name), ".pj").into(),
                codegen: false,
            };

            pijama_interface::Compiler::new().run(config);
        }
    };
}

check!(fibonacci);
check!(apply);
check!(arithmetic);
