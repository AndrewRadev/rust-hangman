use hangman::wordlist::Wordlist;

#[test]
fn test_initialize_wordlist() {
    let data: &[u8] = br"
        one
        two
        three
    ";
    let wordlist = Wordlist::from_io(data);

    assert_eq!(wordlist.get(1), Some(&String::from("two")));
}

#[test]
fn test_wordlist_ignores_invalid_words() {
    let data: &[u8] = br"
        _foo_

        one

        123
    ";
    let wordlist = Wordlist::from_io(data);

    assert_eq!(wordlist.get(0), Some(&String::from("one")));
    assert_eq!(wordlist.get(1), None);
}

#[test]
#[should_panic]
fn test_invalid_wordlist_empty() {
    let data: &[u8] = br"";
    Wordlist::from_io(data).random();
}

#[test]
fn test_random_word() {
    let data: &[u8] = br"
        one
    ";
    let wordlist = Wordlist::from_io(data);

    assert_eq!(wordlist.random(), &String::from("one"));
}
