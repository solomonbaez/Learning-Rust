fn main() {
    let x = Vec::<u32>::from([1, 2, 4]);
    let is_even = my_filter(&x, &|x| x % 2 == 0);
    let is_odd = my_filter(&x, &|x| x % 2 != 0);
    println!("{}, {}", is_even[0], is_odd[0]);

    let times_two = my_map(&x, &|x| x * 2);
    let square = my_map(&x, &|x| x * x);
    println!("{}, {}", times_two[1], square[1]);
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

fn my_map<T: Copy>(input: &Vec<T>, map_fn: &dyn Fn(T) -> T) -> Vec<T> {
    let mut mapped_vec = Vec::<T>::new();

    input.into_iter().for_each(|v| {
        mapped_vec.push(map_fn(*v));
    });

    mapped_vec
}
