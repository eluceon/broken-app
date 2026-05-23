use broken_app::{algo, concurrency, leak_buffer, normalize, sum_even};

#[test]
fn sums_even_numbers() {
    let nums = [1, 2, 3, 4];
    assert_eq!(sum_even(&nums), 6);
}

#[test]
fn sum_even_empty() {
    assert_eq!(sum_even(&[]), 0);
}

#[test]
fn sum_even_all_odd() {
    assert_eq!(sum_even(&[1, 3, 5, 7]), 0);
}

#[test]
fn sum_even_boundary() {
    assert_eq!(sum_even(&[0, -2, 2, -1]), 0);
}

#[test]
fn counts_non_zero_bytes() {
    let data = [0_u8, 1, 0, 2, 3];
    assert_eq!(leak_buffer(&data), 3);
}

#[test]
fn leak_buffer_empty() {
    assert_eq!(leak_buffer(&[]), 0);
}

#[test]
fn leak_buffer_all_zeros() {
    assert_eq!(leak_buffer(&[0, 0, 0]), 0);
}

#[test]
fn dedup_preserves_uniques() {
    let uniq = algo::slow_dedup(&[5, 5, 1, 2, 2, 3]);
    assert_eq!(uniq, vec![1, 2, 3, 5]); // порядок и состав важны
}

#[test]
fn fib_small_numbers() {
    assert_eq!(algo::slow_fib(10), 55);
}

#[test]
fn normalize_simple() {
    assert_eq!(normalize(" Hello World "), "helloworld");
}

#[test]
fn normalize_tabs_and_newlines() {
    assert_eq!(normalize("\tHello\nWorld\t"), "helloworld");
}

#[test]
fn normalize_multiple_spaces_inside() {
    assert_eq!(normalize("Hello    World"), "helloworld");
}

#[test]
fn normalize_unicode_whitespace() {
    assert_eq!(normalize("  Hello   "), "hello");
}

#[test]
fn averages_only_positive() {
    let nums = [-5, 5, 15];
    assert!((broken_app::average_positive(&nums) - 10.0).abs() < f64::EPSILON);
}

#[test]
fn average_positive_empty() {
    assert_eq!(broken_app::average_positive(&[]), 0.0);
}

#[test]
fn average_positive_all_negative() {
    assert_eq!(broken_app::average_positive(&[-1, -2, -3]), 0.0);
}

#[test]
fn average_positive_single() {
    assert!((broken_app::average_positive(&[42]) - 42.0).abs() < f64::EPSILON);
}

#[test]
fn race_increment_is_correct() {
    let total = concurrency::race_increment(1_000, 4);
    assert_eq!(total, 4_000);
}
