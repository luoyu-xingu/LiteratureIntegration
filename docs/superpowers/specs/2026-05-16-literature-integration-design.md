# LiteratureIntegration - 论文检索系统设计文档

## 概述

基于 Neo4j 图数据库的论文检索系统，支持工作区管理、论文自动导入、作者关系网络图可视化、关键词模糊搜索和 Markdown 批量导出。前端 TypeScript (React) + 后端 Rust (Axum)，单用户无认证模式。

## 架构

### 整体架构

```
┌─────────────────────────────────────────────┐
│              React SPA (TypeScript)          │
│  ┌──────────┬──────────┬──────────────────┐ │
│  │ 工作区管理 │ 论文管理  │ 作者关系网络图    │ │
│  │ 页面      │ 页面      │ (react-force-   │ │
│  │          │          │  graph)          │ │
│  └──────────┴──────────┴──────────────────┘ │
│  ┌──────────┬──────────────────────────────┐ │
│  │ 搜索页面  │ Markdown 导出               │ │
│  └──────────┴──────────────────────────────┘ │
└──────────────────┬──────────────────────────┘
                   │ REST API (JSON)
┌──────────────────▼──────────────────────────┐
│           Rust Axum Backend                  │
│  ┌──────────┬──────────┬──────────────────┐ │
│  │ 工作区路由 │ 论文路由  │ 搜索/导出路由    │ │
│  └──────────┴──────────┴──────────────────┘ │
│  ┌─────────────────────────────────────────┐ │
│  │ Service Layer (业务逻辑)                 │ │
│  └─────────────────────────────────────────┘ │
│  ┌─────────────────────────────────────────┐ │
│  │ Neo4j Repository (数据访问)              │ │
│  └─────────────────────────────────────────┘ │
│  ┌─────────────────────────────────────────┐ │
│  │ External API Client (DOI/arXiv 查询)    │ │
│  └─────────────────────────────────────────┘ │
└──────────────────┬──────────────────────────┘
                   │ Bolt Protocol
┌──────────────────▼──────────────────────────┐
│              Neo4j Database                  │
└─────────────────────────────────────────────┘
```

### 技术栈

**后端 (Rust):**

| 组件 | 选择 | 理由 |
|------|------|------|
| Web 框架 | Axum | 性能优秀，生态成熟 |
| Neo4j 驱动 | neo4rs | Rust 社区最活跃的 Neo4j 异步驱动 |
| 序列化 | serde + serde_json | Rust 标准选择 |
| HTTP 客户端 | reqwest | 用于调用外部学术 API |
| UUID | uuid | 生成节点 ID |
| 错误处理 | thiserror + anyhow | 分层错误处理 |
| 模板引擎 | tera | Markdown 导出渲染 |

**前端 (TypeScript):**

| 组件 | 选择 | 理由 |
|------|------|------|
| 框架 | React 18 + TypeScript | 需求指定 |
| 构建 | Vite | 快速开发体验 |
| 路由 | React Router v6 | SPA 路由管理 |
| HTTP | fetch + 自定义 hook | 轻量，无需额外依赖 |
| 网络图 | react-force-graph-2d | 基于 d3-force，专为关系图设计 |
| UI 组件 | Ant Design | 成熟的 React 组件库，中文友好 |
| Markdown 渲染 | react-markdown | 预览导出内容 |
| 状态管理 | React Context + useReducer | 项目规模适中 |

## Neo4j 数据模型

### 节点类型 (Labels)

| Label | 属性 | 说明 |
|-------|------|------|
| Workspace | id, name, description, created_at | 工作区 |
| Paper | id, title, doi, arxiv_id, abstract, user_notes, year, journal, created_at | 论文 |
| Author | id, name, orcid | 作者 |
| Keyword | id, name | 关键词 |

### 关系类型 (Relationships)

| 关系 | 方向 | 属性 | 说明 |
|------|------|------|------|
| CONTAINS | Workspace → Paper | added_at | 工作区包含论文 |
| FIRST_AUTHOR_OF | Author → Paper | — | 一作关系 |
| CORRESPONDING_AUTHOR_OF | Author → Paper | — | 通讯作者关系 |
| CO_AUTHOR_OF | Author ↔ Author | paper_count, workspace_id | 合著关系（一作/通讯之间） |
| HAS_KEYWORD | Paper → Keyword | — | 论文关键词 |

### 关键设计决策

- CO_AUTHOR_OF 是作者之间的边，记录两位作者共同发表的论文数量，是网络图的核心数据
- 一篇论文同时有一作和通讯作者时，自动在他们之间建立 CO_AUTHOR_OF 关系
- 关键词从学术 API 自动获取，用户也可以手动添加
- 工作区是论文的逻辑分组，同一篇论文可以属于多个工作区
- MERGE 语义确保节点和关系不重复

## 后端架构

### 目录结构

