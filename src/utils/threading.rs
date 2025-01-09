use std::thread::{
    available_parallelism,
    scope,
};

pub fn run_threads<T, U, F>(inputs: Vec<T>, fun: F) -> Vec<U>
where
    F: FnOnce(Vec<T>) -> U + Send + Copy,
    T: Send,
    U: Send,
{
    scope(|s| {
        let handles: Vec<_> = chunks(inputs)
            .into_iter()
            .map(|chunk| s.spawn(move || fun(chunk)))
            .collect();
        handles
            .into_iter()
            .map(|handle| handle.join().unwrap())
            .collect()
    })
}

fn chunks<T>(inputs: Vec<T>) -> Vec<Vec<T>> {
    let chunk_size = inputs.len().div_ceil(threads());
    inputs
        .into_iter()
        .enumerate()
        .fold(Vec::new(), |mut chunks, (i, item)| {
            if i % chunk_size == 0 {
                chunks.push(Vec::new());
            }
            chunks.last_mut().unwrap().push(item);
            chunks
        })
}

fn threads() -> usize {
    available_parallelism().unwrap().into()
}
