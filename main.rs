#![allow(non_snake_case)]
fn main() {
  for x in 4..10000{ 
    let mut is_Prime = 1;
    let mut j = x-1;
    while j>1{
      if x%j==0{
        is_Prime =0;
        break;
      }
      if x!=0{
        j-=1;
      }
    }
    if is_Prime == 1{
        println!("{}",x);
    }
}

}
