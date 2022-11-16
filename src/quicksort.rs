pub fn ordenate<T: PartialEq + PartialOrd + Copy>(list: &mut [T]) {
    if list.len() > 1 {
        let part_index = partition(list);
        let len = list.len();

        ordenate(&mut list[0..part_index]);
        ordenate(&mut list[part_index + 1..len]);
    }
}

fn partition<T: PartialEq + PartialOrd + Copy>(list: &mut [T]) -> usize {
    let len = list.len() - 1;
    let pivot = list[len];
    let mut i = 0;
    let mut j = 0;

    while j < len {
        if list[j] <= pivot {
            list.swap(i, j);
            i += 1;
        }
        j += 1;
    }
    
    list.swap(i, len);
    return i;
}