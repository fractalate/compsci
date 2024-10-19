// Merge sorted slices items[p..q] and items[q..r] into items[p..r].
fn merge<T>(items: &mut [T], p: usize, q: usize, r: usize)
where
    T: std::cmp::Ord + Copy {
  // Analysis                                  Worst Case
  //
  // let L = q-p, R = r-q
  //
  //                                           Executions  Cost of Each
  let mut i = 0;                            // 1           1
  let mut j = 0;                            // 1           1
  let mut k = p;                            // 1           1
  let left: Vec<T> = items[p..q].to_vec();  // 1           L
  let right: Vec<T> = items[q..r].to_vec(); // 1           R
  while i < left.len() && j < right.len() { // min(L,R)    2
    if left[i] < right[j] {                 // min(L,R)    1
      items[k] = left[i];                   // min(L,R)    1 ?????
      i += 1;                               // min(L,R)    1
    } else {
      items[k] = right[j];                  // min(L,R)    1 ????? is there an interesting interplay between these cases?
      j += 1;                               // min(L,R)    1
    }
    k += 1;                                 // min(L,R)    1
  }
  while i < left.len() {                    // L           1
    items[k] = left[i];                     // L           1
    i += 1;                                 // L           1
    k += 1;                                 // L           1
  }
  while j < right.len() {                   // R           1
    items[k] = right[j];                    // R           1
    j += 1;                                 // R           1
    k += 1;                                 // R           1
  }
  assert!(k == r);
  //                                    T(n) = 3 + L + R + 7*min(L,R) + 4*L + 4*R
  //                                         = 3 + Theta(5*L) + Theta(5*R) + Theta(7*min(L, R))
  //                                         = Theta(5*L) + Theta(5*R) + Theta(7*L) + Theta(7*R)
  //                                         = Theta(12*L) + Theta(12*R)
  //                                         = Theta(L) + Theta(R)
  // Assume L \approx R, call it n           = Theta(n)
}

pub fn merge_sort<T>(items: &mut [T], p: usize, r: usize)
where
    T: std::cmp::Ord + Copy {
  // Analysis                   Worst Case
  //
  //                            Work Case                      Easy Case
  //                            Executions  Cost of Each       Executions  Cost of Each
  if r - p > 1 {             // 1           1                  1           1
    let q = (p + r) / 2;     // 1           3                  0
    merge_sort(items, p, q); // 1           T(n/2)             0
    merge_sort(items, q, r); // 1           T(n/2)             0
    merge(items, p, q, r);   // 1           Theta(n)           0
  }
  //                        T(n) = 1                        if n < 2
  //                             = 4 + 2*T(n/2) + Theta(n)  if n >= 2
  // 
  //                        T(n) = Theta(n*lg(n))?
  //
  // Assume                 T(n) = 4 + 2*T(n/2) + Theta(n)
  //                             = Theta(n*lg(n))                  for some n >= 2
  // Consider             T(2*n) = 4 + 2*T(2*n/2) + Theta(2*n)
  //                             = 4 + 2*T(n) + Theta(2*n)
  //                             = 4 + 2*Theta(n*lg(n)) + Theta(2*n)
  //                             = 4 + Theta(n*lg(n)) + Theta(n)
  //                             = Theta(n*lg(n))
  //
  // Therefore, T(n) = Theta(n*lg(n)).
  //
  // Note   Theta((2*n)*lg(2*n)) = Theta(n*lg(2*n))
  //                             = Theta(n*lg(2) + n*lg(n))
  //                             = Theta(n*lg(n))
}

#[test]
fn test_merge_sort() {
  let mut v: Vec<i32> = Vec::from([7, 4, 0, 8, 2, 5, 9, 1, 3, 6]);
  let l = v.len();
  merge_sort(&mut v[..], 0, l);
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
