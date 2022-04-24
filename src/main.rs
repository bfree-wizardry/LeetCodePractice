fn main() {

    println!("{:#?}", get_concatenation(vec![1, 2, 3, 4]));
} //nums out of scope


//Concatenation of Array
pub fn get_concatenation(mut nums: Vec<i32>) -> Vec<i32> {
        let lengths = nums.len();
        for item in 0..lengths {
            nums.push(nums[item]);
        }
        return nums;
    }

