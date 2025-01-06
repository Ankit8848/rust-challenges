// Topic: working with a function

// Program requirements:
// Calculate the median of a list of numbers (Vec<f32>).

// Notes:
// - Use a function to calculate the median.
// - Handle three cases:
//   1. Even number of elements
//   2. Odd number of elements
//   3. Empty list
// - Sort the numbers before calculation.

fn median(mut med_num: Vec<f32>) -> Option<f32> {
    if med_num.is_empty() {
        return None;
    }

    // Sort the vector
    med_num.sort_by(|x, y| x.partial_cmp(y).unwrap());

    let n_elements = med_num.len();
    let middle = n_elements / 2;

    // Calculate the median
    let med = if n_elements % 2 == 0 {
        // Even case: average of the two middle elements
        (med_num[middle - 1] + med_num[middle]) / 2.0
    } else {
        // Odd case: middle element
        med_num[middle]
    };

    Some(med)
}

fn main() {
    // Test case with numbers
    let answer = median(vec![1.0, 2.0, 4.0]);
    println!("median([1.0, 2.0, 4.0]) = {:?}", answer);

    // Test case with an empty vector
    let empty_answer = median(Vec::new());
    println!("median([]) = {:?}", empty_answer);

    // Test case with an even number of elements
    let even_answer = median(vec![1.0, 2.0, 3.0, 4.0]);
    println!("median([1.0, 2.0, 3.0, 4.0]) = {:?}", even_answer);
}
