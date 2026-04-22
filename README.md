# API Key Storage

<p align="center">
  <img src="src-tauri/icons/icon.ico" alt="Logo" width="80">
</p>

<p align="center">
  <strong>本地 API 凭证管理工具</strong>
</p>

<p align="center">
  一款简洁、安全的桌面应用，用于管理多个平台的 API Key
</p>

---

## 功能特性

- **本地存储** - 数据存储在本地，不离开你的设备
- **多平台管理** - 支持管理 OpenAI、Anthropic、Azure 等多个平台的凭证
- **安全设计** - API Key 默认隐藏，点击显示
- **快速复制** - 一键复制 Base URL 和 API Key
- **搜索过滤** - 快速查找已保存的平台

## 技术栈

| 层级 | 技术 |
|------|------|
| 前端 | Svelte 5 + TypeScript |
| 后端 | Rust + Tauri 2 |
| 存储 | 本地 JSON 文件 |

## 快速开始

### 环境要求

- Node.js 18+
- Rust 1.70+
- Windows 10/11（目前仅支持 Windows）

### 安装与运行

```bash
# 克隆仓库
git clone https://github.com/your-username/api-key-storage.git
cd api-key-storage

# 安装依赖
npm install

# 开发模式
npm run tauri dev

# 构建发布版本
npm run tauri build
```

## 项目结构

```
api-key-storage/
├── src/                    # 前端源码
│   ├── lib/
│   │   ├── components/     # UI 组件
│   │   ├── stores/         # Svelte stores 状态管理
│   │   └── types/          # TypeScript 类型定义
│   ├── App.svelte          # 主应用组件
│   └── app.css             # 全局样式
│
├── src-tauri/              # 后端源码
│   ├── src/
│   │   ├── main.rs         # Tauri 入口
│   │   ├── commands.rs     # IPC 命令
│   │   ├── models.rs       # 数据模型
│   │   ├── storage.rs      # 存储管理
│   │   └── secrets.rs      # 密钥管理
│   └── tauri.conf.json     # Tauri 配置
│
└── package.json
```

## 数据存储

所有数据存储在本地用户目录：

| 平台 | 路径 |
|------|------|
| Windows | `%APPDATA%/api-key-storage/` |
| macOS | `~/Library/Application Support/api-key-storage/` |
| Linux | `~/.local/share/api-key-storage/` |

存储文件：
- `providers.json` - 平台元信息
- `secrets.json` - API Key（敏感数据）

## 安全说明

- API Key 仅存储在本地，不会上传到任何服务器
- 应用不包含任何网络请求代码
- 建议在生产环境中配合系统
