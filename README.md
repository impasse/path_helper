# path_helper

> 增强版的 macOS /usr/libexec/path_helper 实现

更改了系统 path_helper 的读取顺序。

改为

- /etc/paths.d/0.+
- /etc/paths
- /etc/paths.d/[^0]+

同时增加了排序。

### 安装
- git clone this repo
- cargo build --release
- cp target/release/path_helper /usr/local/bin/
- replace `/usr/libexec/path_helper` to `/usr/local/bin/path_helper` in /etc/zprofile
- reopen terminal window

