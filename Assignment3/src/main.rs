use crate::mergesort::{merge,merge_sort};
use crate::search::{binary_search, linear_search};
use crate::leap::{leap};

mod leap;
mod search;
mod mergesort;

fn main() {
   let mut arr: [i32; 5] = [10,20,30,40,50];
   let mut array_to_sort: [i32;5] = [80,10,70,20,30];
   let mut element=20;
   let mut start=0;
   let mut end=arr.len()-1;
   binary_search(&arr, element, start, end as i32);
   linear_search(&arr, element, 0);
   leap();
   merge_sort(*&mut array_to_sort, 0, 4);
   for i in 0..5{
      print!(" {}",array_to_sort[i]);
   }

}

