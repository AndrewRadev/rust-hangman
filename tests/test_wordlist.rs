extern crate hangman;
use hangman::wordlist::Wordlist;

#[test]
fn test_something() {
    let data: &[u8] = br"
        one
        two
        three
    ";
    let wordlist = Wordlist::from_io(data);

    assert_eq!(wordlist.get(2), Some(&String::from("two")));
}
