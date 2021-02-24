pub fn linear_search(arr:&[i32;5] ,x: i32,index: i32)
{

    if arr[index as usize] == x
    {
        println!("{}" , index);
        return;
    }
    if index as usize > arr.len()
    {
        print!("not present");
        return;
    }
    linear_search(arr,  x , index+1);
}


pub fn binary_search(arr: &[i32;5], x: i32, l: i32, m: i32)
{
    if l > m{
        return;
    }

    let mid = (l+m)/2;

    if arr[mid as usize] == x
    {
        println!("present at : {}",mid);
        return ;
    }
    else if arr[mid as usize] < x{
        binary_search(arr, x, mid+1, m);
        return ;
    }
    else
    {
        binary_search(arr, x, l, mid-1);
    }
}