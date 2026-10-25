use super::*;

use rstest::rstest;

#[rstest]
#[case(vec![],1)]
#[case(vec![1],2)]
#[case(vec![1,1],3)]
#[case(vec![1,1,3],4)]
#[case(vec![1,1,3,4],5)]
#[case(vec![1,1,1,4],5)]
#[case(vec![1],0)]
#[case(vec![1,1],1)]
#[case(vec![1,1,3],2)]
#[case(vec![1,1,3,4],3)]
#[case(vec![1,1,1,4,5],4)]
#[should_panic(expected = "Combination answer slots number does not match declared one")]
fn test_new_panics_if_answer_length_dot_not_match_slots_length(
    #[case] combination: Vec<usize>,
    #[case] total_slots: usize,
) {
    // act / assert
    Game::new_parameterized(true, total_slots, 6, 1, combination.to_vec());
}

#[rstest]
#[case(vec![1,1],2,1)]
#[case(vec![1,1,3],3,2)]
#[case(vec![1,1,3,4],4,2)]
#[case(vec![1,1,1,4],4,3)]
#[should_panic(expected = "Combination slots exceed colors while duplicated are prevented")]
fn test_new_panics_if_slots_exceed_colors_while_preventing_duplicated(
    #[case] combination: Vec<usize>,
    #[case] total_slots: usize,
    #[case] total_colors: usize,
) {
    // act / assert
    Game::new_parameterized(false, total_slots, total_colors, 1, combination.to_vec());
}

#[rstest]
#[case(vec![1,1],2)]
#[case(vec![1,1,3],3)]
#[case(vec![1,1,3,4],4)]
#[case(vec![1,1,1,4],4)]
#[case(vec![1,1,1,1],4)]
#[should_panic(expected = "Combination answer contains duplicates while prevented")]
fn test_new_panics_if_duplicated_while_prevented(
    #[case] combination: Vec<usize>,
    #[case] total_slots: usize,
) {
    // act / assert
    Game::new_parameterized(false, total_slots, 6, 1, combination.to_vec());
}

#[rstest]
#[case(vec![0,1],2,1)]
#[case(vec![1,2],2,1)]
#[case(vec![1,1,3],3,2)]
#[case(vec![1,1,3,4],4,3)]
#[case(vec![1,1,1,5],4,4)]
#[case(vec![1,1,1,1],4,0)]
#[should_panic(expected = "Combination answer contains values higher than permitted")]
fn test_new_panics_if_combination_contains_higher_values_than_permitted(
    #[case] combination: Vec<usize>,
    #[case] total_slots: usize,
    #[case] total_colors: usize,
) {
    // act / assert
    Game::new_parameterized(true, total_slots, total_colors, 1, combination.to_vec());
}

#[rstest]
#[should_panic(expected = "Combination tries number should be greater than 0")]
fn test_new_panics_if_tries_is_equal_to_zero() {
    // act / assert
    Game::new_parameterized(false, 4, 6, 0, vec![1, 1, 1, 4].to_vec());
}

#[rstest]
#[case(1)]
#[case(2)]
#[case(10)]
fn test_generate_random_colors_combination_fails_when_no_colors_and_duplicates(
    #[case] total_slots: usize,
) {
    // act
    let result = Game::generate_random_colors_combination(true, total_slots, 0);

    // assert
    assert!(result.is_err());
    assert_eq!(result.err().unwrap(), "There are no allowed colors");
}

#[rstest]
#[case(1, 0)]
#[case(2, 1)]
#[case(20, 10)]
fn test_generate_random_colors_combination_fails_when_less_colors_than_slots_and_no_duplicates(
    #[case] total_slots: usize,
    #[case] total_colors: usize,
) {
    // act
    let result = Game::generate_random_colors_combination(false, total_slots, total_colors);

    // assert
    assert!(result.is_err());
    assert_eq!(
        result.err().unwrap(),
        "Cannot generate a combination without duplicates when slots exceed colors"
    );
}

#[rstest]
#[case([1,2,3,4], vec![3,2,1])]
#[case([1,2,3,4], vec![2,1])]
#[case([1,2,3,4], vec![1])]
#[case([1,2,3,4], vec![])]
fn test_check_combination_handles_undersized_guess(
    #[case] combination: [usize; 4],
    #[case] combination_try: Vec<usize>,
) {
    // arrange
    let game = Game::new_parameterized(false, 4, 6, 1, combination.to_vec());

    // act
    let result = game.check_combination(&combination_try);

    // assert
    assert!(result.is_err());
    assert_eq!(
        result.err().unwrap(),
        Messages::combination_answer_slots_number_does_not_match_declared_one()
    );
}

#[rstest]
#[case([1,2,3,4], vec![5,4,3,2,1])]
#[case([1,2,3,4], vec![9,8,7,6,5,4,3,2,1])]
fn test_check_combination_handles_oversized_guess(
    #[case] combination: [usize; 4],
    #[case] combination_try: Vec<usize>,
) {
    // arrange
    let game = Game::new_parameterized(false, 4, 10, 1, combination.to_vec());

    // act
    let result = game.check_combination(&combination_try);

    // assert
    assert!(result.is_err());
    assert_eq!(
        result.err().unwrap(),
        Messages::combination_answer_slots_number_does_not_match_declared_one()
    );
}

