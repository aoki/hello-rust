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

    fn glow(&mut self) {
        if self.capacity() == 0 {
            self.elements = Self::allocate_in_heap(1);
        } else {
            let new_elements = Self::allocate_in_heap(self.capacity() * 2);
            let old_elements = std::mem::replace(&mut self.elements, new_elements);
            // https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.enumerate
            for (i, elem) in old_elements.into_vec().into_iter().enumerate() {
                self.elements[i] = elem;
            }
        }
    }

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
            // 上記だと借用しているため所有権が奪えないた、対象の要素だけDefaultと置き換える
            let elem = std::mem::replace(&mut self.elements[self.len], Default::default());
            Some(elem)
        }
    }

    // &self と Iter<T> のライフタイムを同一にしているので、ライフタイムパラメーターを省略できる
    pub fn iter(&self) -> Iter<T> {
        Iter {
            elements: &self.elements,
            len: self.len,
            pos: 0,
        }
    }
}

// 以下のような実装があった場合、コンパイラは elements のライフタイムが推論できないので、指定してあげる必要がある
// pub struct Iter<T> {
//     elements: &Box<T>,
//     len: usize,
//     pos: usize,
// }
pub struct Iter<'a, T> {
    elements: &'a Box<[T]>,
    len: usize,
    pos: usize,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.len {
            None
        } else {
            let res = Some(&self.elements[self.pos]);
            self.pos += 1;
            res
        }
    }
}

impl<'a, T: Default> IntoIterator for &'a ToyVec<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
