# surl

## 关于这个项目

> 项目由 Krcc 和 Keyboard 自动生成，本人不对本项目负责

这个项目是便于自己能够快速的创建短链接而写

你需要搭配 [nelsontky/gh-pages-url-shortener][1] 使用

## 项目优点

- ✨ 使用 Rust 编写，执行速度快
- 👾 修正的目录读取机制，从程序所在目录读写配置（而不是运行目录）
- 🛡️ Token 经过简单加密，安全（**不泄露 `config.json` 的前提下**）
- 🤏 输出简明，报错附带代码执行输出，添加短链输出 issue ID

## 项目缺点

- 使用 `reqwest`，包体大（~6MB）

## 项目使用

1. 下载源码并使用 `cargo build` 编译可执行文件
2. 执行 `./surl` 并按照提示提供信息
3. 复制 `surl` 及 `config.json` 到指定目录
4. 添加目录到环境变量
5. 重新终端，然后享受它！

[1]: <https://github.com/nelsontky/gh-pages-url-shortener>
