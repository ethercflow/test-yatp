use rand::Rng;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use yatp::task::callback::Handle;
use yatp::Builder;

fn scale_workers() {
    let pool = Builder::new("SP")
        .max_thread_count(16)
        .core_thread_count(4)
        .build_callback_pool();
    let handler = pool.remote().clone();
    let builder = thread::Builder::new().name("wl".to_string());
    builder
        .spawn(move || {
            loop {
                let (tx, rx) = mpsc::channel();
                // A bunch of tasks should be executed correctly.
                let cases: Vec<_> = (10..100000000).collect();
                for id in &cases {
                    let t = tx.clone();
                    let id = *id;
                    handler.spawn(move |_: &mut Handle<'_>| t.send(id).unwrap());
                }
                let mut ans = vec![];
                for _ in 10..100000000 {
                    let r = rx.recv_timeout(Duration::from_secs(1)).unwrap();
                    ans.push(r);
                }
                ans.sort();
                assert_eq!(cases, ans);
                println!("finish one loop");
            }
        })
        .unwrap();
    loop {
        let mut rng = rand::thread_rng();
        let new_thread_count = rng.gen_range(1..16);
        println!("scale workers to {}", new_thread_count);
        pool.remote().scale_workers(new_thread_count);
        thread::sleep(Duration::from_secs(10));
    }
}

fn main() {
    // scale_in_current_thread();
    scale_workers();
}
