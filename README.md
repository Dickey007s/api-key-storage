# API Key Storage

一个简洁的本地 API Key 管理工具，基于 Tauri 2 + Svelte 5 构建。

## 功能

- 本地存储 API 凭证，数据不离开设备
- 支持多个平台的 Key 管理
- 快速复制 Base URL 和 API Key
- 密钥默认隐藏，点击显示

## 技术栈

- **前端**: Svelte 5 + TypeScript + Vite
- **后端**: Rust + Tauri 2
- **存储**: 本地 JSON 文件

## 开发

```bash
# 安装依赖
npm install

# 启动开发模式
npm run tauri dev

# 构建发布版本
npm run tauri build
```

## 项目结构

```
src/
├── lib/
│   ├── components/    # UI 组件
│   ├── stores/        # 状态管理
│   └── types/         # TypeScript 类型
├── App.svelte         # 主应用
└── app.css            # 全局样式

src-tauri/src/
├── main.rs            # Tauri 入口
├── commands.rs        # API 命令
├── models.rs          # 数据模型
├── storage.rs         # 存储管理
└── secrets.rs         # 密钥管理
```

## 数据存储位置

数据存储在系统应用数据目录：
- Windows: `%APPDATA%/api-key-storage/`
- macOS: `~/Library/Application Support/api-key-storage/`
- Linux: `~/.local/share/api-key-storage/`

## License

MIT