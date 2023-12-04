fn main() {
    let x = Vec::<u32>::from([1, 2, 4]);
    let is_even = my_filter(&x, &|x| x % 2 == 0);
    let is_odd = my_filter(&x, &|x| x % 2 != 0);
    println!("{}, {}", is_even[0], is_odd[0])
}

fn my_filter<T: Copy>(input: &Vec<T>, filter_fn: &dyn Fn(T) -> bool) -> Vec<T> {
    let mut filtered_vec = Vec::<T>::new();
    input.into_iter().for_each(|v| {
        if filter_fn(*v) {
            filtered_vec.push(*v)
        }
    });

    filtered_vec
}
