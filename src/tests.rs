#[test]
fn test() {
    use {
        crate::parser::cake_mods::gen_cake,
    };

    let cake = gen_cake();
    println!("{}", cake);

    match cake.parse(" \n \nfn test() \n \n \"\\[\\]\" \n; end ", 1024) {
        Ok(option) => match option {
            Some(tree) => println!("{}", tree),
            None => panic!("Parsing Failure"),
        },
        Err(e) => panic!("{:?}", e),
    };
}
