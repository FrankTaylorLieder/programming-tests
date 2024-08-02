fn main() {
    let mut input = vec![1, 50, 2, -30, 67, 42];
    input.sort();

    println!(" Input: {:?}", input);

    for i in [42, 1, -30, 67, 100, -56] {
        println!("bin_search({}) = {:?}", i, bin_search(&input, i));
    }
}

fn bin_search<T: Ord>(input: &[T], x: T) -> Option<usize> {
    bin_search_aux(input, x, 0)
}

fn bin_search_aux<T: Ord>(input: &[T], x: T, prefix: usize) -> Option<usize> {
    match input {
        [] => None,
        [a] => {
            if *a == x {
                Some(prefix)
            } else {
                None
            }
        }
        _ => {
            let m = input.len() / 2;
            let p = &input[m];

            if x == *p {
                Some(prefix + m)
            } else if x < *p {
                bin_search_aux(&input[0..m], x, prefix)
            } else {
                bin_search_aux(&input[m + 1..], x, prefix + m)
            }
        }
    }
}
