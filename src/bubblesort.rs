pub fn ordenate<T: PartialOrd>(list: &mut Vec<T>) {
    ordenate_impl(list, list.len());
}

fn ordenate_impl<T: PartialOrd>(list: &mut Vec<T>, len: usize) {
    if len < 1 {
        return;
    }

    for i in 0..len {
        if i+1 != list.len() {
            if list[i] > list[i + 1] {
                list.swap(i, i+1)
            }
        }
    }
    ordenate_impl(list, len-1);
}