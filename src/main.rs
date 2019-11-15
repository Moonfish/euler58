/*

1 3 5 7 9 13 17 21 25 31 37 43 49
 2 2 2 2 4  4  4  4  6  6  6  6

*/

extern crate euler_lib;

fn main() {

    let mut i:usize = 0;
    let mut s:usize = 0;
    let mut k:usize = 1;
    let mut prime_count:usize = 0;
  
    loop
    {
      if i % 4 == 0{
         s += 2;
      }
          
      i += 1;

      if euler_lib::utility::is_prime(k){
        prime_count += 1;
      }
    
      k = k + s;

      if k > 100000{
        break;
      }

      let r:f64 = prime_count as f64 / k as f64;
      println!("{}", r);

      if r < 0.1{
        println!("Answer: {}", s);
        break;
      }

    }

}
