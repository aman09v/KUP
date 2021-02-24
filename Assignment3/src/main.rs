
use crate::search::{binary_search, linear_search};
use crate::leap::{leap};

mod leap;
mod search;

fn main() {
   let mut arr: [i32; 5] = [10,20,30,40,50];

   let mut element=20;
   let mut start=0;
   let mut end=arr.len()-1;
   binary_search(&arr, element, start, end as i32);
   linear_search(&arr, element, 0);
   leap();


}

