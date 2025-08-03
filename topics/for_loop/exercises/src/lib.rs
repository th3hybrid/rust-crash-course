pub fn sum(nums: Vec<i32>) -> i32 {
    let mut x: i32 = 0;
    for num in nums {
        x+=num;
    }
    x
}

pub fn fill(i: u32, n: usize) -> Vec<u32> {
    let v = vec![i; n];
    v
}
