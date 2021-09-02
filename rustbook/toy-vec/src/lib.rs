#[derive(Debug)]
pub struct ToyVec<T> {
    elements: Box<[T]>,
    len: usize,
}

// トレイト境界 Default トレイトを実装したT型のみ扱える
impl<T: Default> ToyVec<T> {
    fn allocate_in_heap(size: usize) -> Box<[T]> {
        std::iter::repeat_with(Default::default)
            .take(size)
            .collect() // 本当は collect::<Vec<_>>() だけど実験
    }

    // 指定された capacity を持つ ToyVec を作る
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            elements: Self::allocate_in_heap(capacity),
            len: 0,
        }
    }

    // キャパシティが0のToyVecを作成する
    pub fn new() -> Self {
        Self::with_capacity(0)
    }

    pub fn len(self: &Self) -> usize {
        self.len
    }

    // `&self` は `self: &Self` の糖衣構文
    pub fn capacity(&self) -> usize {
        self.elements.len()
    }
}
