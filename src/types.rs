


pub trait SingleValue {
    type Value;


    fn get(&self) -> Self::Value;
    fn set(&mut self, v: Self::Value);
    fn new(v: Self::Value) -> Self;
}

#[macro_export]
macro_rules! impl_single_value {
    ($ty:ty, $val_ty:ty) => {
        impl SingleValue for $ty {
            type Value = $val_ty;

            fn get(&self) -> Self::Value {
                self.0
            }

            fn set(&mut self, v: Self::Value) {
                self.0 = v;
            } 

            fn new(v: Self::Value) -> Self {
                Self(v)
            }
        }
        
    };
}
