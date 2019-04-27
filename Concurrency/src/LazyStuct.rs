pub struct Lazy<T: Sync> (Cell <Option<T>> ,Once);

impl<T:Sync>Lazy<T>{
    pub const fn new ():Self{
        Lazy(Cell::new(None),Once::new())
    }


pub fn get<F> (&' static self, f: impl FnOnce()->T) ->& T{
    self.1.call_once(||{
        self.0.set(Some(f()));
    });
    unsafe {
        match *self.0.as_ptr(){
            Some(ref x) =>x,
            None=>  panic!()
        }
    }
}

}
unsafe impl<T:SYnc>Sync for Lazy<T> {}









}