支持的协议有：tcp、udp、websocket、http、http2、http3、保留：grpc、mqtt、kafka

## 实现方式
- 终端类库：使用rust的ratatui，crossterm，clap(或structopt)构建一个终端的通讯协议调试工具
- 异步运行时使用Tokio库
- 错误处理使用anyhow库
- 序列化和反序列化使用serde库，json使用serde-json库
- tcp/udp相关使用Tokio库
- websocket使用tokio-tungstenite库
- http使用http库

## 页面布局
- 总体布局是上、中、下
- 上：顶部有统计状态：发送，接收，链接状态
- 中：左右或者上下布局，可以通过命令行参数进行控制布局，分为发送layout和接收layout，默认为左右布局
- 下：底部是状态栏，有快捷键提示信息

## 数据显示方式
接收区和发送区的数据显示格式
- 类似日志的显示方式：分两行显示，一行显示时间戳和连接相关信息，比如：tcp server接收区显示时间戳+IP+Port

## 多语言支持
- 通过配置文件即可实现多语言
- 检测当前环境，检测失败则默认为英语
- 检测成功，则选中

## TLS支持
- 使用rustls作为TLS的实现
- 用于websocket和http的TLS实现

## 快捷键
- ctrl+c，退出命令行
- Ctrl+I，弹出对话框，输入要发送的数据，可通过ctrl+v粘贴
    - tcp、udp、websocket可选择需要发送的数据类型，string or hex，hex可检测是否是有空格，有空格则split，没有空格则按两个字符一个hex来算

## 命令行
### tcp-server
- 侦听服务端端口
- 发送区显示服务端发送给客户端的数据
- 接收区显示来自客户端的请求和数据，也可以发送数据给客户端

多个客户端连接时的情况：
- 在接收区使用tab来表示多个客户端的连接，tab的title为客户端的IP和Port
- 在发送区使用tab来区分发送给哪个客户端，tab的title为客户端的IP和Port

发送数据：
- 快捷键ctrl+i，弹出发送消息窗口
- 可选择已经连接的客户端
- 可选择发送数据格式：hex，string

子命令：
nt tcp server | nt tcps
例子：
- nt tcp server 127.0.0.1:8000
- nt tcp server 8000

### tcp-client
- 连接到指定的服务端
- 发送区显示client向server发送的数据
- 接收区显示client接收到的server的数据

发送数据：
- 快捷键ctrl+i，弹出发送消息窗口
- 可选择已经连接的客户端
- 可选择发送数据格式：hex，string

子命令：
nt tcp client | nt tcpc
例子：
- nt tcp client 127.0.0.1:9000 192.168.0.11:9000
- nt tcp client 9000 192.168.0.11:9000
- nt tcp client 192.168.0.11:9000

### udp-server
- 侦听服务端端口
- 发送区显示服务端发送给客户端的数据
- 接收区显示来自客户端的请求和数据，也可以发送数据给客户端

多个客户端连接时的情况：
- 在接收区使用tab来表示多个客户端的连接，tab的title为客户端的IP和Port
- 在发送区使用tab来区分发送给哪个客户端，tab的title为客户端的IP和Port

发送数据：
- 快捷键ctrl+i，弹出发送消息窗口
- 可选择已经连接的客户端
- 可选择发送数据格式：hex，string

子命令：
nt udp server | nt udps
例子：
- nt udp server 127.0.0.1:8000
- nt udp 8000

### udp-client
- 发送区显示client向server发送的数据
- 接收区显示client接收到的server的数据

发送数据：
- 快捷键ctrl+i，弹出发送消息窗口
- 可选择已经连接的客户端
- 可选择发送数据格式：hex，string

子命令：
nt udp client | nt udpc
例子：
- nt udp client 127.0.0.1:9000 192.168.0.11:9000
- nt udp client 9000 192.168.0.11:9000
- nt udp client 192.168.0.11:9000

### websocket-server
- 侦听服务端端口
- 发送区显示服务端发送给客户端的数据
- 接收区显示来自客户端的请求和数据，也可以发送数据给客户端

多个客户端连接时的情况：
- 在接收区使用tab来表示多个客户端的连接，tab的title为客户端的IP和Port
- 在发送区使用tab来区分发送给哪个客户端，tab的title为客户端的IP和Port

发送数据：
- 快捷键ctrl+i，弹出发送消息窗口
- 可选择已经连接的客户端
- 可选择发送数据格式：hex，string

子命令：
nt websocket server | nt ws server
例子：
- nt websocket server 127.0.0.1:8000
- nt websocket server 8000

### websocket-client
- 发送区显示client向server发送的数据
- 接收区显示client接收到的server的数据

发送数据：
- 快捷键ctrl+i，弹出发送消息窗口
- 可选择已经连接的客户端
- 可选择发送数据格式：hex，string

子命令：
nt websocket client | nt ws client
例子：
- nt websocket client 127.0.0.1:9000 192.168.0.11:9000
- nt websocket client 9000 192.168.0.11:9000
- nt websocket client 192.168.0.11:9000

### http-server | http2-server | http3-server
- 侦听服务端端口
- 发送区显示服务端发送给客户端的数据
- 接收区显示来自客户端的请求和数据，也可以发送数据给客户端

接收多个客户端发送数据的情况：
- 所有客户端的数据都在接收区显示
- 通过客户端的IP进行区分

发送数据：
- 快捷键ctrl+i，由于是短链接，此快捷键无需响应

子命令：
nt http server
nt http2 server
nt http3 server
例子：
- nt http server 127.0.0.1:8000
- nt http server 8000
- nt http2 server 127.0.0.1:8000
- nt http2 server 8000
- nt http3 server 127.0.0.1:8000
- nt http3 server 8000

### http-client	| http2-client | http3-client
保留

### grpc-server	
保留

### grpc-client	
保留

### mqtt-server	
保留

### mqtt-client	
保留

### kafka-client 
保留
