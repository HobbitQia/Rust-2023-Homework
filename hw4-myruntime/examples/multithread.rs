use std::{time::Instant, sync::Arc};

use hw4_myruntime::executor::Executor;

static SIZE: usize = 1919810;
static THREADS: usize = 50;

async fn calculate_helper
(
    v: Arc<Vec<i64>>, 
    st: usize, 
    ed: usize, 
    tx: Arc<async_channel::Sender<i64>>
) {
    let mut sum = 0;
    for i in st..ed {
        sum += v[i] as i64;
    }
    let _ = tx.send(sum as i64).await;
}

async fn calculate(v: Arc<Vec<i64>>, threads: usize) -> i64 {
    let (tx, rx) = async_channel::bounded::<i64>(threads);
    let tx = Arc::new(tx);
    let mut st: usize = 0;
    let cnt = SIZE / threads;
    for _ in 0..threads-1 {
        Executor::spawn(calculate_helper(v.clone(), st, st+cnt, tx.clone()));
        st += cnt;
    }
    Executor::spawn(calculate_helper(v.clone(), st, v.len(), tx.clone()));

    let mut sum: i64 = 0;
    for _ in 0..threads {
        sum += rx.recv().await.unwrap();
    }
    sum
}


fn main() {
    println!("测试多线程性能");
    let v = vec![411; SIZE];
    let v = Arc::new(v);
    let sum_ans:i64 = v.iter().sum();
    println!("Ans 应为 {}", sum_ans);
    println!("测试单线程：");
    let now = Instant::now();
    let ex = Executor::new(1);
    let sum = ex.block_on(calculate(v.clone(), 1));
    assert_eq!(sum, sum_ans);
    println!("sum = {}", sum);
    println!("time = {}ms", now.elapsed().as_millis());
    println!("测试多线程：（使用线程为 {} 个）", THREADS);
    let now = Instant::now();
    let ex = Executor::new(THREADS);
    let sum = ex.block_on(calculate(v.clone(), THREADS));
    assert_eq!(sum, sum_ans);
    println!("sum = {}", sum);
    println!("time = {}ms", now.elapsed().as_millis());
}