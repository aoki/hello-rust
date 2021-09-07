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

    pub fn new_with_capacity(size: usize) -> Self {
        Self::with_capacity(size)
    }

    pub fn len(self: &Self) -> usize {
        self.len
    }

    // `&self` は `self: &Self` の糖衣構文
    pub fn capacity(&self) -> usize {
        self.elements.len()
    }

    // `&mut self` を引数に取るので、構造体の内容を変更する
    // `element: T` を引数に取り、`&` がついていないので所有権がムーブする
    pub fn push(&mut self, element: T) {
        if self.len == self.capacity() {
            self.glow();
        }
        self.elements[self.len] = element; // さらに構造体の `elements` に所有権がムーブする
        self.len += 1;
    }

    fn glow(&mut self) {}

    // `&self` を引数に取るので、構造体の内容は変更されない
    // `Option<&T>` を返すため、`self` が所有する値の不変の参照を返す
    pub fn get(&self, index: usize) -> Option<&T> {
        if index < self.len {
            Some(&self.elements[index])
        } else {
            None
        }
    }

    // pub fn get_or(& self, index: usize, default: &T) -> &T {
    //     match self.get(index) {
    //         Some(v) => v, // Someの場合は v が呼び出し元の構造体の参照であるためライフタイムは明確
    //         None => default, // default は何の参照か明確でないため、ライフタイムも不明確
    //     }
    //     // 結果として、match が返す値のライフタイムがミスマッチであるというエラーになる
    // }
    pub fn get_or<'a, 'b>(&'a self, index: usize, default: &'b T) -> &'a T
    where
        'b: 'a,
    {
        match self.get(index) {
            Some(v) => v,
            None => default,
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            // let elem = self.elements[self.len];
            let elem = std::mem::replace(&mut self.elements[self.len], Default::default());
            Some(elem)
        }
    }
}
