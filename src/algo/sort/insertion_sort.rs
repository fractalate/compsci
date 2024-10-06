// TODO - Can I do this without the Copy trait?
pub fn insertion_sort<T>(items: &mut [T])
where
    T: std::cmp::Ord + Copy {
  // Analysis                              Worst Case
  for j in 1..items.len() {      // (n-1)
    let key = items[j];              // (n-1)
    let mut i = j;               // (n-1)
    while i > 0 && items[i - 1] > key { // 1 + 2 + ... + (n-1) = n*(n-1)/2
      items[i] = items[i - 1];          // 1 + 2 + ... + (n-1) = n*(n-1)/2
      i -= 1;                           // 1 + 2 + ... + (n-1) = n*(n-1)/2
    }
    items[i] = key;                     // (n-1)
  }
  //                                T(n) = 4*(n-1) + 3*n*(n-1)/2
  //                                     = 4*n - 4 + (3/2)*(n^2 - n)
  //                                     = (3/2)n^2 + (5/2)*n - 4
  //                                     = Theta(n^2)
}

#[test]
fn test_insertion_sort() {
  let mut v: Vec<i32> = Vec::from([7, 4, 0, 8, 2, 5, 9, 1, 3, 6]);
  insertion_sort(&mut v[..]);
  assert!(v[0] == 0);
  assert!(v[1] == 1);
  assert!(v[2] == 2);
  assert!(v[3] == 3);
  assert!(v[4] == 4);
  assert!(v[5] == 5);
  assert!(v[6] == 6);
  assert!(v[7] == 7);
  assert!(v[8] == 8);
  assert!(v[9] == 9);
}
