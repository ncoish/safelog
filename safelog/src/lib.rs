use std::fmt::Debug;

mod impls;

pub trait Loggable: Sized + Debug {
    fn get_loggable(&self) -> Self;

    fn get_loggable_string(&self) -> String {
        let loggable = self.get_loggable();
        format!("{:?}", loggable)
    }
}