```
src/
├── main.rs                 # 启动入口，Axum 路由注册
├── config.rs               # 配置管理（Neo4j 连接、端口等）
├── routes/
│   ├── mod.rs
│   ├── workspace.rs        # 工作区 CRUD
│   ├── paper.rs            # 论文导入/管理
│   ├── author.rs           # 作者查询
│   ├── search.rs           # 模糊搜索
│   └── export.rs           # Markdown 导出
├── services/
│   ├── mod.rs
│   ├── workspace.rs        # 工作区业务逻辑
│   ├── paper.rs            # 论文业务逻辑（含导入流程）
│   ├── author.rs           # 作者关联逻辑
│   ├── search.rs           # 搜索逻辑
│   └── export.rs           # 导出逻辑
├── repositories/
│   ├── mod.rs
│   ├── neo4j_repo.rs       # Neo4j 查询封装
│   └── external_api.rs     # 外部学术 API 调用
├── models/
│   ├── mod.rs
│   ├── workspace.rs        # Workspace 结构体
│   ├── paper.rs            # Paper 结构体
│   ├── author.rs           # Author 结构体
│   ├── keyword.rs          # Keyword 结构体
│   └── dto.rs              # 请求/响应 DTO
└── errors.rs               # 统一错误类型
```

### REST API

**工作区管理：**

| 方法 | 路径 | 说明 |
|------|------|------|
| POST | /api/workspaces | 创建工作区 |
| GET | /api/workspaces | 列出所有工作区 |
| GET | /api/workspaces/:id | 获取工作区详情 |
| PUT | /api/workspaces/:id | 更新工作区 |
| DELETE | /api/workspaces/:id | 删除工作区 |

**论文管理：**

| 方法 | 路径 | 说明 |
|------|------|------|
| POST | /api/workspaces/:id/papers | 导入论文（传入 DOI 或 arXiv ID） |
| GET | /api/workspaces/:id/papers | 列出工作区内论文 |
| GET | /api/papers/:id | 获取论文详情 |
| PUT | /api/papers/:id | 更新论文（编辑笔记、关键词等） |
| DELETE | /api/papers/:id | 从工作区移除论文 |

**作者 & 关联：**

| 方法 | 路径 | 说明 |
|------|------|------|
| GET | /api/workspaces/:id/authors | 列出工作区内作者 |
| GET | /api/workspaces/:id/graph | 获取作者关系网络图数据 |
| GET | /api/authors/:id/papers | 获取作者的所有论文 |

**搜索：**

| 方法 | 路径 | 说明 |
|------|------|------|
| GET | /api/workspaces/:id/search?q=keyword | 关键词模糊搜索 |
| GET | /api/workspaces/:id/search?author=name | 按作者搜索 |

**导出：**

| 方法 | 路径 | 说明 |
|------|------|------|
| POST | /api/workspaces/:id/export | 批量导出 Markdown |

### 论文导入流程

```
用户输入 DOI/arXiv ID
        │
        ▼
  后端识别 ID 类型
        │
        ├── DOI → 调用 Crossref API / Semantic Scholar API
        │         获取: title, authors, abstract, year, journal, keywords
        │
        └── arXiv ID → 调用 arXiv API
                       获取: title, authors, abstract, year
        │
        ▼
  解析作者信息
  标记 first_author 和 corresponding_author
        │
        ▼
  写入 Neo4j
  ├── MERGE Paper 节点（避免重复）
  ├── MERGE Author 节点（按名字 + ORCID 去重）
  ├── 创建 FIRST_AUTHOR_OF / CORRESPONDING_AUTHOR_OF 关系
  ├── 创建 CONTAINS 关系（加入工作区）
  ├── 创建 HAS_KEYWORD 关系
  └── 在一作和通讯作者之间 MERGE CO_AUTHOR_OF 关系
        │
        ▼
  返回完整论文数据给前端
```

**通讯作者处理策略**：Crossref 和 arXiv API 通常不直接标注通讯作者。如果 API 返回了通讯作者信息则直接使用；如果没有，默认将 last author 标记为通讯作者（学术界惯例）；用户可以在导入后手动修改通讯作者标记。

## 前端架构

### 目录结构

```
frontend/
├── src/
│   ├── main.tsx
│   ├── App.tsx
│   ├── api/
│   │   ├── client.ts          # HTTP 客户端封装
│   │   ├── workspace.ts       # 工作区 API
│   │   ├── paper.ts           # 论文 API
│   │   ├── author.ts          # 作者 API
│   │   ├── search.ts          # 搜索 API
│   │   └── export.ts          # 导出 API
│   ├── components/
│   │   ├── Layout.tsx          # 全局布局（侧边栏 + 内容区）
│   │   ├── WorkspaceList.tsx   # 工作区列表
│   │   ├── WorkspaceForm.tsx   # 创建/编辑工作区表单
│   │   ├── PaperList.tsx       # 论文列表
│   │   ├── PaperDetail.tsx     # 论文详情
│   │   ├── PaperImport.tsx     # 论文导入（输入 DOI/arXiv ID）
│   │   ├── PaperNotes.tsx      # 论文笔记编辑
│   │   ├── AuthorGraph.tsx     # 作者关系网络图
│   │   ├── SearchBar.tsx       # 搜索栏
│   │   ├── SearchResult.tsx    # 搜索结果
│   │   └── ExportPanel.tsx     # 导出面板
│   ├── pages/
│   │   ├── WorkspacesPage.tsx  # 工作区管理页
│   │   ├── WorkspaceDetail.tsx # 工作区详情（论文列表 + 网络图）
│   │   ├── PaperPage.tsx       # 论文详情页
│   │   └── SearchPage.tsx      # 搜索页
│   ├── hooks/
│   │   ├── useWorkspaces.ts
│   │   ├── usePapers.ts
│   │   ├── useGraph.ts
│   │   └── useSearch.ts
│   ├── types/
│   │   └── index.ts           # TypeScript 类型定义
│   └── styles/
│       └── global.css
├── index.html
├── vite.config.ts
├── tsconfig.json
└── package.json
```

