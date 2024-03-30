## 2023-3-29

学习rust异步编程库 ***tokio***

### 总结

+ future
 表示一个尚未完成计算的trait，可以在未来某个时刻完成。
  + src

  ```rust
  trait future {
    type Output;
    fn poll(
      // 固定self地址
      self: Pin<&mut Self>,
      // Context中提供一个Waker类型
      cx: &mut Context<'_>
    ) -> Poll<Self::Output>;
  }
  enum Poll<T> {
    Ready(T),
    Pending,
  }
  ```

+ 原理
  poll方法会被调用来检查Future是否已经完成，完成时提供一个结果值 Ready(output)，未完成时返回Pending

+ async  
 rust内置语法标识一个方法，使其返回值成为 impl future<Output = ()> 类型
  + demo

  ```rust, no_run
  use futures::executor::block_on;
  async fn myfunc() {
   println!("Hello async");
  }
  fn main() {
   // type of future = impl future<Output = ()>
   let future = myfunc();
   block_on(future);
  }
  ```

+ await

 rust内置语法，await表达式的计算结果为Future的最终值 

 ```rust,no_run
 Poll::Ready(Output)
 async fn myfunc() -> u16 {
  /* --skip-- */
  let v = some_io().await;
  /* --skip-- */
 }
 let future = myfunc();
 let val = future.await;
 ```

 rust从异步方法获取值的方法
 当首次轮询myfunc()返回的Future值时，会从函数的顶部开始执行，一直运行到some_io返回的Future的第一个await。await表达式会轮询some_io返回的Future
 直到返回Ready(Output)，然后把Output作为表达式的返回值，因此some_io.await大致等价于如下内容:
 伪代码:

 ```rust,no_run
 {
   let io_future = some_io();
   'retry_point:
   match io_future.poll(cx) {
    Poll::Ready(v) => v,
    Poll::Pending => {
      // 安排对'my_func()'返回的Future进行下一次'poll'
      // 下次poll在'retry_point'处开始执行
      // ----do something
      return Poll::Pending;
    }
   }
 } 
 ```

+ note
 只能在async方法中使用

## 易错
  + 不能在.await期间持有互斥锁
    
    ```rust,no_run
    async fn increment_and_do_stuff(mutex: &Mutex<i32>) {
        let mut lock: MutexGuard<i32> = mutex.lock().unwrap();
        *lock += 1;

        do_something_async().await;
    } // 锁在这里超出作用域
    ```

    如果spawn一个任务执行上面的函数，会报错。因为std::sync::MutexGuard类型没有实现Send特性，意味着不能将一个Mutex锁发送到另一个线程，因为.await可能会让任务转移到另一个线程上执行。

    * 提前释放锁
    ```rust,no_run
    async fn increment_and_do_stuff(mutex: &Mutex<i32>) {
        {
            let mut lock: MutexGuard<i32> = mutex.lock.unwrap();
            *lock += 1;
        }
        do_something_async().await;
    }

