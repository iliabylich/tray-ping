pub(crate) struct FixedSizeQueue<const N: usize, T>
where
    T: Clone,
{
    queue: [Option<T>; N],
}

impl<const N: usize, T> FixedSizeQueue<N, T>
where
    T: Clone,
{
    pub(crate) fn new() -> Self {
        Self {
            queue: std::array::from_fn(|_| None),
        }
    }

    pub(crate) fn push(&mut self, item: T) {
        for i in 0..N - 1 {
            self.queue[i] = self.queue[i + 1].take();
        }
        self.queue[N - 1] = Some(item);
    }

    pub(crate) fn get(&self) -> [Option<T>; N] {
        self.queue.clone()
    }
}
