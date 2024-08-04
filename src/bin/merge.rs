use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let s = 10;
    let mut l1 = (0..s).map(|_| rng.gen_range(0..10)).collect::<Vec<_>>();
    l1.sort();
    let mut l2 = (0..s).map(|_| rng.gen_range(0..10)).collect::<Vec<_>>();
    l2.sort();

    println!("Sources: {:?} {:?}", l1, l2);

    let m = merge(l1, l2);

    println!("Merged: {:?}", m);
}

fn merge(l1: Vec<i32>, l2: Vec<i32>) -> Vec<i32> {
    let mut result = Vec::new();

    let mut l1 = l1.iter();
    let mut l2 = l2.iter();

    let mut o1 = l1.next();
    let mut o2 = l2.next();
    loop {
        match (o1, o2) {
            (None, None) => {
                break;
            }
            (None, Some(i2)) => {
                result.push(*i2);
                result.extend(l2);
                break;
            }
            (Some(i1), None) => {
                result.push(*i1);
                result.extend(l1);
                break;
            }
            (Some(i1), Some(i2)) => {
                if i1 < i2 {
                    result.push(*i1);
                    o1 = l1.next();
                } else {
                    result.push(*i2);
                    o2 = l2.next();
                }
            }
        }
    }
    result
}