### 页面布局

全局布局为左侧边栏 + 右侧内容区。侧边栏显示工作区列表，内容区根据路由显示不同页面。

### 核心页面

1. **工作区详情页**：双 Tab 切换——论文列表视图和作者网络图视图
2. **论文导入**：模态框输入 DOI 或 arXiv ID，导入后自动填充信息，用户可补充笔记
3. **论文详情**：展示标题、年份、期刊、DOI、一作/通讯作者、abstract、用户笔记、关键词，支持编辑
4. **搜索页**：支持关键词搜索和作者搜索两种模式，搜索范围限定在当前工作区
5. **导出面板**：支持按作者或关键词分组，可筛选年份/作者/关键词，可选择包含内容，预览后下载

### 作者网络图交互

- 节点大小：按论文数量缩放，论文越多节点越大
- 节点颜色：一作用实心圆，通讯作者用空心圆，两者兼有用双圈
- 边粗细：按合著论文数量缩放
- 悬停节点：显示作者名称、论文数量
- 点击节点：侧边面板显示该作者的所有论文
- 点击边：显示两位作者合著的论文列表
- 支持缩放和拖拽

## 搜索功能

### 关键词模糊搜索

搜索目标：论文标题、关键词、abstract。使用 Neo4j Cypher 的 CONTAINS 匹配。

```cypher
MATCH (w:Workspace {id: $workspace_id})-[:CONTAINS]->(p:Paper)
WHERE p.title CONTAINS $query
   OR p.abstract CONTAINS $query
   OR EXISTS {
       MATCH (p)-[:HAS_KEYWORD]->(k:Keyword)
       WHERE k.name CONTAINS $query
   }
RETURN p
ORDER BY p.year DESC
```

### 按作者搜索

搜索目标：作者姓名模糊匹配，返回匹配作者及其在该工作区内的所有论文。

```cypher
MATCH (w:Workspace {id: $workspace_id})-[:CONTAINS]->(p:Paper)<-[:FIRST_AUTHOR_OF|CORRESPONDING_AUTHOR_OF]-(a:Author)
WHERE a.name CONTAINS $author_name
RETURN a, collect(p) AS papers
ORDER BY size(papers) DESC
```

## Markdown 导出

### 导出请求格式

```json
POST /api/workspaces/:id/export
{
  "format": "markdown",
  "group_by": "author",
  "filter": {
    "author_ids": [],
    "keyword_ids": [],
    "year_range": [2018, 2024]
  }
}
```

group_by 支持 "author"（按作者分组）和 "keyword"（按关键词分组）。

### 导出 Markdown 模板

```markdown
# 工作区: {workspace_name}

> 导出时间: {export_date}
> 论文数量: {paper_count}

---

## 作者: {author_name}

### {paper_title}
- **年份**: {year} | **期刊**: {journal}
- **DOI**: {doi}
- **一作**: {first_author} | **通讯**: {corresponding_author}
- **关键词**: {keywords}

**Abstract:**
{abstract}

**笔记:**
{user_notes}

---
```

### 导出实现

- Rust 端根据筛选条件从 Neo4j 查询数据
- 按分组方式组织数据结构
- 使用 tera 模板引擎渲染 Markdown 字符串
- 返回 Content-Type: text/markdown，前端触发下载

## 错误处理

### 后端错误响应格式

```json
{
  "error": {
    "code": "PAPER_NOT_FOUND",
    "message": "Paper with id xxx not found"
  }
}
```

### 错误码

| 错误码 | HTTP 状态码 | 说明 |
|--------|------------|------|
| WORKSPACE_NOT_FOUND | 404 | 工作区不存在 |
| PAPER_NOT_FOUND | 404 | 论文不存在 |
| AUTHOR_NOT_FOUND | 404 | 作者不存在 |
| IMPORT_FAILED | 422 | 论文导入失败（DOI/arXiv ID 无效或 API 不可用） |
| NEO4J_ERROR | 500 | 数据库操作失败 |
| VALIDATION_ERROR | 400 | 请求参数验证失败 |

## 配置

通过环境变量或 .env 文件配置：

```
NEO4J_URI=bolt://localhost:7687
NEO4J_USER=neo4j
NEO4J_PASSWORD=password
SERVER_HOST=0.0.0.0
SERVER_PORT=3000
CORS_ORIGIN=http://localhost:5173
```
