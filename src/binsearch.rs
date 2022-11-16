pub fn search<T: PartialEq + PartialOrd>(list: &Vec<T>, target: T) -> Option<usize> {
    let (mut low, mut high) = (0, list.len());
    let mut middle;

    while low != high {
        middle = (high + low) / 2;

        if target == list[middle] {
            return Some(middle);
        }
        else if target > list[middle] {
            low = middle + 1;
        }
        else {
            high = middle - 1;
        }
    }

    return None;
}