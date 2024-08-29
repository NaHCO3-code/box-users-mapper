# Box Users Mapper

爬取box3的用户数据，以便查询之用。

# 使用方法
## 查询用户/统计数据
直接下载项目目录里的box3-user-data.csv

## 爬取用户数据
见下文贡献方法

# 贡献方法

box3的用户数量很多，而且需要考虑反爬，经过测试，每秒最多发送两个请求。

目前只爬取了非常小一部分的用户数据。欢迎各位帮助壮大数据库。

## 1. 配置环境并编译代码
1. [安装rust](https://course.rs/first-try/installation.html)
2. 配置 rust 环境：打开终端 (cmd) ，运行`rustup default stable`
3. 下载项目：`git clone git@github.com:NaHCO3-code/box-users-mapper.git`
4. 编译代码：`cargo build --release`

## 2. 爬取数据
`cargo run --release -- query <startid> <endid>`
这表示，扫描id从`startid`到`endid`的box3用户数据。
id尽量和已有的数据错开。

## 3. 提交
### 提交代码
我只是 rust 初学者，如果你有代码方面的建议欢迎提pull request。
### 提交数据
非常欢迎补全数据库。在提交数据时，请注意以下几点：
- 提交信息请写明你爬取的数据的起始和终止id。例如：1000-5000
- 请不要运行`cargo run --release -- merge`。这个命令会整合所有已经爬取的数据到一个文件里，但是这可能会导致代码冲突。我会定期进行整合。如果你有整合数据的需要，可以将原来的csv文件备份复制到其他地方，提交的时候再复制回来。
- 请不要删除已有的数据，哪怕你发现部分数据发生了重叠或者其他异常情况。如果发现了这类问题，请提issue。
- 如果你对这些数据感兴趣，可以将csv文件直接导入到你的电子表格软件。但是，如果你对这些数据进行了分析或者排序，请保存到其他位置，不要覆盖原始数据。同样，这可能导致代码冲突。
