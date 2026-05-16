# Literature Integration Desktop App — Design Spec

## Overview

将现有的 Web 论文检索系统（Axum + React + Neo4j）重构为 Tauri v2 桌面应用（exe），使用纯文件系统存储，每个工作区对应一个文件夹，论文以 Markdown + YAML Frontmatter 格式存储，支持工作区内模糊查询。

## Architecture

```
Tauri v2 Desktop App
├── React Frontend (TypeScript) ←→ Rust Backend (Tauri Commands)
│                                    ├── 文件系统读写
│                                    ├── YAML/Markdown 解析
│                                    ├── 模糊搜索
│                                    ├── Crossref/arXiv API
│                                    └── Markdown 导出
└── 本地文件系统 (根目录)
    ├── _index.yaml
    ├── 工作区A/
    │   ├── _workspace.yaml
    │   └── paper.md
    └── 工作区B/
```

去掉 Neo4j，去掉 Axum HTTP 服务，所有数据操作通过 Tauri Commands 在 Rust 侧直接完成。

## File System Layout

```
~/LiteratureIntegration/              ← 根目录（首次启动时用户选择）
├── _index.yaml                       ← 全局索引
├── 深度学习论文集/                    ← 工作区 = 文件夹名
│   ├── _workspace.yaml               ← 工作区元数据
│   ├── attention-is-all-you-need.md   ← 论文文件
│   └── ...
└── 自然语言处理/
    ├── _workspace.yaml
    └── ...
```

### _index.yaml

```yaml
version: 1
root: "/Users/xxx/LiteratureIntegration"
workspaces:
  - id: "ef40c39c-..."
    name: "深度学习论文集"
    path: "深度学习论文集"
  - id: "abc123-..."
    name: "自然语言处理"
    path: "自然语言处理"
```

### _workspace.yaml

```yaml
id: "ef40c39c-..."
name: "深度学习论文集"
description: "深度学习相关论文收集"
created_at: "2025-01-01T00:00:00Z"
```

### Paper Markdown File

```markdown
---
id: "paper-uuid"
title: "Attention Is All You Need"
doi: "10.5555/3295222.3295349"
arxiv_id: "1706.03762"
year: 2017
journal: "NeurIPS"
first_author: "Ashish Vaswani"
corresponding_author: "Noam Shazeer"
keywords: [transformer, attention, deep-learning]
created_at: "2025-01-01T00:00:00Z"
---

## Abstract

We propose a new simple network architecture, the Transformer...

## 我的笔记

### 核心贡献
- 提出了 Self-Attention 机制
```

论文文件名由标题生成：小写、空格替换为连字符、去除特殊字符、截断至 80 字符。

## Tauri Commands

| Command | 功能 |
|---------|------|
| `list_workspaces` | 列出所有工作区 |
| `create_workspace(name, desc)` | 创建工作区文件夹 + _workspace.yaml |
| `get_workspace(id)` | 读取 _workspace.yaml |
| `update_workspace(id, name?, desc?)` | 更新 _workspace.yaml，可重命名文件夹 |
| `delete_workspace(id)` | 删除整个工作区文件夹 |
| `list_papers(workspace_id)` | 遍历工作区内 .md 文件（排除 _ 开头） |
| `import_paper(workspace_id, identifier)` | 通过 DOI/arXiv 获取元数据，写入 .md 文件 |
| `get_paper(id)` | 解析 .md 文件返回 frontmatter + 正文 |
| `update_paper(id, notes)` | 更新 .md 文件的笔记部分 |
| `delete_paper(workspace_id, paper_id)` | 删除 .md 文件 |
| `search(workspace_id, query, mode)` | 模糊搜索（keyword/author/content） |
| `export_workspace(workspace_id, group_by)` | 导出为 Markdown 文件 |
| `select_root_dir()` | 打开文件夹选择对话框 |
| `get_authors(workspace_id)` | 从所有论文中提取作者列表 |
| `get_graph(workspace_id)` | 生成作者关系图数据 |

## Fuzzy Search

遍历工作区内所有 .md 文件，解析 frontmatter + 正文，多字段模糊匹配：

- **keyword 模式**：匹配 title、keywords、abstract 正文
- **author 模式**：匹配 first_author、corresponding_author
- **content 模式**：匹配全文（frontmatter + 正文）

匹配算法：大小写不敏感子串匹配，空格分隔多关键词 AND 查询。

## Project Structure

```
/workspace/literature-app/
├── src-tauri/
│   ├── Cargo.toml
│   ├── tauri.conf.json
│   ├── src/
│   │   ├── main.rs
│   │   ├── lib.rs
│   │   ├── commands/
│   │   │   ├── mod.rs
│   │   │   ├── workspace.rs
│   │   │   ├── paper.rs
│   │   │   ├── search.rs
│   │   │   ├── author.rs
│   │   │   └── export.rs
│   │   ├── storage/
│   │   │   ├── mod.rs
│   │   │   ├── index.rs
│   │   │   ├── workspace.rs
│   │   │   └── paper.rs
│   │   ├── models/
│   │   │   ├── mod.rs
│   │   │   ├── workspace.rs
│   │   │   ├── paper.rs
│   │   │   └── author.rs
│   │   └── external/
│   │       ├── mod.rs
│   │       ├── crossref.rs
│   │       └── arxiv.rs
├── src/                             ← React 前端
│   ├── App.tsx
│   ├── main.tsx
│   ├── components/
│   ├── pages/
│   ├── hooks/
│   ├── api/                         ← invoke() 调用
│   ├── types/
│   └── styles/
├── package.json
├── vite.config.ts
└── tsconfig.json
```

## Frontend Changes

- 所有 api/*.ts 从 fetch() 改为 invoke()（Tauri IPC）
- 去掉 api/client.ts HTTP 客户端
- 新增根目录选择界面（首次启动 / 设置页）
- 搜索页面增加「内容搜索」模式
- 保留现有 UI 设计（Scholar's Atelier 深色主题）

## Key Dependencies (Rust)

- tauri 2.x
- serde + serde_yaml（YAML 读写）
- gray_matter（Markdown frontmatter 解析）
- reqwest（Crossref/arXiv API）
- uuid（ID 生成）
- anyhow（错误处理）
- tokio（异步运行时）

## Key Dependencies (Frontend)

- @tauri-apps/api 2.x（invoke 调用）
- react + react-router-dom
- antd（UI 组件）
- react-force-graph-2d（图谱可视化）
- react-markdown（Markdown 渲染）
