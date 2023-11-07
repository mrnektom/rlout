
pub trait MaybeDerivedFrom<'a, T: ?Sized + 'a> {
    fn try_as(& self) -> Option<& T>;
    fn try_as_mut(&mut self) -> Option<&mut T>;
}



#[macro_export]
macro_rules! derive {
    ($name: ty, $super: ty) => {
        impl<'a> rlout_runtime::MaybeDerivedFrom<'a, $super> for $name {
            fn try_as(&self) -> Option<&$super> {
                Some(self)
            }
        
            fn try_as_mut(&mut self) -> Option<&mut $super> {
                Some(self)
            }
        }
    };

    ($name: ty, $super: ty, false) => {
        impl<'a> rlout_runtime::MaybeDerivedFrom<'a, $super> for $name {
            fn try_as(&self) -> Option<&$super> {
                None
            }
        
            fn try_as_mut(&mut self) -> Option<&mut $super> {
                None
            }
        }
    };
}
