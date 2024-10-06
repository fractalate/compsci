pub fn bubble_sort<T>(items: &mut [T])
where 
    T: std::cmp::Ord {
  // Analysis                                      Worst Case

  for i in 0..items.len()-1 {           //  (n-1)
    for j in (i+1..items.len()).rev() { //  (n-2)*(n-1)/2
      if items[j] < items[j-1] {               //  (n-2)*(n-1)/2
        items.swap(j-1,j);                //  (n-2)*(n-1)/2
      }
    }
  }
  //                                        T(n) = (n-1) + 3*(n-2)*(n-1)/2
  //                                             = (3/2)*n^2 - (7/2)*n -2
  //                                             = Theta(n^2)
}

#[test]
fn test_bubble_sort() {
  let mut v: Vec<i32> = Vec::from([7, 4, 0, 8, 2, 5, 9, 1, 3, 6]);
  bubble_sort(&mut v[..]);
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
