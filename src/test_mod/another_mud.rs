
    use crate::external;

    #[derive(Debug)]
    pub struct Lang{
        pub name: String,
        pub version: i32,

    }
    pub fn print_another_mod(){
        println!("this is another mod");
        let rust = Lang{
            name: "Rust".to_string(),
            version: 2,
        };

        println!("my best programming language is {:?}", {rust});


    }

    pub fn call_external(){
        external::ext();
    }
