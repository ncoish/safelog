use std::fmt::Debug;

pub trait Loggable: Debug + Sized {
    fn get_loggable(&self) -> Self;

    fn get_loggable_string(&self) -> String {
        let loggable = self.get_loggable();
        format!("{:?}", loggable)
    }
}

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
