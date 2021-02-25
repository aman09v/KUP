pub fn leap() {
    let array=[(12,2,2008),(16,2,2024)];
    let mut count = 0;
    let mut index=0;
    while index < array.len()

    {
        let t = array[index];
        let year = t.2;
        if ((year % 4 == 0) && (year % 100 != 0)) ||
            (year % 400 == 0)
        {
            count +=1;
        }
        index+=1;
    }
    println!("leap years are = {}",count);
}