#[rstest]
#[case([1,2,3,4], vec![1,0,0,0], 1)]
#[case([1,2,3,4], vec![0,2,0,0], 1)]
#[case([1,2,3,4], vec![0,0,3,0], 1)]
#[case([1,2,3,4], vec![0,0,0,4], 1)]
#[case([1,2,3,4], vec![1,2,0,0], 2)]
#[case([1,2,3,4], vec![0,2,3,0], 2)]
#[case([1,2,3,4], vec![0,0,3,4], 2)]
#[case([1,2,3,4], vec![1,0,0,4], 2)]
#[case([1,2,3,4], vec![1,2,3,0], 3)]
#[case([1,2,3,4], vec![0,2,3,4], 3)]
#[case([1,2,3,4], vec![1,0,3,4], 3)]
#[case([1,2,3,4], vec![1,2,3,4], 4)]
fn test_check_combination_handles_well_placed(
    #[case] combination: [usize; 4],
    #[case] combination_try: Vec<usize>,
    #[case] expected: usize,
) {
    // arrange
    let game = Game::new_parameterized(true, 4, 6, 1, combination.to_vec());

    // act
    let result = game.check_combination(&combination_try);

    // assert
    assert_eq!(result.unwrap().well_placed(), expected);
}

#[rstest]
#[case([1,1,2,2], vec![1,0,0,0], 1)]
#[case([1,1,2,2], vec![0,1,0,0], 1)]
#[case([1,1,2,2], vec![1,1,0,0], 2)]
#[case([1,1,2,2], vec![0,0,2,0], 1)]
#[case([1,1,2,2], vec![0,0,0,2], 1)]
#[case([1,1,2,2], vec![0,0,2,2], 2)]
#[case([1,1,1,2], vec![1,0,0,0], 1)]
#[case([1,1,1,2], vec![0,1,0,0], 1)]
#[case([1,1,1,2], vec![0,0,1,0], 1)]
#[case([1,1,1,2], vec![1,1,1,0], 3)]
#[case([1,2,2,2], vec![0,0,0,2], 1)]
#[case([1,2,2,2], vec![0,0,2,2], 2)]
#[case([1,2,2,2], vec![0,2,2,2], 3)]
#[case([1,1,1,1], vec![1,1,1,1], 4)]
fn test_check_combination_handles_well_placed_duplicates(
    #[case] combination: [usize; 4],
    #[case] combination_try: Vec<usize>,
    #[case] expected: usize,
) {
    // arrange
    let game = Game::new_parameterized(true, 4, 4, 1, combination.to_vec());

    // act
    let result = game.check_combination(&combination_try);

    // assert
    assert_eq!(result.unwrap().well_placed(), expected);
}

#[rstest]
#[case([1,2,3,4], vec![4,0,0,0], 1)]
#[case([1,2,3,4], vec![0,1,0,0], 1)]
#[case([1,2,3,4], vec![0,0,2,0], 1)]
#[case([1,2,3,4], vec![0,0,0,3], 1)]
#[case([1,2,3,4], vec![4,1,0,0], 2)]
#[case([1,2,3,4], vec![0,1,2,0], 2)]
#[case([1,2,3,4], vec![0,0,2,3], 2)]
#[case([1,2,3,4], vec![4,0,0,3], 2)]
#[case([1,2,3,4], vec![4,1,2,0], 3)]
#[case([1,2,3,4], vec![0,1,2,3], 3)]
#[case([1,2,3,4], vec![4,0,2,3], 3)]
#[case([1,2,3,4], vec![1,2,3,4], 0)]
fn test_check_combination_handles_misplaced(
    #[case] combination: [usize; 4],
    #[case] combination_try: Vec<usize>,
    #[case] expected: usize,
) {
    // arrange
    let game = Game::new_parameterized(true, 4, 6, 1, combination.to_vec());

    // act
    let result = game.check_combination(&combination_try);

    // assert
    assert_eq!(result.unwrap().misplaced(), expected);
}

#[rstest]
#[case([1,1,2,2], vec![2,0,0,0], 1)]
#[case([1,1,2,2], vec![0,2,0,0], 1)]
#[case([1,1,2,2], vec![2,2,0,0], 2)]
#[case([1,1,2,2], vec![0,0,1,0], 1)]
#[case([1,1,2,2], vec![0,0,0,1], 1)]
#[case([1,1,2,2], vec![0,0,1,1], 2)]
fn test_check_combination_handles_misplaced_duplicates(
    #[case] combination: [usize; 4],
    #[case] combination_try: Vec<usize>,
    #[case] expected: usize,
) {
    // arrange
    let game = Game::new_parameterized(true, 4, 4, 1, combination.to_vec());

    // act
    let result = game.check_combination(&combination_try);

    // assert
    assert_eq!(result.unwrap().misplaced(), expected);
}

#[rstest]
#[case([1,2,3,4], vec![4,4,0,0], 1)]
#[case([1,2,3,4], vec![0,1,1,0], 1)]
#[case([1,2,3,4], vec![0,0,2,2], 1)]
#[case([1,2,3,4], vec![3,0,0,3], 1)]
#[case([1,2,3,4], vec![4,1,4,1], 2)]
#[case([1,2,3,4], vec![2,1,2,1], 2)]
#[case([1,2,3,4], vec![2,3,2,3], 2)]
#[case([1,2,3,4], vec![4,3,4,3], 2)]
#[case([1,2,3,4], vec![4,1,2,1], 3)]
#[case([1,2,3,4], vec![3,1,2,3], 3)]
#[case([1,2,3,4], vec![4,3,2,3], 3)]
fn test_check_combination_handles_misplaced_proposed_duplicates(
    #[case] combination: [usize; 4],
    #[case] combination_try: Vec<usize>,
    #[case] expected: usize,
) {
    // arrange
    let game = Game::new_parameterized(true, 4, 6, 1, combination.to_vec());

    // act
    let result = game.check_combination(&combination_try);

    // assert
    assert_eq!(result.unwrap().misplaced(), expected);
}
