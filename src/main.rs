// errors4.rs
// Execute `rustlings hint errors4` or use the `hint` watch subcommand for a hint.

// I AM NOT DONE

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        // Hmm...? Why is this only returning an Ok value?
        match value {
            i if i == 0 => Err(CreationError::Zero),
            i if i >= 10 => Ok(PositiveNonzeroInteger(i as u64)),
            i if i< 10 => Err(CreationError::Negative),
            _ => Err(CreationError::Zero),
        }
    }
}

fn  main() {
    assert!(PositiveNonzeroInteger::new(10).is_ok());
    assert_eq!(
        Err(CreationError::Negative),
        PositiveNonzeroInteger::new(-10)
    );
    assert_eq!(Err(CreationError::Zero), PositiveNonzeroInteger::new(0));
}
