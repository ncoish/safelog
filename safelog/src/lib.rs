use std::fmt::Debug;

mod impls;

// TODO: impl specialization would make this amazing
// https://github.com/rust-lang/rfcs/blob/master/text/1210-impl-specialization.md
pub trait Loggable: Sized + Debug {
    fn get_loggable(&self) -> Self;

    fn get_loggable_string(&self) -> String {
        let loggable = self.get_loggable();
        format!("{:?}", loggable)
    }
}
