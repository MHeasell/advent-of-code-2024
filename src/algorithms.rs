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
