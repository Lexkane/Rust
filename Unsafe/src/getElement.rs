pub fn get<I :SliceIndex<Self>>(&self,index:I)
-<Oprion<&ILOutput>
{
    index.get(self)
}
pub fn get_mut<I:SliceIndex<Self>>(&mut self, index:I)
    -> Option <&mut I::Output>
    {
        index.get_mut(self)
    }

 impl <T> SliceIndex<[T] for usize{
     type Output=T;
     fn get( self,slice:&[T])->Option<&T>{
         if self<slice.len(){
             unsafe{Some(self.get_unchecked(slice))}
         } else{
             None
         }

        unsafe fn get_unchecked(self,slice:&[T]->&T){
            &+slice.as_ptr().add(self)
        } 

        impl <T:?Sized>*const T{
            pub unsafe fn add(self,count:usize)->Self where T:Sized
        }
     }
 
 impl <T> SliceIndex<[T]> for ops::Range<usize>{
     type Output=[T];
    fn get(self,slice:&[T])->Option <&[T]>{
        if self.start >self.end || self.end>slice.len(){
            None
        } else{
            unsafe{Some{self.get_unchecked(slice)}}
        }
    }
    unsafe fn get_unchecked(self, slice:&[T])->&[T]{
        from_raw_parts{
            slice.as_ptr().add(self.start),
            self.end-self.start,
        }
    }
 }