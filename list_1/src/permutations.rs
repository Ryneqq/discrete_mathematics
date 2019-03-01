pub fn run_examples() {
    dbg!(is_permutation(&[1, 2, 3, 4, 5, 6 ,7]));
    dbg!(is_permutation(&[1, 2, 3, 4, 5, 6 ,6]));
    dbg!(get_all_permutations(5));
}

fn is_permutation(perm: &[u64]) -> bool {
    let mut perm = Vec::from(perm);
    perm.sort();

    perm.windows(2)
        .skip_while(|con| con[0] != con[1])
        .next()
        .is_none()
}

fn get_all_permutations(n: usize) -> String {
    let mut result = String::new();

    generate_permutations(n).iter()
        .for_each(|perm| result.push_str(&format!("{:?}", perm)));

    result
}

fn generate_permutations(n: usize) -> Vec<Vec<usize>> {
    let mut tab: Vec<usize> = (0..n).collect();
    let mut result = vec![];

    permutate(&mut result, &mut tab, 0, n);

    result
}

fn permutate(result: &mut Vec<Vec<usize>>, tab: &mut Vec<usize>, lvl: usize, len: usize) {
    match lvl == len {
        true  => result.push(tab.clone()),
        false => {
            for i in lvl..len {
                tab.swap(lvl, i);
                permutate(result, tab, lvl + 1, len);
                tab.swap(lvl, i);
            }
        }
    }
}
