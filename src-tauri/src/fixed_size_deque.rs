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

    pub(crate) fn push(&mut self, item: Option<T>) {
        self.deque.pop_front();
        self.deque.push_back(item);
    }

    pub(crate) fn get(&self) -> &VecDeque<Option<T>> {
        &self.deque
    }
}
