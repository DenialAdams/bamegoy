pub trait LoHi {
    type Output;
    
    fn lo(&self) -> Self::Output;
    fn hi(&self) -> Self::Output;
}

impl LoHi for u16 {
    type Output = u8;
    
    fn lo(&self) -> Self::Output { *self as u8 }
    fn hi(&self) -> Self::Output { (*self >> 8) as u8 }
}