


--------BUILDING SCRIPT---------------------
```sh
cargo build --release --target=x86_64-unknown-linux-gnu --target=x86_64-apple-darwin --target=x86_64-pc-windows-msvc
```


------------------------------------------------

```rs
extern crate args_parser.rs
extern crate main

#[cfg(ts)]
mod test {
    fn panics_if_invalid_arg_passed() {
        let args = ["testing", "--source", "baba ali", "--dest", "okene"]
        let matches = app.get_matches_from(args)
        assert_panic!()
    }
}



fn args are parsed correctly() {
    let matches = app.get_matches_from([])
    assert_eq(matches.a, a)
    assert_eq(matches.b, b)
}


fn method_stuff() {
    return aaa
}

fn right_file_is_generated() {

}


fn error_is_wrong() {
    
}
```





```rs
// building a snake game with rust

/**
 * dependencies: rand & piston_window
 * 
 * */

// Top level constant declaration
const DIRECTION :f32 = 40.30


``


------------------------------------------------------

Can you help me generate a README.md file for my project. It is a terminal file/folder zipping application written in Rust. I want you to include standard sections found in the README of good open source projects. Using this snippet, I want you to use it to build the `usage section`:

```rust

pub struct ZippaArgs {
    #[clap(short = 's', long = "source")]
    pub source: String,

    #[clap(short = 'd', long = "dest")]
    pub dest: String,

    #[clap(short = 'c', long = "compression", default_value = "bzip2")]
    pub compression: String,

    #[clap(short = 'o', long = "override", takes_value = false)]
    pub over_ride: bool,
}

```