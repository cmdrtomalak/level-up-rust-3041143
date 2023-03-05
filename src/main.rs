fn median(a: Vec<f32>) -> Option<f32> {
    let mut b = a.clone();
    b.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mid = b.len() / 2;
    if mid % 2 == 0 {
        if mid == 0 {
            return None;
        }
        match (b.get(mid - 1), b.get(mid)) {
            (Some(a), Some(b)) => {
                Some((a.clone() + b.clone()) / 2.0)
            },
            _ => None,
        }
    } else {
        match b.get(mid) {
            Some(a) => {
                Some(a.clone())
            },
            _ => None,
        }
    }
}

fn main() {
    let answer = median(vec![1.0, 2.0, 5.0]);

    println!("median([1,2,5]) = {:?}", answer);
}

#[test]
fn empty_list() {
    let input = vec![];
    let expected_output = None;
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn sorted_list() {
    let input = vec![1.0, 4.0, 5.0];
    let expected_output = Some(4.0);
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn even_length() {
    let input = vec![1.0, 3.0, 5.0, 6.0];
    let expected_output = Some(4.0);
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn unsorted_list() {
    let input = vec![1.0, 5.0, 2.0];
    let expected_output = Some(2.0);
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}
