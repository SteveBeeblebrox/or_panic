mod sealed {
    pub trait SealedTrait {}
}

impl<T,E: std::fmt::Display> sealed::SealedTrait for Result<T,E> {}

pub trait OrPanic<T> : sealed::SealedTrait {
    fn or_panic(self) -> T;
}

impl<T,E: std::fmt::Display> OrPanic<T> for Result<T,E> {
    fn or_panic(self) -> T {
        match self {
            Ok(t) => t,
            Err(e) => panic!("{e}")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn does_panic() {
        let result: Result<i32,&'static str> = Err("Ahhhh!");
        let _ = result.or_panic();
    }

    #[test]
    fn does_not_panic() {
        let result: Result<i32,&'static str> = Ok(1);
        assert_eq!(result.or_panic(), 1);
    }
}
