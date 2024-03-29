# rust练习
学习rust笔记

## 2023-3-29
学习rust异步编程库 ***tokio***
**总结**
- future
	表示一个尚未完成计算的trait，可以在未来某个时刻完成。
	- src
		~~~rust
		trait future {
			type Output;
			fn poll(
				// 固定self地址
				self: Pin<&mut Self>,
				// Context中提供一个Waker类型
				cx: &mut Context<'_>)
				-> Poll<Self::Output>;
		}
		~~~
	- 原理
		poll方法会被调用来检查Future是否已经完成，完成时提供一个结果值 Ready(output)，未完成时返回Pending
- async		
	标识一个方法，使其返回值成为 impl future<Output = ()> 类型
	- demo
		~~~rust
		use futures::executor::block_on;
		async fn myfunc() {
			println!("Hello async");
		}
		fn main() {
			// type of future = impl future<Output = ()>
			let future = myfunc();
			block_on(future);
		}
- await
	



