use std::mem;

pub fn partition<T, F: Fn(&T) -> bool>(slice: &mut [T], comparator: F) -> usize {
    let mut i = 0;
    let mut j = slice.len() - 1;

    loop {
        while i < j && comparator(&slice[i]) {
            i += 1;
        }

        while i < j && !comparator(&slice[j]) {
            j -= 1;
        }

        if i >= j {
            return i;
        }

        let (left_slice, right_slice) = slice.split_at_mut(j);
        mem::swap(&mut left_slice[i], &mut right_slice[0]);
    }
}
