 fn main(){
   for b in 4..1000{
     let mut x = 1;
     for i in 2..b{
       if b%i==0{
         x=0;
         }
    }
    if x == 1{
      println!("{}",b);
    }
   }
 }
