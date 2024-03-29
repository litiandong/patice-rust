# rust练习

## 2023-3-29
学习rust异步编程库 ***tokio***
**总结**
- 基础
	- future
		表示一个尚未完成计算的trait，可以在未来某个时刻完成。
		- src
			trait future {
				type Output;
				fn poll(self: Pin<&mut self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
			}
		- 
完成时提供一个结果值 Ready(output)，未完成时
	- async		
		标识一个方法，使其返回值成为 impl future<Output = ()> 类型
	- await


