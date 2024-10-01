pub struct PriorityHeap<T: std::cmp::Ord> {
  vec: Vec<T>,
}

impl<T: std::cmp::Ord> PriorityHeap<T> {
  pub fn new() -> PriorityHeap<T> {
    PriorityHeap{
      vec: Vec::new(),
    }
  }

  pub fn is_empty(&self) -> bool {
    self.vec.is_empty()
  }

  pub fn push(&mut self, value: T) {
    self.vec.push(value);

    let mut index = self.vec.len();
    while index >= 2 {
      let parent_index = index / 2;

      if self.vec[parent_index-1] > self.vec[index-1] {
        self.vec.swap(parent_index-1, index-1);
      } else {
        break;
      }

      index = parent_index;
    }
  }

  pub fn len(&self) -> usize {
    self.vec.len()
  }

  pub fn pop(&mut self) -> Option<T> {
    if self.vec.is_empty() {
      return None;
    }

    let tail = self.len()-1;
    if tail > 0 {
      self.vec.swap(0, tail);
    }

    let item = self.vec.pop().unwrap();

    let mut index = 1;
    loop {
      let lchild = 2 * index;
      let rchild = lchild + 1;

      if rchild-1 < self.vec.len() && self.vec[rchild-1] < self.vec[lchild-1] && self.vec[rchild-1] < self.vec[index-1] {
        self.vec.swap(index-1,rchild-1);
        index = rchild;
      } else if lchild-1 < self.vec.len() && self.vec[lchild-1] < self.vec[index-1] {
        self.vec.swap(index-1,lchild-1);
        index = lchild;
      } else {
        break;
      }
    }

    Some(item)
  }
}

#[test]
fn test_priority_heap() {
  let mut heap: PriorityHeap<i32> = PriorityHeap::new();

  heap.push(4);
  heap.push(3);
  heap.push(1);
  heap.push(0);
  heap.push(2);

  assert!(heap.pop().unwrap() == 0);
  assert!(heap.pop().unwrap() == 1);
  assert!(heap.pop().unwrap() == 2);
  assert!(heap.pop().unwrap() == 3);
  assert!(heap.pop().unwrap() == 4);

  for i in 0..100 {
    heap.push(i);
  }

  for i in 0..100 {
    assert!(heap.pop().unwrap() == i);
  }
}
