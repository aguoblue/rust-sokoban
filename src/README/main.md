? 传播错误  
如果 Result 的值是 Ok，这个表达式将会返回 Ok 中的值而程序将继续执行。如果值是 Err，Err 将作为整个函数的返回值，就好像使用了 return 关键字一样，这样错误值就被传播给了调用者。  

配置属性，构造context，把事件循环event_loop和context绑定  

run传进可变的context，可变的game  
用事件循环event_loop调用run处理闭包，底层是loop循环  

移交了game的所有权，game可变，game实现了event::EventHandler<ggez::GameError>  

------------------------  

初始化，组件的生成，绑定到world  
world作为game的属性  
EventHandler渲染  

------------------------  
不可变借用(&Position, &Renderable)  

------------------------  
可变借用(&mut Position, &Player)， 修改状态  

------------------------  
主要逻辑还是在run_input上，根据键盘输入来修改组件状态，组件渲染由框架完成  
一开始的疑问是mov  (&Position, &Movable)是不可变借用
而玩家是可移动的，(&mut Position, &Player)是可变借用
一开始是往类型不同去思考的，虽然不影响后续改变world里面的组件，后续还是利用可变借用来改变组件的状态  
gpt解释mov 是已拷贝的数据，与 world 的借用无关。collect() 消耗了迭代器，将所有数据拷贝到 HashMap。

------------------------
模块化