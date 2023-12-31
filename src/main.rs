use std::cmp::Ordering;

fn main() {
    let ordered_elements: Vec<i32> = vec![1, 3, 5, 7, 9, 11, 13, 15]; // base 2 log of 8
    match binary_search_element(&ordered_elements, 0) {
        Some(index) => println!("Element found at index: {}", index),
        None => eprintln!("Element not found"),
    }
}

fn binary_search_element(ordered_elements: &Vec<i32>, element_to_find: i32) -> Option<usize> {
    let mut lower_index: usize = 0;
    let mut higher_index: usize = ordered_elements.len() - 1;
    let mut num_of_iterations: i32 = 0;

    while lower_index <= higher_index {
        num_of_iterations += 1;
        println!("Iteration number: {}", num_of_iterations);
        let mid_index = (lower_index + higher_index) / 2;
        match ordered_elements[mid_index].cmp(&element_to_find) {
            Ordering::Less => lower_index = mid_index + 1, // the element in the middle is lower than the one we want to find, therefore, our new lower index starts from mid_index + 1 as lower indexes can be safely avoided
            Ordering::Greater => {
                if mid_index == 0 {
                    // If mid_index is 0, the element is not in the list
                    return None;
                }
                higher_index = mid_index - 1;
            } // the element in the middle is greater than the one we want to find, therefore, our new higher index starts from mid_index - 1 as higher indexes can be safely avoided
            Ordering::Equal => return Some(mid_index),
        }
    }

    return None;
}
