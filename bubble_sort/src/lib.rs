use std::fmt::Debug;

// worst case O(n^2)
pub fn bubble_sort<T:PartialOrd + Debug> (v: &mut [T]) {
    let real_len: usize = v.len() - 1;

    for e in 0..real_len + 1 {
        println!("{:?}", v);
        let mut sorted = true;
        let mut inv_i: usize;

        for i in 0..real_len {
            // Reverse direction each pass
            if e % 2 == 0 {
                if v[i] > v[i+1] {
                    v.swap(i, i+1);
                    sorted = false;
                }
            } else {
                inv_i = real_len - i;
                if v[inv_i] < v[inv_i - 1] {
                    v.swap(inv_i, inv_i - 1);
                    sorted = false;
                }
            }
        }
        if sorted {
            return;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bubble_sort() {
        let mut v = vec![9, 3, 5, 1];
        bubble_sort(&mut v);
        assert_eq!(v, vec![1, 3, 5, 9]);
    }

}
