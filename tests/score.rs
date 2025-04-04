use duel_game::game::calculate_score;

#[test]
fn test_exact_hit() {
    let score = calculate_score(100, 0, 100, 10);
    assert_eq!(score, (100 + 10) / 1);
}

#[test]
fn test_near_hit_low_diff() {
    let score = calculate_score(99, 0, 100, 10); // diff = 1
    assert_eq!(score, (80 + 10) / 1);
}

#[test]
fn test_mid_diff() {
    let score = calculate_score(90, 0, 100, 10); // diff = 10
    assert_eq!(score, (60 + 10) / 1);
}

#[test]
fn test_high_diff() {
    let score = calculate_score(60, 0, 100, 10); // diff = 40
    assert_eq!(score, (20 + 10) / 1);
}

#[test]
fn test_very_high_diff() {
    let score = calculate_score(0, 0, 100, 10); // diff = 100
    assert_eq!(score, (100 + 10) / 1);
}

#[test]
fn test_with_misses() {
    let score = calculate_score(99, 2, 100, 10); // diff = 1, miss = 2
    assert_eq!(score, (80 + 10) / 3);
}

#[test]
fn test_with_negative_diff() {
    let score = calculate_score(120, 1, 100, 10); // diff = 20
    assert_eq!(score, (40 + 10) / 2);
}
