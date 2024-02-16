use std::collections::VecDeque;

pub(crate) struct FixedSizeDeque<const N: usize, T>
where
    T: Clone,
{
    deque: VecDeque<Option<T>>,
}

impl<const N: usize, T> FixedSizeDeque<N, T>
where
    T: Clone,
{
    pub(crate) fn new() -> Self {
        Self {
            deque: VecDeque::from_iter(std::iter::repeat_with(|| None).take(N)),
        }
    }

    pub(crate) fn push(&mut self, item: T) {
        self.deque.pop_front();
        self.deque.push_back(Some(item));
    }

    pub(crate) fn get(&self) -> [Option<T>; N] {
        let mut result: [Option<T>; N] = std::array::from_fn(|_| None);
        for (i, item) in self.deque.iter().enumerate() {
            result[i] = item.clone();
        }
        result
    }
}
