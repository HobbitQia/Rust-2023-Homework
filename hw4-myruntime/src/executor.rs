/*  executor.rs
    通过 Executor 机制，我们可以执行任务。具体分为了下面几个部分：
    1. Task：任务，包含了一个 Future 和一个 Signal
    2. TaskQueue：任务队列，用于存放任务。
    3. Executor：执行器，用于新建任务，并提供 block_on 方法来调度完成任务。
    4. Worker：工作线程，用于执行任务。
    5. Pool：线程池，用于管理工作线程。
*/
use std::{
    cell::RefCell,
    task::Wake,
    sync::{Arc, Mutex, mpsc},
    collections::VecDeque,
    future::Future,
    task::{Context, Poll, Waker},
    thread::JoinHandle,
};

use futures::{future::BoxFuture, FutureExt};
use scoped_tls::scoped_thread_local;

use crate::waker::Signal;
scoped_thread_local!(pub(crate) static EX: Executor);

/*  Task
    任务，将 Future 和 Signal 封装在一起。
*/
pub struct Task {
    future: RefCell<BoxFuture<'static, ()>>,
    signal: Arc<Signal>,
}

unsafe impl Send for Task {}
unsafe impl Sync for Task {}

impl Wake for Task {
    fn wake(self: Arc<Self>) {
        self.wake_by_ref();
    }

    fn wake_by_ref(self: &Arc<Self>) {
        EX.with( |ex| ex.task_queue.push(self.clone()) );
        self.signal.notify();
    }
}

/*  TaskQueue
    任务队列，用于存放任务。
    由于我们的线程池是固定大小的，所以我们可以使用一个 VecDeque 来存放任务。
*/
struct TaskQueue {
    queue: RefCell<VecDeque<Arc<Task>>>,        // RefCell 用于内部可变性，这样可以修改内部的元素。
}

impl TaskQueue {
    pub fn new() -> Self {
        const DEFAULT_CAPACITY: usize = 1024;
        TaskQueue {
            queue: RefCell::new(VecDeque::with_capacity(DEFAULT_CAPACITY)),
        }
    }

    pub(crate) fn push(&self, runnable: Arc<Task>) {
        self.queue.borrow_mut().push_back(runnable);
    }
    pub fn pop(&self) -> Option<Arc<Task>> {
        self.queue.borrow_mut().pop_front()
    }
}

/*  Executor
    执行器
*/
pub struct Executor {
    task_queue: TaskQueue,      // 任务队列
    thread_pool: Pool           // 线程池，用于实现线程调度（线程池的实现在下面）
}

impl Executor {
    pub fn new(size: usize) -> Self {
        Executor {
            task_queue: TaskQueue::new(),
            thread_pool: Pool::new(size)
        }
    }
    /*  spawn
        新建一个任务，并将其放入任务队列中。
    */
    pub fn spawn(fut: impl Future<Output = ()> + 'static + std::marker::Send) {
        let t = Arc::new(Task {
            future: RefCell::new(fut.boxed()),
            signal: Arc::new(Signal::new())
        });
        EX.with( |ex| ex.task_queue.push(t.clone()) );
        // self.task_queue.push(t);
    }
    /*  block_on
        调度任务，直到任务完成。
    */
    pub fn block_on<F: Future>(&self, future: F) -> F::Output {
        let signal = Arc::new(Signal::new());
        let waker = Waker::from(signal.clone());
        let mut cx = Context::from_waker(&waker);
        EX.set(self, || {
            let mut fut = std::pin::pin!(future);
            loop {
                if let Poll::Ready(output) = fut.as_mut().poll(&mut cx) {
                    return output;
                }
                while let Some(task) = self.task_queue.pop() {
                    self.thread_pool.execute(task);
                }
                if let Poll::Ready(output) = fut.as_mut().poll(&mut cx) {
                    return output;
                }
                signal.wait();
            }
        })
    }
}

/*  Worker  
    工作线程，用于执行任务。（配合下面的线程池使用）
*/
#[allow(unused)]
pub struct Worker {
    w_id: usize,            // 工作线程的 id（这里没有用上）
    w_thread: Option<JoinHandle<()>>,       // 工作线程
}

impl Worker {
    fn new(id: usize, receiver: Arc::<Mutex<mpsc::Receiver<Option<Arc<Task>>>>>) -> Self {
        let thread = std::thread::spawn(move || {
            loop {
                let message = receiver.lock().unwrap().recv().unwrap();
                // let tmp = receiver.lock().unwrap();
                // let message = tmp.recv().unwrap();
                match message {
                    Some(task) => {
                        let waker = Waker::from(task.clone());
                        let mut cx = Context::from_waker(&waker);
                        let _ = task.future.borrow_mut().as_mut().poll(&mut cx);
                    }
                    None => {       // 当收到 None 时，说明线程池已经被 drop，这时候可以终止当前线程的运行了。
                        break;  
                    }
                }
            }   
        });
        Worker {
            w_id: id,
            w_thread: Some(thread),
        }
    }
}

/*  Pool
    线程池，用于管理工作线程。
*/
pub struct Pool {
    workers: Vec<Worker>,           
    max_workers: usize,
    sender: mpsc::Sender<Option<Arc<Task>>>     
}
  
impl Pool {
    pub fn new(max_workers: usize) -> Pool {
        let (tx, rx) = mpsc::channel::<Option<Arc<Task>>>();
        let mut workers = Vec::with_capacity(max_workers);
        let receiver = Arc::new(Mutex::new(rx));
        for i in 0..max_workers {
            workers.push(Worker::new(i, receiver.clone()));
        }
        Pool {
            max_workers, 
            workers, 
            sender: tx
        }
}
    // F 是一个能在线程里执行的闭包函数。
    pub fn execute(&self, task: Arc<Task>){
        let _ = self.sender.send(Some(task));
    }
}
/*  Drop
    当线程池被 drop 时，我们需要将线程池中的线程 drop 掉。
*/
impl Drop for Pool {
    fn drop(&mut self) {
        // 注意这里两个循环必须分开写，否则会有死锁情况。
        for _ in 0..self.max_workers {
            let _ = self.sender.send(None);
        }
        for worker in &mut self.workers {
            if let Some(thread) = worker.w_thread.take() {
                let _ = thread.join();
            }
        }
    }
}