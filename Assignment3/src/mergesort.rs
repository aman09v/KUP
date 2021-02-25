pub fn merge_sort(mut arr:[i32; 5], l:usize, r:usize) {
    let mut m;
    if l < r {
        m = (l + (r - 1)) / 2;
        merge_sort(arr, l, m);
        merge_sort(arr, m + 1, r);
        merge(*&mut arr, l, m, r);
    }
}

pub fn merge(mut arr:[i32;5], l:usize, m:usize, r:usize) {
    let mut n1;
    let mut n2;
    let mut i;
    let mut j;
    let mut k:usize;
    let mut L:[i32;5] = [0,0,0,0,0];
    let mut R:[i32;5] = [0,0,0,0,0];
    n1 = m - l + 1;
    n2 = r - m;

    for ii in 0..n1 {
        i = ii as usize;
        L[i] = arr[l + i]
    }
    for jj in 0..n2 {
        j = jj as usize;
        R[j] = arr[m + 1 + j]
    }
    i = 0;
    j = 0;
    k = l;

    while i < n1 as usize && j < n2 as usize {
        if L[i] <= R[j] {
            arr[k] = L[i];
            i += 1;
        }
        else {
            arr[k] = R[j];
            j += 1;
            k += 1;
        }
    }

    while i < n1 as usize {
        arr[k] = L[i];
        i += 1;
        k += 1;
    }

    while j < n2 as usize {
        arr[k] = R[j];
        j += 1;
        k += 1;
    }
}