
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;
use std::sync::{Condvar, Mutex, Arc};
use std::task::{Context, Waker, RawWaker, RawWakerVTable, Poll, Wake};
use std::time::Duration;

use futures::FutureExt;
use futures::future::LocalBoxFuture;
use futures::future::BoxFuture;
use futures::task::SpawnExt;    

struct Demo;

impl Future for Demo {
    type Output = ();

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> std::task::Poll<Self::Output> {
        println!("demo_1 (Future handmade)");
        std::task::Poll::Ready(())
    }
}

fn dummy_waker() -> Waker {
    static DATA: () = ();
    unsafe { Waker::from_raw(RawWaker::new(&DATA, &VTABLE)) }
}
const VTABLE: RawWakerVTable = RawWakerVTable::new(vtable_clone, vtable_wake, vtable_wake_by_ref, vtable_drop);
unsafe fn vtable_clone(_p: *const ()) -> RawWaker { RawWaker::new(_p, &VTABLE) }
unsafe fn vtable_wake(_p: *const ()) { }
unsafe fn vtable_wake_by_ref(_p: *const ()) { }
unsafe fn vtable_drop(_p: *const ()) { }

fn block_on<F: Future>(future: F) -> F::Output {
    let mut fut = std::pin::pin!(future);
    let waker = dummy_waker();
    let mut cx = Context::from_waker(&waker);
    loop {
        if let Poll::Ready(output) = fut.as_mut().poll(&mut cx) {
            return output;
        }
    }
}

async fn demo_1() {
    println!("demo_1");
}

async fn demo_2() {
    let (tx, rx) = async_channel::bounded::<()>(1);
    std::thread::spawn(move || {
        std::thread::sleep(Duration::from_secs(2));
        let _ = tx.send_blocking(());
    });
    let _ = rx.recv().await;
    println!("demo_2");
}

fn main() {
    println!("Hello, world!");
    // block_on(demo_1());
    // block_on(demo_2());
    block_on_3(demo());
}

struct Signal {
    state: Mutex<State>,
    cond: Condvar,
}

enum State {
    Empty,
    Waiting,
    Notified,
}

impl Signal {
    fn new () -> Self {
        Self {
            state: Mutex::new(State::Empty),
            cond: Condvar::new(),
        }
    }
    fn wait(&self) {
        let mut state = self.state.lock().unwrap();
        match *state {
            State::Notified => *state = State::Empty,
            State::Waiting => {
                panic!("multiple wait");
            }
            State::Empty => {
                *state = State::Waiting;
                while let State::Waiting = *state {
                    state = self.cond.wait(state).unwrap();
                }
            }
        }
    }
    
    fn notify(&self) {
        let mut state = self.state.lock().unwrap();
        match *state {
            State::Notified => {}
            State::Empty => *state = State::Notified,
            State::Waiting => {
                *state = State::Empty;
                self.cond.notify_one();
            }
        }
    }
}

impl Wake for Signal {
    fn wake(self: Arc<Self>) {
        self.notify();
    }
}

fn block_on_2<F: Future>(future: F) -> F::Output {
    let mut fut = std::pin::pin!(future);
    let signal = Arc::new(Signal::new());
    let waker = Waker::from(signal.clone());
    let mut cx = Context::from_waker(&waker);
    loop {
        if let Poll::Ready(output) = fut.as_mut().poll(&mut cx) {
            return output;
        }
        signal.wait();
    }
}

struct Task {
    future: RefCell<BoxFuture<'static, ()>>,
    signal: Arc<Signal>,
}

struct TaskQueue {
    queue: RefCell<VecDeque<Arc<Task>>>,
}

impl TaskQueue {
    pub(crate) fn push(&self, runnable: Arc<Task>) {
        self.queue.borrow_mut().push_back(runnable);
    }
    pub fn pop(&self) -> Option<Arc<Task>> {
        self.queue.borrow_mut().pop_front()
    }
}

unsafe impl Send for Task {}
unsafe impl Sync for Task {}

impl Wake for Task {
    fn wake(self: Arc<Self>) {
        RUNNABLE.with(|runnable| runnable.lock().unwrap().push_back(self.clone()));
        self.signal.notify();
    }
}

scoped_tls::scoped_thread_local!(static SIGNAL: Arc<Signal>);
scoped_tls::scoped_thread_local!(static RUNNABLE: Mutex<VecDeque<Arc<Task>>>);

fn block_on_3<F: Future>(future: F) -> F::Output {
    let mut fut = std::pin::pin!(future);
    let runnable = Mutex::new(VecDeque::with_capacity(1024));
    let signal = Arc::new(Signal::new());
    let waker = Waker::from(signal.clone());
    let mut cx = Context::from_waker(&waker);
    SIGNAL.set(&signal, || {
        RUNNABLE.set(&&runnable, || {
            loop {
                if let Poll::Ready(output) = fut.as_mut().poll(&mut cx) {
                    return output;
                }
                while let Some(task) = runnable.lock().unwrap().pop_front() {
                    let waker = Waker::from(task.clone());
                    let mut cx = Context::from_waker(&waker);
                    let _ = task.future.borrow_mut().as_mut().poll(&mut cx);
                }
                signal.wait();
            }
        })
    })
}

async fn demo2(tx: async_channel::Sender<()>) {
    println!("multitask demo2 hello world2!");
    let _ = tx.send(()).await;
}

async fn demo() {
    let (tx, rx) = async_channel::bounded(1);
    spawn(demo2(tx));
    println!("multitask demo hello world!");
    let _ = rx.recv().await;
}

pub fn spawn(fut: impl Future<Output = ()> + 'static + std::marker::Send) {
    let t = Arc::new(Task {
        future: RefCell::new(fut.boxed()),
        signal: Arc::new(Signal::new())
    });
    RUNNABLE.with(|ex: &Mutex<VecDeque<Arc<Task>>>| ex.lock().unwrap().push_back(t));
}