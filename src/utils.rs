pub trait IteratorExt: Iterator {}

#[macro_export]
macro_rules! next_tuple {
    ($iter:expr, $($t:ty),+) => {
        (
            $(
                $iter.next().unwrap().parse::<$t>().unwrap()
            ),+
        )
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn next_tuple_works() {
        let (_a, _b, _c) = next_tuple!(
            &mut ["42", "hello", "3.14"].into_iter(),
            String,
            String,
            String
        );
    }
}
