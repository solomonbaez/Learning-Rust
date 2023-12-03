fn main() {
    let x = Vec::<u32>::from([1, 2, 4]);
    let filtered_x = my_filter(x, &filter_is_even);
    println!("{}, {}", filtered_x[0], filtered_x[1])
}

fn my_filter<T: Copy>(input: Vec<T>, filter_fn: &dyn Fn(T) -> bool) -> Vec<T> {
    let mut filtered_vec = Vec::<T>::new();
    for v in input {
        if filter_fn(v) {
            filtered_vec.push(v);
        }
    }

    filtered_vec
}

fn filter_is_even(x: u32) -> bool {
    (x % 2) == 0
}
