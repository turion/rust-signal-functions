use signal_functions::*;
use signal_functions::accum::*;
use signal_functions::fun::*;

#[test]
fn test() {
    let sum = Accum::new(0, |acc, input| *acc += input);
    let squares = Fun::new(|n| n * n);
    let doubles = Fun::new(|n| n * 2);
    let result: Vec<_> = sum
        .and_then(squares)
        .fan_out(doubles)
        .apply_to(std::iter::repeat(()).zip(vec![1,1,1].iter()))
        .collect();
    assert_eq!(result, vec![
        (0, 2),
        (1, 2),
        (4, 2),
    ]);
}
