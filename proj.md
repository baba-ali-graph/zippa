

===========================================


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
```