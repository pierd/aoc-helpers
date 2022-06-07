pub fn permutations<T, F>(elements: Vec<T>, callback: F) -> Vec<T>
where
    F: FnMut(&Vec<T>),
{
    gen_permutations(elements.len(), elements, callback).0
}

fn gen_permutations<T, F>(k: usize, mut elements: Vec<T>, mut callback: F) -> (Vec<T>, F)
where
    F: FnMut(&Vec<T>),
{
    if k == 1 {
        callback(&elements);
    } else {
        (elements, callback) = gen_permutations(k - 1, elements, callback);
        for i in 0..k - 1 {
            elements.swap(if k % 2 == 0 { i } else { 0 }, k - 1);
            (elements, callback) = gen_permutations(k - 1, elements, callback);
        }
    }
    (elements, callback)
}
