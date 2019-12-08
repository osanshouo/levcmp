
pub fn levenshtein_distance<T: PartialEq>(u1: &[T], u2: &[T]) -> usize {
    if u1.len() > u2.len() {
        lev_distance(u1, u2)
    } else {
        lev_distance(u2, u1)
    }
}

fn lev_distance<T: PartialEq>(u1: &[T], u2: &[T]) -> usize {
    let n2 = u2.len();
    let mut score = (0..n2+1).collect::<Vec<_>>();
    
    for (i, c1) in u1.iter().enumerate() {
        let mut tmp = Vec::with_capacity(n2);
        tmp.push(i);
        
        for (j, c2) in u2.iter().enumerate() {
            let cost = if c1 == c2 { 0 } else { 1 };
            tmp.push(minimum(
                tmp[j]+1, score[j+1]+1, score[j]+cost
            ));
        }

        score = tmp;
    }
    
    score.pop().unwrap()
}

#[inline]
fn minimum(x: usize, y: usize, z: usize) -> usize {
    x.min(y).min(z)
}


#[cfg(test)]
mod tests {
    use crate::levenshtein_distance;

    #[test]
    fn it_works() {
        let u1 = "kitten".chars().collect::<Vec<_>>();
        let u2 = "sitting".chars().collect::<Vec<_>>();

        let dist = levenshtein_distance(&u1, &u2);
        assert_eq!( dist, 3 );
    }
}
