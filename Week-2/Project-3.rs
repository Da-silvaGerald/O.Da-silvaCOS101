fn main(){

     let p:u64=210_000;
     let r:u64=3;
     let n:u64=5;

     //compound interest depreciation
     let a=p*(1+(r/100))^n;
     println!("The value is {}",a)

}