fn main(){
      let qt= 2.0;
      let t=450_000.0;
      println!("This is the quantity for {} {} And this is the price: {}","TOSHIBA:",qt,t);

      let qm=1.0;
      let m=1_500_000.0;
      println!("This is the quantity for {} {} And this is the price: {}","MAC:",qm,m);

      let qh=3.0;
      let h=750_000.0;
      println!("This is the quantity for {} {} And this is the price: {}","HP:",qh,h);

      let qd=3.0;
      let d=2_850_000.0;
      println!("This is the quantity for {} {} And this is the price: {}","DELL:",qd,d);

      let qa=1.0;
      let a=250_000.0;
      println!("This is the quantity for {} {} And this is the price: {}","ACER:",qa,a);

      //total amount
      let ta=((qt*t)+(qm*m)+(qh*h)+(qd*d)+(qa*a));
      println!("The total Amount is {}",ta);

      //Average
      let avg= (ta/(t+m+h+d+a));
      println!("The average is {}",avg)
}      


