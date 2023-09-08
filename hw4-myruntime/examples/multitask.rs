
use hw4_myruntime::executor::Executor;

fn sum() {
    for _i in 0..1000 {
        let mut _sum = 0;
        for _j in 0..1000 {
            _sum += 1;
        }
    }
}

async fn demo1(tx: async_channel::Sender<()>) {
    println!("demo1.");
    println!("100000 个数相加...");
    sum();
    println!("sleep 5s...");
    std::thread::sleep(std::time::Duration::from_secs(5));
    println!("唤醒 demo1");
    let _ = tx.send(()).await;
}

async fn demo2(tx: async_channel::Sender<()>) {
    println!("demo2.");
    println!("100000 个数相加...");
    sum();
    println!("sleep 5s...");
    std::thread::sleep(std::time::Duration::from_secs(5));
    println!("唤醒 demo2");
    let _ = tx.send(()).await;
}

async fn demo() {
    let (tx1, rx1) = async_channel::bounded::<()>(1);
    let (tx2, rx2) = async_channel::bounded::<()>(1);
    Executor::spawn(demo1(tx1));
    Executor::spawn(demo2(tx2));
    let _ = rx1.recv().await;
    let _ = rx2.recv().await;
}

fn main() {
    println!("Hello, world!");
    let ex = Executor::new(1);
    println!("测试多任务");
    ex.block_on(demo());
}