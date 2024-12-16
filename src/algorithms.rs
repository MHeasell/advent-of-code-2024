use std::{
    collections::{HashSet, VecDeque},
    hash::Hash,
};

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

pub fn flood_fill2<T, F>(start: &[T], succ: F) -> HashSet<T>
where
    T: Eq + Hash + Copy,
    F: Fn(&T) -> Vec<T>,
{
    let mut stack = Vec::from(start);
    let mut seen = HashSet::from_iter(start.iter().copied());

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

/// Inserts the given value into the queue
/// with the given priority.
///
/// If the value already exists in the queue
/// and its priority value is greater than the priority given,
/// the existing value's priority is lowered.
///
/// If the existing value's priority is lower than the priority given
/// then the existing value is left unchanged.
pub fn priority_queue_insert<T: Eq>(queue: &mut VecDeque<(T, i64)>, value: T, priority: i64) {
    let existing_elem = queue.iter().enumerate().find(|(_, elem)| elem.0 == value);

    match existing_elem {
        Some((_, elem)) if elem.1 <= priority => {
            return;
        }
        Some((i, _)) => {
            queue.remove(i);
        }
        None => {}
    };

    let insert_index = queue
        .iter()
        .enumerate()
        .find_map(|(i, elem)| (elem.1 > priority).then_some(i));

    queue.insert(insert_index.unwrap_or(queue.len()), (value, priority));
}

pub fn dijkstra_search<T, Succ, GPred>(
    start: &[T],
    get_successors: Succ,
    is_goal: GPred,
) -> Option<i64>
where
    T: Hash + Eq + Copy,
    Succ: Fn(&T) -> Vec<(T, i64)>,
    GPred: Fn(&T) -> bool,
{
    let mut open_list = VecDeque::<(T, i64)>::new();
    let mut closed_set = HashSet::<T>::new();

    for s in start {
        open_list.push_back((*s, 0));
    }

    while let Some((value, cost)) = open_list.pop_front() {
        if is_goal(&value) {
            return Some(cost);
        }

        closed_set.insert(value);

        for (successor_val, successor_cost) in get_successors(&value) {
            if closed_set.contains(&successor_val) {
                continue;
            }

            priority_queue_insert(&mut open_list, successor_val, cost + successor_cost);
        }
    }

    None
}
