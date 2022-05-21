use rand::{thread_rng, Rng};

pub fn random_range(min: usize, max: usize, inclusive: bool) -> usize {
    let mut rng = thread_rng();

    if inclusive {
        return rng.gen_range(min..=max);
    }

    return rng.gen_range(min..max);
}
