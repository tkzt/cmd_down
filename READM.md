# Cmd 按键状态提示工具

由于时间与地域的关系，手上这台 MacBook Air 的左侧 `command` 疑似微坏，有时处于按下状态，手动再按下一次方可解除。这很影响体验，遂基于 Tauri 实现了个检测和提示的小工具。

## 功能

- 启动后仅显示在菜单栏，不创建窗口
- 检测左侧 Cmd 是否持续按下超过阈值
- 超过阈值后，菜单栏标题显示为 🍎
- 未超过阈值时，菜单栏标题显示为 🍏
- 阈值支持在菜单栏菜单中切换：1s / 2s / 3s / 5s

## 开发环境

- macOS
- Rust stable
- Xcode Command Line Tools

## 运行

```bash
cd src-tauri
cargo tauri dev
```

## 打包

```bash
./build.sh
```

打包产物目录：

- src-tauri/target/release/bundle/

## 说明

- 键盘全局状态读取依赖系统能力。若检测异常，请确认系统权限设置允许应用正常访问输入事件。