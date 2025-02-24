use word_ladder::ladder_length;

#[test]
fn test_ladder_length() {
    let begin_word = String::from("hit");
    let end_word = String::from("cog");
    let word_list = vec![
        String::from("hot"),
        String::from("dot"),
        String::from("dog"),
        String::from("lot"),
        String::from("log"),
        String::from("cog"),
    ];
    assert_eq!(ladder_length(begin_word, end_word, word_list), 5);
}

#[test]
fn test_no_possible_transformation() {
    let begin_word = String::from("hit");
    let end_word = String::from("cog");
    let word_list = vec![
        String::from("hot"),
        String::from("dot"),
        String::from("dog"),
        String::from("lot"),
        String::from("log"),
    ];
    assert_eq!(ladder_length(begin_word, end_word, word_list), 0);
}