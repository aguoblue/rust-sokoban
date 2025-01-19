? 传播错误
如果 Result 的值是 Ok，这个表达式将会返回 Ok 中的值而程序将继续执行。如果值是 Err，Err 将作为整个函数的返回值，就好像使用了 return 关键字一样，这样错误值就被传播给了调用者。


配置属性，构造context，把事件循环event_loop和context绑定

run传进可变的context，可变的game
用事件循环event_loop调用run处理闭包，底层是loop循环

移交了game的所有权，game可变，game实现了event::EventHandler<ggez::GameError>

