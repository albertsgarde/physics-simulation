pub fn all_pairs_mut<T>(slice: &mut [T], mut f: impl FnMut(&mut T, &mut T)) {
    let mut tail = slice;
    while let Some((item_a, new_tail)) = tail.split_first_mut() {
        tail = new_tail;
        for item_b in tail.iter_mut() {
            f(item_a, item_b);
        }
    }
}
