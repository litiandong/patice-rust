# 2024-2-20

**多线程编程 in rust**

### 创建线程

  使用thread::spawn创建线程
  示例:

  ```rust,no_run
  use std::thread;
  use std::time::Duration;
  
  fn main() {
      thread::spawn(|| {
          for i in 1..10 {
              println!("Hi number {i} from the spawned thread!");
              thread::sleep(Duration::from_millis(1));
          }
      });
  
      for i in 1..5 {
          println!("Hi number {i} from the main thread!");
          thread::sleep(Duration::from_millis(2));
      }
  }
  ```
  note:
  + 线程内部的代码使用闭包执行

### 等待自线程结束

  调用join()可以让当前线程阻塞，直到它等待的子线程结束。
  示例:

  ```rust,no_run
  use std::thread;
  use std::time::Duration;
  
  fn main() {
      let handle = thread::spawn(|| {
          for i in 1..10 {
              println!("Hi number {i} from the spawned thread!");
              thread::sleep(Duration::from_millis(1));
          }
      });
    
      for i in 1..5 {
          println!("Hi number {i} from the main thread!");
          thread::sleep(Duration::from_millis(2));
      }
      handle.join().unwrap();
  }
  ```

## 多线程中所有权与生命周期问题

    * 学前思考:

    rust以安全可靠的特性闻名，而这个特性的基础就是rust对所有权与生命周期的严格要求。
    所有线程都会在main线程结束后结束，main线程的生命周期就是'static，那么其他线程呢，其他线程的生命周期是根据创建其的父线程决定还是由其他方式决定呢。
    如果把一个栈上数据的所有权交给一个线程，那么线程持有这个数据的方式是什么，直接拿走数据的指针还是复制到自己的栈上呢（假设rust的线程与c语言一样有自己的独立栈）。
    栈上数据的所有权交给线程后，数据的生命周期会是什么。

    如果有以下场景:
          

















