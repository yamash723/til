use std::collections::HashMap;

fn main() {
    let nums = vec![0, 1, 2, 3, 4, 5, 6, 0, 2, 3, 4, 5, 5];
    
    let avg = calc_avg(&nums);
    let median = calc_median(&nums);
    let mode = calc_mode(&nums);

    println!("Avg: {} / Median: {} / Mode: {}", avg, median, mode);
}

fn calc_avg(nums: &Vec<u32>) -> u32 {
    let sum: u32 = nums.iter().sum();
    sum / nums.len() as u32
}

fn calc_median(nums: &Vec<u32>) -> u32 {
    let mut nums = nums.clone();
    nums.sort();

    let center_index = nums.len() / 2;
    nums[center_index]
}

fn calc_mode(nums: &Vec<u32>) -> u32 {
    let mut map = HashMap::new();
    for num in nums {
        let count = map.entry(num).or_insert(0);
        *count += 1;
    }

    let mut map_array: Vec<(&u32, isize)> = map.into_iter().collect();
    map_array.sort_by_key(|m| m.1);
    map_array.reverse();
    println!("{:?}", map_array);

    *map_array[0].0 as u32
}