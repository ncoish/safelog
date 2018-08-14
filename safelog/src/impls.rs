use Loggable;

macro_rules! signed_loggable_impl {
    ($($t:ty)*) => ($(
        impl Loggable for $t {
            #[inline]
            fn get_loggable(&self) -> $t { Self::from(0_i8) }
        }
    )*)
}

macro_rules! unsigned_loggable_impl {
    ($($t:ty)*) => ($(
        impl Loggable for $t {
            #[inline]
            fn get_loggable(&self) -> $t { Self::from(0_u8) }
        }
    )*)
}

signed_loggable_impl! { f32 f64 isize i8 i16 i32 i64 i128 }
unsigned_loggable_impl! { usize u8 u16 u32 u64 u128 }

impl Loggable for String {
    fn get_loggable(&self) -> Self {
        String::from("REDACTED")
    }
}

impl<T: Loggable> Loggable for Option<T> {
    fn get_loggable(&self) -> Self {
        self.as_ref().and_then(|a| Some(a.get_loggable()))
    }
}

impl Loggable for char {
    fn get_loggable(&self) -> Self {
        'X'
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numeric_conversions() {
        let (
            n_f32,
            n_f64,
            n_isize,
            n_i8,
            n_i16,
            n_i32,
            n_i64,
            n_i128,
            n_usize,
            n_u8,
            n_u16,
            n_u32,
            n_u64,
            n_u128,
        ): (
            f32,
            f64,
            isize,
            i8,
            i16,
            i32,
            i64,
            i128,
            usize,
            u8,
            u16,
            u32,
            u64,
            u128,
        ) = (5.0, 5.0, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5);
        assert_eq!(n_f32.get_loggable(), 0.0);
        assert_eq!(n_f64.get_loggable(), 0.0);
        assert_eq!(n_isize.get_loggable(), 0);
        assert_eq!(n_i8.get_loggable(), 0);
        assert_eq!(n_i16.get_loggable(), 0);
        assert_eq!(n_i32.get_loggable(), 0);
        assert_eq!(n_i64.get_loggable(), 0);
        assert_eq!(n_i128.get_loggable(), 0);
        assert_eq!(n_usize.get_loggable(), 0);
        assert_eq!(n_u8.get_loggable(), 0);
        assert_eq!(n_u16.get_loggable(), 0);
        assert_eq!(n_u32.get_loggable(), 0);
        assert_eq!(n_u64.get_loggable(), 0);
        assert_eq!(n_u128.get_loggable(), 0);
    }

    #[test]
    fn string_converts_to_loggable() {
        let test_string = String::from("Hello! This is a string!");

        assert_eq!(
            test_string.get_loggable_string(),
            format!("{:?}", String::from("REDACTED"))
        );
    }

    #[test]
    fn option_string_converts_to_loggable() {
        let test_opt_some = Some(String::from("Hello! This is a string!"));
        assert_eq!(
            test_opt_some.get_loggable_string(),
            format!("{:?}", Some(String::from("REDACTED")))
        );

        let test_opt_none: Option<String> = None;
        assert_eq!(
            test_opt_none.get_loggable_string(),
            format!("{:?}", None::<String>)
        );
    }
}
