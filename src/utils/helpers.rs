pub fn transpose<T>(v: &[Vec<T>]) -> Vec<Vec<T>>
    where
        T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}


pub fn get_char(string: &str, idx: usize) -> char {
    string.chars().nth(idx).unwrap()
}
