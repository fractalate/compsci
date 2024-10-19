pub fn selection_sort<T>(items: &mut [T])
where
    T: std::cmp::Ord {
  // Analysis                      Worse Case
  for i in 0..items.len()-1 {   // (n-1)
    let mut bj = i;             // (n-1)
    for j in i+1..items.len() { // (n-1) + (n-2) + ... + 1 = n*(n-1)/2
      if items[j] < items[bj] { // (n-1) + (n-2) + ... + 1 = n*(n-1)/2
        bj = j;                 // (n-1) + (n-2) + ... + 1 = n*(n-1)/2
      }
    }
    items.swap(i, bj);          // (n-1)
  }
  //                        T(n) = 3*(n-1) + 3*n*(n-1)/2
  //                             = 3*n - 3 + (3/2)*n^2 - (3/2)*n
  //                             = (3/2)*n^2 + (3/2)*n - 3
  //                             = Theta(n^2)
}

#[test]
fn test_selection_sort() {
  let mut v: Vec<i32> = Vec::from([7, 4, 0, 8, 2, 5, 9, 1, 3, 6]);
  selection_sort(&mut v[..]);
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
