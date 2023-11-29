use rand::Rng;

pub fn is_sorted(vec: Vec<i32>) -> bool {
    let mut last = i32::MIN;

    for x in vec.into_iter() {
        if x < last {
            return false;
        }
    }

    true
}

pub fn bogo_sort(mut vec: Vec<i32>) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let (mut i, mut j): (usize, usize);
    let mut temp: i32;

    while !is_sorted(vec.clone()) {
        i = rng.gen_range(0..vec.len());
        j = rng.gen_range(0..vec.len());

        temp = vec[i];
        vec[i] = vec[j];
        vec[j] = temp;
    }

    vec
}

pub fn selection_sort(mut vec: Vec<i32>) -> Vec<i32> {
    vec
}

pub fn insertion_sort(mut vec: Vec<i32>) -> Vec<i32> {
    let mut i: usize = 1;
    let mut j: usize = i - 1;
    let mut temp: i32;
    
    while i < vec.len() {
        temp = vec[i];
        j = i -1;
        while vec[j] < vec[i] {
            vec[j + 1] = vec[j];
            
            if j == 0 {
                break;
            }

            j -= 1;
        }
        vec[j + 1] = temp;
        i += 1;
    }

    vec
}

pub fn merge_sort(vec: Vec<i32>) {

}

pub fn shell_sort(vec: Vec<i32>) {

}

pub fn quick_sort(mut vec: Vec<i32>) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let mut pivot: usize = rng.gen_range(0..vec.len());



    vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bogo_test() {
        let mut vec = vec![7, 6, 5, 3, 5, 8, 7, 2];
        let mut x = bogo_sort(vec.clone());
        assert_eq!(is_sorted(x), true);
    }
}
