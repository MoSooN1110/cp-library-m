// source snippet: key=lib_next_permutation  prefix=lib_next_permutation

fn next_permutation(nums: &mut Vec<usize>) {
    let mut i = nums.len() - 1;
    while i > 0 {
        i -= 1;
        if nums[i] < nums[i + 1] {
            let mut j = nums.len() - 1;
            while nums[i] >= nums[j] {
                j -= 1;
            }
            nums.swap(i, j);

            let mut low = i + 1;
            let mut high = nums.len() - 1;

            while low < high {
                nums.swap(low, high);
                low += 1;
                high -= 1;
            }
            return;
        }
    }
    nums.sort()
}

fn permutation_gen(n: usize) -> Vec<Vec<usize>> {
    let mut v: Vec<Vec<usize>> = Vec::new();
    let mut nums: Vec<usize> = (0..n).collect();
    v.push(nums.clone());
    while nums.len() > 0 {
        next_permutation(&mut nums);
        if v[0] == nums {
            break;
        }
        v.push(nums.clone());
    }
    v
}
