/// Spritesheet index
#[inline(always)]
pub fn ss_idx(x: u16, y: u16) -> u16 {
    x / 8 + y / 8 * 16
}

pub fn make_weighted_vec<T: Copy>(items: &[(T, usize)]) -> Vec<T> {
    let mut values = items.iter().map(|(v, t)| vec![*v; *t]).collect::<Vec<_>>();
    let values = values
        .iter_mut()
        .reduce(|a, b| {
            a.append(b);
            a
        })
        .unwrap();
    values.to_vec()
}

// https://stackoverflow.com/a/7616484
pub fn str_to_hashed(str: String) -> u64 {
    let mut hash = 0;
    if str.len() == 0 {
        return 0;
    }
    for chr in str.chars() {
        hash = ((hash << 5) - hash) + chr as u64;
        hash |= 0;
    }
    hash
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ss_idx() {
        assert_eq!(ss_idx(0, 0), 0);
        assert_eq!(ss_idx(8, 0), 1);
        assert_eq!(ss_idx(0, 8), 16);
        assert_eq!(ss_idx(8, 8), 17);
        assert_eq!(ss_idx(120, 8), 31);
    }

    #[test]
    fn test_make_weighted_vec() {
        assert_eq!(
            make_weighted_vec(&[('a', 3), ('b', 4), ('c', 3)]),
            ['a', 'a', 'a', 'b', 'b', 'b', 'b', 'c', 'c', 'c']
        );
    }
}
