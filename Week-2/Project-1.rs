fn main(){
       let p = 520_000_000.0;
       let r = 10.0;
       let n = 5.0;

       //compound interest
       let a = p*(1.0+(r/100.0))*n;
       println!("The amount is {}",a);
       let ci = a-p;
       println!("The compound interest is {}",ci)
      }