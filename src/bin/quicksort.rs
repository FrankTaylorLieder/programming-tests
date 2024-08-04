use rand::prelude::*;

fn main() {
    let mut rng = rand::thread_rng();
    let mut nums: Vec<i32> = (0..100).collect();
    nums.shuffle(&mut rng);

    quicksort(&mut nums);

    println!("Result: {:?}", nums);
}

fn swap(nums: &mut [i32], a: usize, b: usize) {
    println!("Swap: {} {}", a, b);
    let x = nums[a];
    nums[a] = nums[b];
    nums[b] = x;
}

fn quicksort(nums: &mut [i32]) {
    println!("Quicksort: {:?}", nums);
    if nums.len() < 2 {
        return;
    }
    let l = nums.len();
    let pivot = nums[l - 1];

    let mut s = 0;
    for i in 0..(l - 1) {
        println!("Partial: {} {:?}", i, nums);
        if nums[i] <= pivot {
            swap(nums, i, s);
            s += 1;
        }
    }

    swap(nums, s, l - 1);
    println!("Mid: {} - {:?}", s, nums);

    if s > 0 {
        quicksort(&mut nums[0..s]);
    }
    if s < l {
        quicksort(&mut nums[s + 1..]);
    }
}
