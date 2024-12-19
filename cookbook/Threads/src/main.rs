fn main() {
    let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let max = find_max(&arr);
    println!("max: {}", max.unwrap());
}

fn find_max(arr: &[i32]) -> Option<i32> {
    const THRESHOLD: usize = 2;

    if arr.len() <= THRESHOLD {
        // cloned() 方法将迭代器中的元素克隆到一个新的迭代器中
        // cloned() 不是马上复制对象，而是在迭代器中复制对象
        // 这里可以认为的max的遍历是惰性的，只有在需要的时候才会复制对象
        return arr.iter().cloned().max();
    }

    let mid = arr.len() / 2;
    // split_at -> [0, mid) [mid, len)
    let (left, right) = arr.split_at(mid);

    // 在里面的创建的线程 会等待其完成 才会返回
    std::thread::scope(|scope| {
        let thread_l = scope.spawn(|| find_max(left));
        let thread_r = scope.spawn(|| find_max(right));

        let max_l = thread_l.join().unwrap()?;
        let max_r = thread_r.join().unwrap()?;

        Some(max_l.max(max_r))
    })
}