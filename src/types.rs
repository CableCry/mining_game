

pub trait ValueAccess {
    fn get_value(&self) -> f64;
    fn set_value(&mut self, v: f64);
}


pub trait NewForSingleValue<T> {
    fn new(v: T) -> Self;
}

