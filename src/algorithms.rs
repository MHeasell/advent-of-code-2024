use std::{collections::HashSet, hash::Hash};

/// Returns (steps before loop, loop length).
///
/// Steps before loop is an overestimation.
/// It is always some multiple of the loop length.
/// After taking that many steps you are guaranteed
/// to be inside the loop, but it doesn't tell you
/// exactly where the loop starts.
pub fn detect_loop<T, A>(it: &A) -> Option<(usize, usize)>
where
    T: Eq,
    A: Iterator<Item = T> + Clone,
{
    let mut a = it.clone();
    let mut b = it.clone();

    let mut tortoise = a.next()?;
    b.next()?;
    let mut hare = b.next()?;

    let mut steps = 1;

    while tortoise != hare {
        tortoise = a.next()?;
        b.next()?;
        hare = b.next()?;
        steps += 1;
    }

    let loop_length = a.take_while(|x| *x != hare).count() + 1;

    Some((steps, loop_length))
}

pub fn flood_fill<T, I, F>(start: T, mut succ: F) -> HashSet<T>
where
    T: Eq + Hash + Copy,
    I: Iterator<Item = T>,
    F: FnMut(&T) -> I,
{
    let mut stack = vec![start];
    let mut seen = HashSet::from([start]);

    while let Some(elem) = stack.pop() {
        let neighbours = succ(&elem);
        for n in neighbours {
            if seen.insert(n) {
                stack.push(n);
            }
        }
    }

    seen
}
