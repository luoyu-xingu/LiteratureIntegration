# Literature Integration Desktop App — Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 将 Web 论文检索系统重构为 Tauri v2 桌面应用，使用纯文件系统存储，支持工作区内模糊查询。

**Architecture:** Tauri v2 桌面应用，Rust 侧通过 Tauri Commands 直接读写本地文件系统（YAML + Markdown Frontmatter），前端通过 invoke() IPC 调用。去掉 Neo4j 和 Axum HTTP 服务。

**Tech Stack:** Tauri v2, Rust, React TypeScript, Ant Design, YAML/Markdown 文件系统

---

## File Structure

### Rust Backend (src-tauri/src/)

| File | Responsibility |
|------|---------------|
| `main.rs` | Tauri 入口，构建 app |
| `lib.rs` | 模块注册，注册所有 commands |
| `models/mod.rs` | 模块导出 |
| `models/workspace.rs` | Workspace 数据结构 + serde |
| `models/paper.rs` | Paper 数据结构 + serde |
| `models/author.rs` | Author/GraphNode/GraphLink 数据结构 |
| `storage/mod.rs` | 模块导出 + AppState |
| `storage/index.rs` | _index.yaml 读写 |
| `storage/workspace.rs` | 工作区文件夹 CRUD |
| `storage/paper.rs` | 论文 .md 文件 CRUD |
| `commands/mod.rs` | 模块导出 |
| `commands/workspace.rs` | 工作区 Tauri commands |
| `commands/paper.rs` | 论文 Tauri commands |
| `commands/search.rs` | 模糊搜索 command |
| `commands/author.rs` | 作者/图谱 commands |
| `commands/export.rs` | 导出 command |
| `commands/app.rs` | 根目录选择等 app 级 commands |
| `external/mod.rs` | 模块导出 |
| `external/crossref.rs` | Crossref API 客户端 |
| `external/arxiv.rs` | arXiv API 客户端 |

### React Frontend (src/)

| File | Responsibility |
|------|---------------|
| `api/client.ts` | Tauri invoke 封装 |
| `api/workspace.ts` | 工作区 API |
| `api/paper.ts` | 论文 API |
| `api/author.ts` | 作者 API |
| `api/search.ts` | 搜索 API |
| `api/export.ts` | 导出 API |
| `api/app.ts` | 根目录选择 API |
| `components/RootSelector.tsx` | 首次启动根目录选择 |
| `components/Layout.tsx` | 主布局（从现有迁移） |
| `components/WorkspaceList.tsx` | 工作区列表（从现有迁移） |
| `components/WorkspaceForm.tsx` | 新建工作区表单（从现有迁移） |
| `components/PaperList.tsx` | 论文列表（从现有迁移） |
| `components/PaperImport.tsx` | 导入论文（从现有迁移） |
| `components/PaperDetail.tsx` | 论文详情（从现有迁移） |
| `components/PaperNotes.tsx` | 笔记编辑（从现有迁移） |
| `components/AuthorGraph.tsx` | 作者关系图（从现有迁移） |
| `components/SearchBar.tsx` | 搜索栏（增加内容搜索模式） |
| `components/SearchResult.tsx` | 搜索结果（从现有迁移） |
| `components/ExportPanel.tsx` | 导出面板（从现有迁移） |
| `pages/WorkspacesPage.tsx` | 工作区页面 |
| `pages/WorkspaceDetail.tsx` | 工作区详情页面 |
| `pages/PaperPage.tsx` | 论文详情页面 |
| `pages/SearchPage.tsx` | 搜索页面 |
| `hooks/useWorkspaces.ts` | 工作区 hook |
| `hooks/usePapers.ts` | 论文 hook |
| `hooks/useSearch.ts` | 搜索 hook |
| `hooks/useGraph.ts` | 图谱 hook |
| `types/index.ts` | 类型定义 |
| `styles/global.css` | 全局样式（Scholar's Atelier 主题） |

---

### Task 1: 初始化 Tauri v2 项目

**Files:**
- Create: `/workspace/literature-app/` 整个项目目录

- [ ] **Step 1: 安装 Tauri CLI 并创建项目**

```bash
cd /workspace
npm create tauri-app@latest literature-app -- --template react-ts
```

选择 React + TypeScript 模板。

- [ ] **Step 2: 安装前端依赖**

```bash
cd /workspace/literature-app
npm install antd @ant-design/icons react-router-dom react-force-graph-2d react-markdown
```

- [ ] **Step 3: 安装 Rust 依赖**

编辑 `/workspace/literature-app/src-tauri/Cargo.toml`，添加：

```toml
[dependencies]
tauri = { version = "2", features = [] }
tauri-plugin-dialog = "2"
tauri-plugin-fs = "2"
serde = { version = "1", features = ["derive"] }
serde_yaml = "0.9"
serde_json = "1"
gray_matter = "0.2"
reqwest = { version = "0.12", features = ["json"] }
uuid = { version = "1", features = ["v4"] }
anyhow = "1"
tokio = { version = "1", features = ["fs", "io-util"] }
chrono = { version = "0.4", features = ["serde"] }
```

- [ ] **Step 4: 配置 tauri.conf.json**

编辑 `/workspace/literature-app/src-tauri/tauri.conf.json`，确保：

```json
{
  "identifier": "com.literature-integration.app",
  "plugins": {
    "fs": {
      "scope": ["**"]
    },
    "dialog": {
      "open": true,
      "save": true
    }
  }
}
```

- [ ] **Step 5: 验证项目可构建**

```bash
cd /workspace/literature-app
npx tauri build --debug 2>&1 | head -20
```

如果构建失败，修复依赖问题。如果构建成功，确认 `src-tauri/target/debug/` 下生成了可执行文件。

- [ ] **Step 6: Commit**

```bash
git add -A
git commit -m "feat: initialize Tauri v2 project with dependencies"
```

---

### Task 2: Rust 数据模型

**Files:**
- Create: `/workspace/literature-app/src-tauri/src/models/mod.rs`
- Create: `/workspace/literature-app/src-tauri/src/models/workspace.rs`
- Create: `/workspace/literature-app/src-tauri/src/models/paper.rs`
- Create: `/workspace/literature-app/src-tauri/src/models/author.rs`

- [ ] **Step 1: 创建 models/mod.rs**

```rust
pub mod workspace;
pub mod paper;
pub mod author;
```

- [ ] **Step 2: 创建 models/workspace.rs**

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workspace {
    pub id: String,
    pub name: String,
    pub description: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceIndexEntry {
    pub id: String,
    pub name: String,
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RootIndex {
    pub version: i32,
    pub root: String,
    pub workspaces: Vec<WorkspaceIndexEntry>,
}

#[derive(Debug, Deserialize)]
pub struct CreateWorkspaceRequest {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateWorkspaceRequest {
    pub name: Option<String>,
    pub description: Option<String>,
}
```

- [ ] **Step 3: 创建 models/paper.rs**

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Paper {
    pub id: String,
    pub title: String,
    pub doi: Option<String>,
    pub arxiv_id: Option<String>,
    pub year: Option<i32>,
    pub journal: Option<String>,
    pub first_author: Option<String>,
    pub corresponding_author: Option<String>,
    pub keywords: Vec<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaperFile {
    pub frontmatter: Paper,
    pub abstract_text: Option<String>,
    pub user_notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaperDetailResponse {
    pub paper: Paper,
    pub abstract_text: Option<String>,
    pub user_notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ImportPaperRequest {
    pub identifier: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePaperRequest {
    pub user_notes: Option<String>,
}
```

- [ ] **Step 4: 创建 models/author.rs**

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Author {
    pub name: String,
    pub first_author_count: i32,
    pub corresponding_author_count: i32,
    pub paper_count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphNode {
    pub id: String,
    pub name: String,
    pub paper_count: i32,
    pub author_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphLink {
    pub source: String,
    pub target: String,
    pub paper_count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphDataResponse {
    pub nodes: Vec<GraphNode>,
    pub links: Vec<GraphLink>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorWithPapers {
    pub author_name: String,
    pub papers: Vec<crate::models::paper::Paper>,
}
```

- [ ] **Step 5: 验证编译**

```bash
cd /workspace/literature-app/src-tauri && cargo check 2>&1
```

修复任何编译错误。

- [ ] **Step 6: Commit**

```bash
git add -A
git commit -m "feat: add Rust data models for workspace, paper, author"
```

---

### Task 3: 文件系统存储层 — Index

**Files:**
- Create: `/workspace/literature-app/src-tauri/src/storage/mod.rs`
- Create: `/workspace/literature-app/src-tauri/src/storage/index.rs`

- [ ] **Step 1: 创建 storage/mod.rs**

```rust
pub mod index;
pub mod workspace;
pub mod paper;

use std::path::PathBuf;
use std::sync::Mutex;

pub struct AppState {
    pub root_dir: Mutex<Option<PathBuf>>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            root_dir: Mutex::new(None),
        }
    }
}
```

- [ ] **Step 2: 创建 storage/index.rs**

```rust
use crate::models::workspace::{RootIndex, WorkspaceIndexEntry};
use crate::storage::AppState;
use std::path::PathBuf;
use anyhow::{Context, Result};

pub fn get_index_path(root: &PathBuf) -> PathBuf {
    root.join("_index.yaml")
}

pub fn read_index(root: &PathBuf) -> Result<RootIndex> {
    let path = get_index_path(root);
    if !path.exists() {
        return Ok(RootIndex {
            version: 1,
            root: root.to_string_lossy().to_string(),
            workspaces: vec![],
        });
    }
    let content = std::fs::read_to_string(&path)
        .context("Failed to read _index.yaml")?;
    let index: RootIndex = serde_yaml::from_str(&content)
        .context("Failed to parse _index.yaml")?;
    Ok(index)
}

pub fn write_index(root: &PathBuf, index: &RootIndex) -> Result<()> {
    let path = get_index_path(root);
    let content = serde_yaml::to_string(index)
        .context("Failed to serialize _index.yaml")?;
    std::fs::write(&path, content)
        .context("Failed to write _index.yaml")?;
    Ok(())
}

pub fn ensure_index(root: &PathBuf) -> Result<RootIndex> {
    let index = read_index(root)?;
    if index.version == 0 {
        let new_index = RootIndex {
            version: 1,
            root: root.to_string_lossy().to_string(),
            workspaces: vec![],
        };
        write_index(root, &new_index)?;
        return Ok(new_index);
    }
    Ok(index)
}

pub fn get_root_dir(state: &AppState) -> Result<PathBuf> {
    let guard = state.root_dir.lock().unwrap();
    guard.clone().context("Root directory not set. Please select a root directory first.")
}

pub fn add_workspace_to_index(root: &PathBuf, entry: WorkspaceIndexEntry) -> Result<()> {
    let mut index = read_index(root)?;
    index.workspaces.push(entry);
    write_index(root, &index)
}

pub fn remove_workspace_from_index(root: &PathBuf, workspace_id: &str) -> Result<()> {
    let mut index = read_index(root)?;
    index.workspaces.retain(|w| w.id != workspace_id);
    write_index(root, &index)
}

pub fn update_workspace_in_index(root: &PathBuf, workspace_id: &str, new_name: &str, new_path: &str) -> Result<()> {
    let mut index = read_index(root)?;
    for ws in &mut index.workspaces {
        if ws.id == workspace_id {
            ws.name = new_name.to_string();
            ws.path = new_path.to_string();
        }
    }
    write_index(root, &index)
}
```

- [ ] **Step 3: 验证编译**

```bash
cd /workspace/literature-app/src-tauri && cargo check 2>&1
```

- [ ] **Step 4: Commit**

```bash
git add -A
git commit -m "feat: add file system storage layer — index read/write"
```

---

### Task 4: 文件系统存储层 — Workspace

**Files:**
- Create: `/workspace/literature-app/src-tauri/src/storage/workspace.rs`

- [ ] **Step 1: 创建 storage/workspace.rs**

```rust
use crate::models::workspace::{Workspace, WorkspaceIndexEntry};
use crate::storage::index::{read_index, add_workspace_to_index, remove_workspace_from_index, update_workspace_in_index};
use anyhow::{Context, Result};
use std::path::PathBuf;

pub fn get_workspace_dir(root: &PathBuf, workspace_path: &str) -> PathBuf {
    root.join(workspace_path)
}

pub fn get_workspace_yaml(root: &PathBuf, workspace_path: &str) -> PathBuf {
    root.join(workspace_path).join("_workspace.yaml")
}

pub fn create_workspace(root: &PathBuf, name: &str, description: &str) -> Result<Workspace> {
    let id = uuid::Uuid::new_v4().to_string();
    let created_at = chrono::Utc::now().to_rfc3339();
    let path = sanitize_folder_name(name);

    let dir = get_workspace_dir(root, &path);
    std::fs::create_dir_all(&dir)
        .context("Failed to create workspace directory")?;

    let workspace = Workspace {
        id: id.clone(),
        name: name.to_string(),
        description: description.to_string(),
        created_at: created_at.clone(),
    };

    let yaml_path = get_workspace_yaml(root, &path);
    let content = serde_yaml::to_string(&workspace)
        .context("Failed to serialize workspace")?;
    std::fs::write(&yaml_path, content)
        .context("Failed to write _workspace.yaml")?;

    let entry = WorkspaceIndexEntry {
        id: id.clone(),
        name: name.to_string(),
        path: path.clone(),
    };
    add_workspace_to_index(root, entry)?;

    Ok(workspace)
}

pub fn list_workspaces(root: &PathBuf) -> Result<Vec<Workspace>> {
    let index = read_index(root)?;
    let mut workspaces = Vec::new();
    for entry in &index.workspaces {
        let yaml_path = get_workspace_yaml(root, &entry.path);
        if yaml_path.exists() {
            let content = std::fs::read_to_string(&yaml_path)?;
            let ws: Workspace = serde_yaml::from_str(&content)?;
            workspaces.push(ws);
        }
    }
    Ok(workspaces)
}

pub fn get_workspace(root: &PathBuf, workspace_id: &str) -> Result<Workspace> {
    let index = read_index(root)?;
    let entry = index.workspaces.iter()
        .find(|w| w.id == workspace_id)
        .context(format!("Workspace not found: {}", workspace_id))?;
    let yaml_path = get_workspace_yaml(root, &entry.path);
    let content = std::fs::read_to_string(&yaml_path)?;
    let ws: Workspace = serde_yaml::from_str(&content)?;
    Ok(ws)
}

pub fn update_workspace(root: &PathBuf, workspace_id: &str, name: Option<String>, description: Option<String>) -> Result<Workspace> {
    let index = read_index(root)?;
    let entry = index.workspaces.iter()
        .find(|w| w.id == workspace_id)
        .context(format!("Workspace not found: {}", workspace_id))?;

    let yaml_path = get_workspace_yaml(root, &entry.path);
    let content = std::fs::read_to_string(&yaml_path)?;
    let mut ws: Workspace = serde_yaml::from_str(&content)?;

    let old_path = entry.path.clone();
    let mut new_path = old_path.clone();

    if let Some(ref n) = name {
        ws.name = n.clone();
        new_path = sanitize_folder_name(n);
    }
    if let Some(ref d) = description {
        ws.description = d.clone();
    }

    if new_path != old_path {
        let old_dir = root.join(&old_path);
        let new_dir = root.join(&new_path);
        std::fs::rename(&old_dir, &new_dir)
            .context("Failed to rename workspace directory")?;
        update_workspace_in_index(root, workspace_id, &ws.name, &new_path)?;
    } else {
        let updated_content = serde_yaml::to_string(&ws)?;
        std::fs::write(&yaml_path, updated_content)?;
        if name.is_some() {
            update_workspace_in_index(root, workspace_id, &ws.name, &new_path)?;
        }
    }

    Ok(ws)
}

pub fn delete_workspace(root: &PathBuf, workspace_id: &str) -> Result<()> {
    let index = read_index(root)?;
    let entry = index.workspaces.iter()
        .find(|w| w.id == workspace_id)
        .context(format!("Workspace not found: {}", workspace_id))?;

    let dir = root.join(&entry.path);
    if dir.exists() {
        std::fs::remove_dir_all(&dir)
            .context("Failed to delete workspace directory")?;
    }
    remove_workspace_from_index(root, workspace_id)
}

pub fn get_workspace_path_by_id(root: &PathBuf, workspace_id: &str) -> Result<String> {
    let index = read_index(root)?;
    let entry = index.workspaces.iter()
        .find(|w| w.id == workspace_id)
        .context(format!("Workspace not found: {}", workspace_id))?;
    Ok(entry.path.clone())
}

fn sanitize_folder_name(name: &str) -> String {
    let sanitized: String = name
        .chars()
        .map(|c| if c.is_alphanumeric() || c == '-' || c == '_' { c } else if c == ' ' { '-' } else { '-' })
        .collect();
    let trimmed = sanitized.trim_matches('-');
    if trimmed.is_empty() { "unnamed-workspace".to_string() } else { trimmed.to_string() }
}
```

- [ ] **Step 2: 验证编译**

```bash
cd /workspace/literature-app/src-tauri && cargo check 2>&1
```

- [ ] **Step 3: Commit**

```bash
git add -A
git commit -m "feat: add workspace storage layer — CRUD on file system"
```

---

### Task 5: 文件系统存储层 — Paper

**Files:**
- Create: `/workspace/literature-app/src-tauri/src/storage/paper.rs`

- [ ] **Step 1: 创建 storage/paper.rs**

```rust
use crate::models::paper::{Paper, PaperFile, PaperDetailResponse};
use crate::storage::workspace::get_workspace_path_by_id;
use anyhow::{Context, Result};
use std::path::PathBuf;

pub fn create_paper_file(root: &PathBuf, workspace_path: &str, paper: &Paper, abstract_text: Option<&str>, user_notes: Option<&str>) -> Result<()> {
    let filename = sanitize_filename(&paper.title);
    let dir = root.join(workspace_path);
    let filepath = dir.join(format!("{}.md", filename));

    let mut content = String::new();
    let frontmatter = serde_yaml::to_string(paper)
        .context("Failed to serialize paper frontmatter")?;
    content.push_str("---\n");
    content.push_str(&frontmatter);
    content.push_str("---\n\n");

    if let Some(abs) = abstract_text {
        content.push_str("## Abstract\n\n");
        content.push_str(abs);
        content.push_str("\n\n");
    }

    if let Some(notes) = user_notes {
        content.push_str("## 我的笔记\n\n");
        content.push_str(notes);
        content.push_str("\n");
    }

    std::fs::write(&filepath, content)
        .context("Failed to write paper file")?;
    Ok(())
}

pub fn list_papers(root: &PathBuf, workspace_path: &str) -> Result<Vec<Paper>> {
    let dir = root.join(workspace_path);
    if !dir.exists() {
        return Ok(vec![]);
    }

    let mut papers = Vec::new();
    for entry in std::fs::read_dir(&dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().map_or(false, |e| e == "md") {
            let filename = path.file_name().unwrap().to_string_lossy().to_string();
            if filename.starts_with('_') {
                continue;
            }
            if let Ok(pf) = parse_paper_file(&path) {
                papers.push(pf.frontmatter);
            }
        }
    }
    papers.sort_by(|a, b| b.created_at.cmp(&a.created_at));
    Ok(papers)
}

pub fn get_paper_detail(root: &PathBuf, paper_id: &str) -> Result<PaperDetailResponse> {
    let (path, pf) = find_paper_by_id(root, paper_id)?;
    Ok(PaperDetailResponse {
        paper: pf.frontmatter,
        abstract_text: pf.abstract_text,
        user_notes: pf.user_notes,
    })
}

pub fn update_paper_notes(root: &PathBuf, paper_id: &str, new_notes: &str) -> Result<PaperDetailResponse> {
    let (path, mut pf) = find_paper_by_id(root, paper_id)?;
    pf.user_notes = Some(new_notes.to_string());
    write_paper_file(&path, &pf)?;
    Ok(PaperDetailResponse {
        paper: pf.frontmatter,
        abstract_text: pf.abstract_text,
        user_notes: pf.user_notes,
    })
}

pub fn delete_paper_file(root: &PathBuf, workspace_path: &str, paper_id: &str) -> Result<()> {
    let (path, _) = find_paper_by_id(root, paper_id)?;
    std::fs::remove_file(&path)
        .context("Failed to delete paper file")?;
    Ok(())
}

pub fn find_paper_by_id(root: &PathBuf, paper_id: &str) -> Result<(PathBuf, PaperFile)> {
    let index = crate::storage::index::read_index(root)?;
    for ws_entry in &index.workspaces {
        let dir = root.join(&ws_entry.path);
        if !dir.exists() {
            continue;
        }
        for entry in std::fs::read_dir(&dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().map_or(false, |e| e == "md") {
                let filename = path.file_name().unwrap().to_string_lossy().to_string();
                if filename.starts_with('_') {
                    continue;
                }
                if let Ok(pf) = parse_paper_file(&path) {
                    if pf.frontmatter.id == paper_id {
                        return Ok((path, pf));
                    }
                }
            }
        }
    }
    anyhow::bail!("Paper not found: {}", paper_id)
}

fn parse_paper_file(path: &PathBuf) -> Result<PaperFile> {
    let content = std::fs::read_to_string(path)
        .context("Failed to read paper file")?;

    let parsed = gray_matter::Matter::<gray_matter::engine::YAML>::new()
        .parse(&content);

    let paper: Paper = serde_yaml::from_str(&parsed.matter)
        .context("Failed to parse paper frontmatter")?;

    let body = parsed.content.trim();

    let (abstract_text, user_notes) = split_body(body);

    Ok(PaperFile {
        frontmatter: paper,
        abstract_text,
        user_notes,
    })
}

fn split_body(body: &str) -> (Option<String>, Option<String>) {
    let mut abstract_text = None;
    let mut user_notes = None;

    if let Some(abs_start) = body.find("## Abstract") {
        let after_abs = &body[abs_start + "## Abstract".len()..];
        let abs_end = after_abs.find("## 我的笔记").unwrap_or(after_abs.len());
        let abs_content = after_abs[..abs_end].trim();
        if !abs_content.is_empty() {
            abstract_text = Some(abs_content.to_string());
        }
    }

    if let Some(notes_start) = body.find("## 我的笔记") {
        let after_notes = &body[notes_start + "## 我的笔记".len()..];
        let notes_content = after_notes.trim();
        if !notes_content.is_empty() {
            user_notes = Some(notes_content.to_string());
        }
    }

    (abstract_text, user_notes)
}

fn write_paper_file(path: &PathBuf, pf: &PaperFile) -> Result<()> {
    let mut content = String::new();
    let frontmatter = serde_yaml::to_string(&pf.frontmatter)?;
    content.push_str("---\n");
    content.push_str(&frontmatter);
    content.push_str("---\n\n");

    if let Some(ref abs) = pf.abstract_text {
        content.push_str("## Abstract\n\n");
        content.push_str(abs);
        content.push_str("\n\n");
    }

    if let Some(ref notes) = pf.user_notes {
        content.push_str("## 我的笔记\n\n");
        content.push_str(notes);
        content.push_str("\n");
    }

    std::fs::write(path, content)?;
    Ok(())
}

pub fn sanitize_filename(title: &str) -> String {
    let sanitized: String = title
        .to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() || c == '-' || c == '_' { c } else if c == ' ' { '-' } else { '-' })
        .collect();
    let result: String = sanitized
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-");
    if result.len() > 80 { result[..80].trim_end_matches('-').to_string() } else { result }
}
```

- [ ] **Step 2: 验证编译**

```bash
cd /workspace/literature-app/src-tauri && cargo check 2>&1
```

- [ ] **Step 3: Commit**

```bash
git add -A
git commit -m "feat: add paper storage layer — read/write Markdown files"
```

---

### Task 6: 外部 API 客户端 — Crossref + arXiv

**Files:**
- Create: `/workspace/literature-app/src-tauri/src/external/mod.rs`
- Create: `/workspace/literature-app/src-tauri/src/external/crossref.rs`
- Create: `/workspace/literature-app/src-tauri/src/external/arxiv.rs`

- [ ] **Step 1: 创建 external/mod.rs**

```rust
pub mod crossref;
pub mod arxiv;

use crate::models::paper::Paper;

pub struct ImportResult {
    pub paper: Paper,
    pub abstract_text: Option<String>,
}

pub async fn import_by_identifier(identifier: &str) -> anyhow::Result<ImportResult> {
    let identifier = identifier.trim();
    if identifier.contains('/') && identifier.contains('10.') {
        crossref::fetch_by_doi(identifier).await
    } else if identifier.starts_with("http") && identifier.contains("arxiv.org") {
        let id = identifier.split('/').last().unwrap_or(identifier);
        arxiv::fetch_by_arxiv_id(id).await
    } else {
        arxiv::fetch_by_arxiv_id(identifier).await
    }
}
```

- [ ] **Step 2: 创建 external/crossref.rs**

```rust
use crate::models::paper::Paper;
use super::ImportResult;

#[derive(serde::Deserialize)]
struct CrossrefResponse {
    message: CrossrefMessage,
}

#[derive(serde::Deserialize)]
struct CrossrefMessage {
    title: Vec<String>,
    #[serde(rename = "published-print")]
    published_print: Option<CrossrefDate>,
    #[serde(rename = "published-online")]
    published_online: Option<CrossrefDate>,
    #[serde(rename = "container-title")]
    container_title: Option<Vec<String>>,
    author: Option<Vec<CrossrefAuthor>>,
    abstract_text: Option<String>,
}

#[derive(serde::Deserialize)]
struct CrossrefDate {
    #[serde(rename = "date-parts")]
    date_parts: Vec<Vec<i32>>,
}

#[derive(serde::Deserialize)]
struct CrossrefAuthor {
    given: Option<String>,
    family: Option<String>,
    sequence: Option<String>,
}

pub async fn fetch_by_doi(doi: &str) -> anyhow::Result<ImportResult> {
    let url = format!("https://api.crossref.org/works/{}", doi);
    let client = reqwest::Client::new();
    let resp = client.get(&url)
        .header("User-Agent", "LiteratureIntegration/1.0 (mailto:test@example.com)")
        .send().await?;

    if !resp.status().is_success() {
        anyhow::bail!("Crossref API error: {}", resp.status());
    }

    let data: CrossrefResponse = resp.json().await?;
    let msg = data.message;

    let title = msg.title.first().cloned().unwrap_or_default();
    let year = msg.published_print
        .as_ref()
        .or(msg.published_online.as_ref())
        .and_then(|d| d.date_parts.first())
        .and_then(|p| p.first().copied());

    let journal = msg.container_title
        .and_then(|v| v.first().cloned());

    let authors = msg.author.unwrap_or_default();
    let first_author = authors.first().and_then(|a| {
        match (&a.given, &a.family) {
            (Some(g), Some(f)) => Some(format!("{} {}", g, f)),
            (None, Some(f)) => Some(f.clone()),
            (Some(g), None) => Some(g.clone()),
            _ => None,
        }
    });

    let corresponding_author = authors.iter()
        .find(|a| a.sequence.as_deref() == Some("corresponding"))
        .or_else(|| authors.first())
        .and_then(|a| {
            match (&a.given, &a.family) {
                (Some(g), Some(f)) => Some(format!("{} {}", g, f)),
                (None, Some(f)) => Some(f.clone()),
                (Some(g), None) => Some(g.clone()),
                _ => None,
            }
        });

    let abstract_text = msg.abstract_text;

    let paper = Paper {
        id: uuid::Uuid::new_v4().to_string(),
        title,
        doi: Some(doi.to_string()),
        arxiv_id: None,
        year,
        journal,
        first_author,
        corresponding_author,
        keywords: vec![],
        created_at: chrono::Utc::now().to_rfc3339(),
    };

    Ok(ImportResult { paper, abstract_text })
}
```

- [ ] **Step 3: 创建 external/arxiv.rs**

```rust
use crate::models::paper::Paper;
use super::ImportResult;

pub async fn fetch_by_arxiv_id(arxiv_id: &str) -> anyhow::Result<ImportResult> {
    let url = format!("http://export.arxiv.org/api/query?id_list={}", arxiv_id);
    let client = reqwest::Client::new();
    let resp = client.get(&url).send().await?;

    if !resp.status().is_success() {
        anyhow::bail!("arXiv API error: {}", resp.status());
    }

    let body = resp.text().await?;

    let entry = extract_between(&body, "<entry>", "</entry>")
        .ok_or_else(|| anyhow::anyhow!("Paper not found on arXiv: {}", arxiv_id))?;

    let title = extract_between(&entry, "<title>", "</title>")
        .unwrap_or_default()
        .trim()
        .replace('\n', " ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");

    let abstract_text = extract_between(&entry, "<summary>", "</summary>")
        .map(|s| s.trim().replace('\n', " ").split_whitespace().collect::<Vec<_>>().join(" "));

    let first_author = extract_between(&entry, "<name>", "</name>");

    let year = extract_between(&entry, "<published>", "</published>")
        .and_then(|d| d.get(..4).and_then(|y| y.parse::<i32>().ok()));

    let paper = Paper {
        id: uuid::Uuid::new_v4().to_string(),
        title,
        doi: None,
        arxiv_id: Some(arxiv_id.to_string()),
        year,
        journal: None,
        first_author,
        corresponding_author: None,
        keywords: vec![],
        created_at: chrono::Utc::now().to_rfc3339(),
    };

    Ok(ImportResult { paper, abstract_text })
}

fn extract_between<'a>(text: &'a str, start: &str, end: &str) -> Option<&'a str> {
    let s = text.find(start)? + start.len();
    let e = text[s..].find(end)? + s;
    Some(&text[s..e])
}
```

- [ ] **Step 4: 验证编译**

```bash
cd /workspace/literature-app/src-tauri && cargo check 2>&1
```

- [ ] **Step 5: Commit**

```bash
git add -A
git commit -m "feat: add Crossref and arXiv API clients"
```

---

### Task 7: Tauri Commands — Workspace + App

**Files:**
- Create: `/workspace/literature-app/src-tauri/src/commands/mod.rs`
- Create: `/workspace/literature-app/src-tauri/src/commands/workspace.rs`
- Create: `/workspace/literature-app/src-tauri/src/commands/app.rs`

- [ ] **Step 1: 创建 commands/mod.rs**

```rust
pub mod app;
pub mod workspace;
pub mod paper;
pub mod search;
pub mod author;
pub mod export;
```

- [ ] **Step 2: 创建 commands/app.rs**

```rust
use crate::storage::AppState;
use crate::storage::index::{ensure_index, read_index};
use tauri::State;

#[tauri::command]
pub async fn select_root_dir(app: tauri::AppHandle, state: State<'_, AppState>) -> Result<String, String> {
    let dir = tauri_plugin_dialog::Dialog::new(&app)
        .file()
        .blocking_pick_folder()
        .ok_or("No directory selected")?
        .to_string();

    let root = std::path::PathBuf::from(&dir);
    ensure_index(&root).map_err(|e| e.to_string())?;

    let mut guard = state.root_dir.lock().unwrap();
    *guard = Some(root);

    Ok(dir)
}

#[tauri::command]
pub async fn get_root_dir(state: State<'_, AppState>) -> Result<String, String> {
    let guard = state.root_dir.lock().unwrap();
    guard.clone()
        .map(|p| p.to_string_lossy().to_string())
        .ok_or("Root directory not set".to_string())
}

#[tauri::command]
pub async fn set_root_dir(path: String, state: State<'_, AppState>) -> Result<String, String> {
    let root = std::path::PathBuf::from(&path);
    if !root.exists() {
        std::fs::create_dir_all(&root).map_err(|e| e.to_string())?;
    }
    ensure_index(&root).map_err(|e| e.to_string())?;

    let mut guard = state.root_dir.lock().unwrap();
    *guard = Some(root);

    Ok(path)
}
```

- [ ] **Step 3: 创建 commands/workspace.rs**

```rust
use crate::models::workspace::{CreateWorkspaceRequest, UpdateWorkspaceRequest, Workspace};
use crate::storage::AppState;
use crate::storage::index::get_root_dir;
use crate::storage::workspace;
use tauri::State;

#[tauri::command]
pub async fn list_workspaces(state: State<'_, AppState>) -> Result<Vec<Workspace>, String> {
    let root = get_root_dir(&state).map_err(|e| e.to_string())?;
    workspace::list_workspaces(&root).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_workspace(req: CreateWorkspaceRequest, state: State<'_, AppState>) -> Result<Workspace, String> {
    let root = get_root_dir(&state).map_err(|e| e.to_string())?;
    workspace::create_workspace(&root, &req.name, &req.description.unwrap_or_default())
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_workspace(id: String, state: State<'_, AppState>) -> Result<Workspace, String> {
    let root = get_root_dir(&state).map_err(|e| e.to_string())?;
    workspace::get_workspace(&root, &id).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_workspace(id: String, req: UpdateWorkspaceRequest, state: State<'_, AppState>) -> Result<Workspace, String> {
    let root = get_root_dir(&state).map_err(|e| e.to_string())?;
    workspace::update_workspace(&root, &id, req.name, req.description)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_workspace(id: String, state: State<'_, AppState>) -> Result<bool, String> {
    let root = get_root_dir(&state).map_err(|e| e.to_string())?;
    workspace::delete_workspace(&root, &id).map_err(|e| e.to_string())?;
    Ok(true)
}
```

- [ ] **Step 4: 验证编译**

```bash
cd /workspace/literature-app/src-tauri && cargo check 2>&1
```

- [ ] **Step 5: Commit**

```bash
git add -A
git commit -m "feat: add Tauri commands for workspace and app root dir"
```

---

### Task 8: Tauri Commands — Paper + Search + Author + Export

**Files:**
- Create: `/workspace/literature-app/src-tauri/src/commands/paper.rs`
- Create: `/workspace/literature-app/src-tauri/src/commands/search.rs`
- Create: `/workspace/literature-app/src-tauri/src/commands/author.rs`
- Create: `/workspace/literature-app/src-tauri/src/commands/export.rs`

- [ ] **Step 1: 创建 commands/paper.rs**

```rust
use crate::models::paper::{ImportPaperRequest, PaperDetailResponse, Paper, UpdatePaperRequest};
use crate::storage::AppState;
use crate::storage::index::get_root_dir;
use crate::storage::workspace::get_workspace_path_by_id;
use crate::storage::paper;
use crate::external;
use tauri::State;

#[tauri::command]
pub async fn list_papers(workspace_id: String, state: State<'_, AppState>) -> Result<Vec<Paper>, String> {
    let root = get_root_dir(&state).map_err(|e| e.to_string())?;
    let ws_path = get_workspace_path_by_id(&root, &workspace_id).map_err(|e| e.to_string())?;
    paper::list_papers(&root, &ws_path).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn import_paper(workspace_id: String, req: ImportPaperRequest, state: State<'_, AppState>) -> Result<PaperDetailResponse, String> {
    let root = get_root_dir(&state).map_err(|e| e.to_string())?;
    let ws_path = get_workspace_path_by_id(&root, &workspace_id).map_err(|e| e.to_string())?;

    let result = external::import_by_identifier(&req.identifier).await
        .map_err(|e| e.to_string())?;

    paper::create_paper_file(&root, &ws_path, &result.paper, result.abstract_text.as_deref(), None)
        .map_err(|e| e.to_string())?;

    Ok(PaperDetailResponse {
        paper: result.paper,
        abstract_text: result.abstract_text,
        user_notes: None,
    })
}

#[tauri::command]
pub async fn get_paper(id: String, state: State<'_, AppState>) -> Result<PaperDetailResponse, String> {
    let root = get_root_dir(&state).map_err(|e| e.to_string())?;
    paper::get_paper_detail(&root, &id).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_paper(id: String, req: UpdatePaperRequest, state: State<'_, AppState>) -> Result<PaperDetailResponse, String> {
    let root = get_root_dir(&state).map_err(|e| e.to_string())?;
    if let Some(notes) = req.user_notes {
        paper::update_paper_notes(&root, &id, &notes).map_err(|e| e.to_string())
    } else {
        paper::get_paper_detail(&root, &id).map_err(|e| e.to_string())
    }
}

#[tauri::command]
pub async fn delete_paper(workspace_id: String, paper_id: String, state: State<'_, AppState>) -> Result<bool, String> {
    let root = get_root_dir(&state).map_err(|e| e.to_string())?;
    let ws_path = get_workspace_path_by_id(&root, &workspace_id).map_err(|e| e.to_string())?;
    paper::delete_paper_file(&root, &ws_path, &paper_id).map_err(|e| e.to_string())?;
    Ok(true)
}
```

- [ ] **Step 2: 创建 commands/search.rs**

```rust
use crate::models::paper::Paper;
use crate::models::author::AuthorWithPapers;
use crate::storage::AppState;
use crate::storage::index::get_root_dir;
use crate::storage::workspace::get_workspace_path_by_id;
use crate::storage::paper;
use tauri::State;
use serde::Serialize;

#[derive(Serialize)]
pub struct SearchResponse {
    pub mode: String,
    pub results: serde_json::Value,
}

#[tauri::command]
pub async fn search(workspace_id: String, query: String, mode: String, state: State<'_, AppState>) -> Result<SearchResponse, String> {
    let root = get_root_dir(&state).map_err(|e| e.to_string())?;
    let ws_path = get_workspace_path_by_id(&root, &workspace_id).map_err(|e| e.to_string())?;
    let papers = paper::list_papers(&root, &ws_path).map_err(|e| e.to_string())?;

    let terms: Vec<String> = query.to_lowercase()
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();

    if terms.is_empty() {
        return Ok(SearchResponse {
            mode: mode.clone(),
            results: serde_json::json!([]),
        });
    }

    match mode.as_str() {
        "keyword" => {
            let filtered: Vec<Paper> = papers.into_iter().filter(|p| matches_keyword(p, &terms)).collect();
            Ok(SearchResponse { mode, results: serde_json::json!(filtered) })
        }
        "author" => {
            let filtered: Vec<Paper> = papers.into_iter().filter(|p| matches_author(p, &terms)).collect();
            let mut author_map: std::collections::HashMap<String, Vec<Paper>> = std::collections::HashMap::new();
            for p in &filtered {
                if let Some(ref name) = p.first_author {
                    author_map.entry(name.clone()).or_default().push(p.clone());
                }
                if let Some(ref name) = p.corresponding_author {
                    author_map.entry(name.clone()).or_default().push(p.clone());
                }
            }
            let results: Vec<AuthorWithPapers> = author_map.into_iter()
                .map(|(name, papers)| AuthorWithPapers { author_name: name, papers })
                .collect();
            Ok(SearchResponse { mode, results: serde_json::json!(results) })
        }
        "content" => {
            let mut filtered = Vec::new();
            for p in &papers {
                if let Ok(detail) = paper::get_paper_detail(&root, &p.id) {
                    if matches_content(&detail, &terms) {
                        filtered.push(p.clone());
                    }
                }
            }
            Ok(SearchResponse { mode, results: serde_json::json!(filtered) })
        }
        _ => Err("Invalid search mode. Use: keyword, author, content".to_string()),
    }
}

fn matches_keyword(paper: &Paper, terms: &[String]) -> bool {
    terms.iter().all(|term| {
        let t = term.to_lowercase();
        paper.title.to_lowercase().contains(&t)
            || paper.keywords.iter().any(|k| k.to_lowercase().contains(&t))
            || paper.journal.as_ref().map_or(false, |j| j.to_lowercase().contains(&t))
    })
}

fn matches_author(paper: &Paper, terms: &[String]) -> bool {
    terms.iter().all(|term| {
        let t = term.to_lowercase();
        paper.first_author.as_ref().map_or(false, |a| a.to_lowercase().contains(&t))
            || paper.corresponding_author.as_ref().map_or(false, |a| a.to_lowercase().contains(&t))
    })
}

fn matches_content(detail: &crate::models::paper::PaperDetailResponse, terms: &[String]) -> bool {
    let full_text = format!(
        "{} {} {} {} {}",
        detail.paper.title,
        detail.abstract_text.as_deref().unwrap_or(""),
        detail.user_notes.as_deref().unwrap_or(""),
        detail.paper.first_author.as_deref().unwrap_or(""),
        detail.paper.corresponding_author.as_deref().unwrap_or(""),
    ).to_lowercase();

    terms.iter().all(|term| full_text.contains(&term.to_lowercase()))
}
```

- [ ] **Step 3: 创建 commands/author.rs**

```rust
use crate::models::author::{Author, AuthorWithPapers, GraphDataResponse, GraphNode, GraphLink};
use crate::models::paper::Paper;
use crate::storage::AppState;
use crate::storage::index::get_root_dir;
use crate::storage::workspace::get_workspace_path_by_id;
use crate::storage::paper;
use tauri::State;
use std::collections::HashMap;

#[tauri::command]
pub async fn get_authors(workspace_id: String, state: State<'_, AppState>) -> Result<Vec<Author>, String> {
    let root = get_root_dir(&state).map_err(|e| e.to_string())?;
    let ws_path = get_workspace_path_by_id(&root, &workspace_id).map_err(|e| e.to_string())?;
    let papers = paper::list_papers(&root, &ws_path).map_err(|e| e.to_string())?;

    let mut author_map: HashMap<String, Author> = HashMap::new();
    for p in &papers {
        if let Some(ref name) = p.first_author {
            let entry = author_map.entry(name.clone()).or_insert_with(|| Author {
                name: name.clone(),
                first_author_count: 0,
                corresponding_author_count: 0,
                paper_count: 0,
            });
            entry.first_author_count += 1;
            entry.paper_count += 1;
        }
        if let Some(ref name) = p.corresponding_author {
            let entry = author_map.entry(name.clone()).or_insert_with(|| Author {
                name: name.clone(),
                first_author_count: 0,
                corresponding_author_count: 0,
                paper_count: 0,
            });
            entry.corresponding_author_count += 1;
            if p.first_author.as_ref() != Some(name) {
                entry.paper_count += 1;
            }
        }
    }

    Ok(author_map.into_values().collect())
}

#[tauri::command]
pub async fn get_graph(workspace_id: String, state: State<'_, AppState>) -> Result<GraphDataResponse, String> {
    let root = get_root_dir(&state).map_err(|e| e.to_string())?;
    let ws_path = get_workspace_path_by_id(&root, &workspace_id).map_err(|e| e.to_string())?;
    let papers = paper::list_papers(&root, &ws_path).map_err(|e| e.to_string())?;

    let mut author_papers: HashMap<String, (i32, String)> = HashMap::new();
    let mut coauthor_pairs: HashMap<(String, String), i32> = HashMap::new();

    for p in &papers {
        let mut paper_authors: Vec<String> = vec![];
        if let Some(ref name) = p.first_author {
            paper_authors.push(name.clone());
            let entry = author_papers.entry(name.clone()).or_insert((0, "first".to_string()));
            entry.0 += 1;
        }
        if let Some(ref name) = p.corresponding_author {
            if !paper_authors.contains(name) {
                paper_authors.push(name.clone());
            }
            let entry = author_papers.entry(name.clone()).or_insert((0, "corresponding".to_string()));
            entry.0 += 1;
            if entry.1 == "first" {
                entry.1 = "both".to_string();
            }
        }
        for i in 0..paper_authors.len() {
            for j in (i + 1)..paper_authors.len() {
                let mut a = paper_authors[i].clone();
                let mut b = paper_authors[j].clone();
                if a > b { std::mem::swap(&mut a, &mut b); }
                *coauthor_pairs.entry((a, b)).or_insert(0) += 1;
            }
        }
    }

    let nodes: Vec<GraphNode> = author_papers.iter()
        .map(|(name, (count, atype))| GraphNode {
            id: name.clone(),
            name: name.clone(),
            paper_count: *count,
            author_type: atype.clone(),
        })
        .collect();

    let links: Vec<GraphLink> = coauthor_pairs.iter()
        .map(|((a, b), count)| GraphLink {
            source: a.clone(),
            target: b.clone(),
            paper_count: *count,
        })
        .collect();

    Ok(GraphDataResponse { nodes, links })
}

#[tauri::command]
pub async fn get_author_papers(author_name: String, state: State<'_, AppState>) -> Result<AuthorWithPapers, String> {
    let root = get_root_dir(&state).map_err(|e| e.to_string())?;
    let index = crate::storage::index::read_index(&root).map_err(|e| e.to_string())?;
    let mut papers = Vec::new();
    for ws in &index.workspaces {
        if let Ok(ws_papers) = paper::list_papers(&root, &ws.path) {
            for p in ws_papers {
                if p.first_author.as_deref() == Some(&author_name)
                    || p.corresponding_author.as_deref() == Some(&author_name) {
                    papers.push(p);
                }
            }
        }
    }
    Ok(AuthorWithPapers { author_name, papers })
}
```

- [ ] **Step 4: 创建 commands/export.rs**

```rust
use crate::storage::AppState;
use crate::storage::index::get_root_dir;
use crate::storage::workspace::get_workspace_path_by_id;
use crate::storage::paper;
use tauri::State;

#[tauri::command]
pub async fn export_workspace(workspace_id: String, group_by: Option<String>, state: State<'_, AppState>) -> Result<String, String> {
    let root = get_root_dir(&state).map_err(|e| e.to_string())?;
    let ws_path = get_workspace_path_by_id(&root, &workspace_id).map_err(|e| e.to_string())?;
    let ws = crate::storage::workspace::get_workspace(&root, &workspace_id).map_err(|e| e.to_string())?;
    let papers = paper::list_papers(&root, &ws_path).map_err(|e| e.to_string())?;

    let group = group_by.unwrap_or_else(|| "author".to_string());

    let mut md = format!("# 工作区: {}\n\n", ws.name);
    if !ws.description.is_empty() {
        md.push_str(&format!("{}\n\n", ws.description));
    }

    match group.as_str() {
        "author" => {
            let mut author_papers: std::collections::HashMap<String, Vec<_>> = std::collections::HashMap::new();
            for p in &papers {
                let author = p.first_author.as_deref()
                    .or(p.corresponding_author.as_deref())
                    .unwrap_or("Unknown")
                    .to_string();
                author_papers.entry(author).or_default().push(p);
            }
            for (author, ps) in author_papers {
                md.push_str(&format!("## {}\n\n", author));
                for p in ps {
                    md.push_str(&format_paper(p));
                }
            }
        }
        "keyword" => {
            let mut kw_papers: std::collections::HashMap<String, Vec<_>> = std::collections::HashMap::new();
            for p in &papers {
                let kws = if p.keywords.is_empty() { vec!["Uncategorized".to_string()] } else { p.keywords.clone() };
                for kw in kws {
                    kw_papers.entry(kw).or_default().push(p);
                }
            }
            for (kw, ps) in kw_papers {
                md.push_str(&format!("## {}\n\n", kw));
                for p in ps {
                    md.push_str(&format_paper(p));
                }
            }
        }
        _ => {
            for p in &papers {
                md.push_str(&format_paper(p));
            }
        }
    }

    Ok(md)
}

fn format_paper(p: &crate::models::paper::Paper) -> String {
    let mut s = format!("### {}\n\n", p.title);
    if let Some(ref y) = p.year { s.push_str(&format!("- 年份: {}\n", y)); }
    if let Some(ref j) = p.journal { s.push_str(&format!("- 期刊: {}\n", j)); }
    if let Some(ref a) = p.first_author { s.push_str(&format!("- 一作: {}\n", a)); }
    if let Some(ref a) = p.corresponding_author { s.push_str(&format!("- 通讯: {}\n", a)); }
    if let Some(ref d) = p.doi { s.push_str(&format!("- DOI: {}\n", d)); }
    if !p.keywords.is_empty() { s.push_str(&format!("- 关键词: {}\n", p.keywords.join(", "))); }
    s.push_str("\n");
    s
}
```

- [ ] **Step 5: 验证编译**

```bash
cd /workspace/literature-app/src-tauri && cargo check 2>&1
```

- [ ] **Step 6: Commit**

```bash
git add -A
git commit -m "feat: add Tauri commands for paper, search, author, export"
```

---

### Task 9: Tauri 入口 — main.rs + lib.rs

**Files:**
- Create: `/workspace/literature-app/src-tauri/src/lib.rs`
- Modify: `/workspace/literature-app/src-tauri/src/main.rs`

- [ ] **Step 1: 创建 lib.rs**

```rust
mod models;
mod storage;
mod commands;
mod external;

use storage::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![
            commands::app::select_root_dir,
            commands::app::get_root_dir,
            commands::app::set_root_dir,
            commands::workspace::list_workspaces,
            commands::workspace::create_workspace,
            commands::workspace::get_workspace,
            commands::workspace::update_workspace,
            commands::workspace::delete_workspace,
            commands::paper::list_papers,
            commands::paper::import_paper,
            commands::paper::get_paper,
            commands::paper::update_paper,
            commands::paper::delete_paper,
            commands::search::search,
            commands::author::get_authors,
            commands::author::get_graph,
            commands::author::get_author_papers,
            commands::export::export_workspace,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

- [ ] **Step 2: 修改 main.rs**

```rust
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    literature_app_lib::run()
}
```

注意：Tauri 的库名取决于 `Cargo.toml` 中的 `[lib]` 配置。如果 `Cargo.toml` 中 `name = "literature_app_lib"`，则使用 `literature_app_lib::run()`。确认 `Cargo.toml` 中有：

```toml
[lib]
name = "literature_app_lib"
crate-type = ["staticlib", "cdylib", "rlib"]
```

- [ ] **Step 3: 验证编译**

```bash
cd /workspace/literature-app/src-tauri && cargo check 2>&1
```

- [ ] **Step 4: Commit**

```bash
git add -A
git commit -m "feat: add Tauri entry point with all command registrations"
```

---

### Task 10: 前端 — API 层迁移到 Tauri invoke

**Files:**
- Create: `/workspace/literature-app/src/api/client.ts`
- Create: `/workspace/literature-app/src/api/workspace.ts`
- Create: `/workspace/literature-app/src/api/paper.ts`
- Create: `/workspace/literature-app/src/api/author.ts`
- Create: `/workspace/literature-app/src/api/search.ts`
- Create: `/workspace/literature-app/src/api/export.ts`
- Create: `/workspace/literature-app/src/api/app.ts`

- [ ] **Step 1: 创建 api/client.ts**

```typescript
import { invoke } from '@tauri-apps/api/core';

export async function tauriInvoke<T>(command: string, args?: Record<string, unknown>): Promise<T> {
  return invoke<T>(command, args);
}
```

- [ ] **Step 2: 创建 api/app.ts**

```typescript
import { tauriInvoke } from './client';

export const selectRootDir = () => tauriInvoke<string>('select_root_dir');
export const getRootDir = () => tauriInvoke<string>('get_root_dir');
export const setRootDir = (path: string) => tauriInvoke<string>('set_root_dir', { path });
```

- [ ] **Step 3: 创建 api/workspace.ts**

```typescript
import { tauriInvoke } from './client';
import type { Workspace } from '../types';

export const listWorkspaces = () => tauriInvoke<Workspace[]>('list_workspaces');
export const createWorkspace = (data: { name: string; description?: string }) =>
  tauriInvoke<Workspace>('create_workspace', { req: data });
export const getWorkspace = (id: string) => tauriInvoke<Workspace>('get_workspace', { id });
export const updateWorkspace = (id: string, data: { name?: string; description?: string }) =>
  tauriInvoke<Workspace>('update_workspace', { id, req: data });
export const deleteWorkspace = (id: string) => tauriInvoke<boolean>('delete_workspace', { id });
```

- [ ] **Step 4: 创建 api/paper.ts**

```typescript
import { tauriInvoke } from './client';
import type { Paper, PaperDetail } from '../types';

export const listPapers = (workspaceId: string) =>
  tauriInvoke<Paper[]>('list_papers', { workspaceId });
export const importPaper = (workspaceId: string, identifier: string) =>
  tauriInvoke<PaperDetail>('import_paper', { workspaceId, req: { identifier } });
export const getPaper = (id: string) =>
  tauriInvoke<PaperDetail>('get_paper', { id });
export const updatePaper = (id: string, data: { user_notes?: string }) =>
  tauriInvoke<PaperDetail>('update_paper', { id, req: data });
export const deletePaper = (workspaceId: string, paperId: string) =>
  tauriInvoke<boolean>('delete_paper', { workspaceId, paperId });
```

- [ ] **Step 5: 创建 api/author.ts**

```typescript
import { tauriInvoke } from './client';
import type { Author, GraphData, AuthorWithPapers } from '../types';

export const listAuthors = (workspaceId: string) =>
  tauriInvoke<Author[]>('get_authors', { workspaceId });
export const getGraphData = (workspaceId: string) =>
  tauriInvoke<GraphData>('get_graph', { workspaceId });
export const getAuthorPapers = (authorName: string) =>
  tauriInvoke<AuthorWithPapers>('get_author_papers', { authorName });
```

- [ ] **Step 6: 创建 api/search.ts**

```typescript
import { tauriInvoke } from './client';

export interface SearchResponse {
  mode: string;
  results: any;
}

export const searchByKeyword = (workspaceId: string, query: string) =>
  tauriInvoke<SearchResponse>('search', { workspaceId, query, mode: 'keyword' });
export const searchByAuthor = (workspaceId: string, query: string) =>
  tauriInvoke<SearchResponse>('search', { workspaceId, query, mode: 'author' });
export const searchByContent = (workspaceId: string, query: string) =>
  tauriInvoke<SearchResponse>('search', { workspaceId, query, mode: 'content' });
```

- [ ] **Step 7: 创建 api/export.ts**

```typescript
import { tauriInvoke } from './client';

export const exportWorkspace = (workspaceId: string, groupBy: string = 'author') =>
  tauriInvoke<string>('export_workspace', { workspaceId, groupBy });
```

- [ ] **Step 8: Commit**

```bash
git add -A
git commit -m "feat: migrate frontend API layer to Tauri invoke"
```

---

### Task 11: 前端 — 类型定义 + 根目录选择组件

**Files:**
- Create: `/workspace/literature-app/src/types/index.ts`
- Create: `/workspace/literature-app/src/components/RootSelector.tsx`

- [ ] **Step 1: 创建 types/index.ts**

```typescript
export interface Workspace {
  id: string;
  name: string;
  description: string;
  created_at: string;
}

export interface Paper {
  id: string;
  title: string;
  doi: string | null;
  arxiv_id: string | null;
  year: number | null;
  journal: string | null;
  first_author: string | null;
  corresponding_author: string | null;
  keywords: string[];
  created_at: string;
}

export interface PaperDetail {
  paper: Paper;
  abstract_text: string | null;
  user_notes: string | null;
}

export interface Author {
  name: string;
  first_author_count: number;
  corresponding_author_count: number;
  paper_count: number;
}

export interface GraphNode {
  id: string;
  name: string;
  paper_count: number;
  author_type: string;
}

export interface GraphLink {
  source: string;
  target: string;
  paper_count: number;
}

export interface GraphData {
  nodes: GraphNode[];
  links: GraphLink[];
}

export interface AuthorWithPapers {
  author_name: string;
  papers: Paper[];
}

export interface ExportRequest {
  format: string;
  group_by?: string;
}
```

- [ ] **Step 2: 创建 components/RootSelector.tsx**

```tsx
import { useState } from 'react';
import { Button, Typography } from 'antd';
import { FolderOpenOutlined } from '@ant-design/icons';
import { selectRootDir, setRootDir } from '../api/app';

interface Props {
  onSelected: () => void;
}

export default function RootSelector({ onSelected }: Props) {
  const [loading, setLoading] = useState(false);

  const handleSelect = async () => {
    setLoading(true);
    try {
      await selectRootDir();
      onSelected();
    } catch (e) {
      console.error(e);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div
      style={{
        display: 'flex',
        flexDirection: 'column',
        alignItems: 'center',
        justifyContent: 'center',
        height: '100vh',
        background: 'var(--bg-deep)',
        gap: 24,
      }}
    >
      <div style={{ fontSize: 64, opacity: 0.3 }}>📚</div>
      <Typography.Title
        level={2}
        style={{
          fontFamily: 'var(--font-display)',
          color: 'var(--text-primary)',
          margin: 0,
        }}
      >
        Literature Integration
      </Typography.Title>
      <Typography.Text style={{ color: 'var(--text-muted)', fontSize: 15 }}>
        选择一个文件夹作为根目录，所有工作区和论文将存储在此
      </Typography.Text>
      <Button
        type="primary"
        size="large"
        icon={<FolderOpenOutlined />}
        loading={loading}
        onClick={handleSelect}
        style={{ marginTop: 8 }}
      >
        选择根目录
      </Button>
    </div>
  );
}
```

- [ ] **Step 3: Commit**

```bash
git add -A
git commit -m "feat: add TypeScript types and root directory selector component"
```

---

### Task 12: 前端 — 迁移所有组件和页面

**Files:**
- Copy and adapt all components from `/workspace/frontend/src/components/` to `/workspace/literature-app/src/components/`
- Copy and adapt all pages from `/workspace/frontend/src/pages/` to `/workspace/literature-app/src/pages/`
- Copy and adapt all hooks from `/workspace/frontend/src/hooks/` to `/workspace/literature-app/src/hooks/`
- Copy `styles/global.css` from `/workspace/frontend/src/styles/`
- Modify: `/workspace/literature-app/src/App.tsx`
- Modify: `/workspace/literature-app/src/main.tsx`

- [ ] **Step 1: 复制组件文件**

将以下文件从 `/workspace/frontend/src/components/` 复制到 `/workspace/literature-app/src/components/`（无需修改，API 接口不变）：
- Layout.tsx
- WorkspaceList.tsx
- WorkspaceForm.tsx
- PaperList.tsx
- PaperImport.tsx
- PaperDetail.tsx
- PaperNotes.tsx
- AuthorGraph.tsx
- SearchBar.tsx
- SearchResult.tsx
- ExportPanel.tsx

- [ ] **Step 2: 修改 SearchBar.tsx 增加内容搜索**

在 SearchBar 组件的 Radio.Group 中增加一个选项：

```tsx
<Radio.Group value={mode} onChange={(e) => setMode(e.target.value)} size="small">
  <Radio.Button value="keyword">关键词</Radio.Button>
  <Radio.Button value="author">作者</Radio.Button>
  <Radio.Button value="content">内容</Radio.Button>
</Radio.Group>
```

- [ ] **Step 3: 复制页面文件**

将以下文件从 `/workspace/frontend/src/pages/` 复制到 `/workspace/literature-app/src/pages/`：
- WorkspacesPage.tsx
- WorkspaceDetail.tsx
- PaperPage.tsx
- SearchPage.tsx

- [ ] **Step 4: 复制 hooks**

将以下文件从 `/workspace/frontend/src/hooks/` 复制到 `/workspace/literature-app/src/hooks/`：
- useWorkspaces.ts
- usePapers.ts
- useSearch.ts
- useGraph.ts

- [ ] **Step 5: 复制样式**

复制 `/workspace/frontend/src/styles/global.css` 到 `/workspace/literature-app/src/styles/global.css`

- [ ] **Step 6: 修改 App.tsx**

```tsx
import { useState, useEffect } from 'react';
import { BrowserRouter, Routes, Route } from 'react-router-dom';
import { getRootDir } from './api/app';
import Layout from './components/Layout';
import RootSelector from './components/RootSelector';
import WorkspacesPage from './pages/WorkspacesPage';
import WorkspaceDetail from './pages/WorkspaceDetail';
import PaperPage from './pages/PaperPage';
import SearchPage from './pages/SearchPage';
import './styles/global.css';

export default function App() {
  const [hasRoot, setHasRoot] = useState(false);
  const [checking, setChecking] = useState(true);

  useEffect(() => {
    getRootDir()
      .then(() => setHasRoot(true))
      .catch(() => setHasRoot(false))
      .finally(() => setChecking(false));
  }, []);

  if (checking) return null;

  if (!hasRoot) {
    return <RootSelector onSelected={() => setHasRoot(true)} />;
  }

  return (
    <BrowserRouter>
      <Routes>
        <Route element={<Layout />}>
          <Route path="/" element={<WorkspacesPage />} />
          <Route path="/workspace/:id" element={<WorkspaceDetail />} />
          <Route path="/workspace/:id/search" element={<SearchPage />} />
          <Route path="/paper/:id" element={<PaperPage />} />
        </Route>
      </Routes>
    </BrowserRouter>
  );
}
```

- [ ] **Step 7: 修改 main.tsx**

```tsx
import React from 'react';
import ReactDOM from 'react-dom/client';
import App from './App';

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);
```

- [ ] **Step 8: Commit**

```bash
git add -A
git commit -m "feat: migrate all frontend components, pages, hooks to Tauri app"
```

---

### Task 13: Vite 配置 + Tauri 集成

**Files:**
- Modify: `/workspace/literature-app/vite.config.ts`
- Modify: `/workspace/literature-app/package.json`

- [ ] **Step 1: 配置 vite.config.ts**

```typescript
import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

const host = process.env.TAURI_DEV_HOST;

export default defineConfig(async () => ({
  plugins: [react()],
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
    host: host || false,
    hmr: host ? { protocol: "ws", host, port: 1421 } : undefined,
    watch: { ignored: ["**/src-tauri/**"] },
  },
}));
```

- [ ] **Step 2: 确认 package.json scripts**

确保 package.json 中有：

```json
{
  "scripts": {
    "dev": "vite",
    "build": "tsc && vite build",
    "tauri": "tauri"
  }
}
```

- [ ] **Step 3: 验证前端构建**

```bash
cd /workspace/literature-app && npx tsc --noEmit 2>&1
```

修复任何 TypeScript 错误。

- [ ] **Step 4: Commit**

```bash
git add -A
git commit -m "feat: configure Vite for Tauri integration"
```

---

### Task 14: 集成测试 — 构建并运行桌面应用

**Files:**
- None (verification only)

- [ ] **Step 1: 开发模式运行**

```bash
cd /workspace/literature-app && npm run tauri dev 2>&1
```

验证：
1. 应用窗口启动
2. 首次启动显示根目录选择界面
3. 选择根目录后进入主界面
4. 可以创建工作区
5. 可以导入论文
6. 可以搜索论文
7. 可以编辑笔记
8. 可以查看作者关系图
9. 可以导出 Markdown

- [ ] **Step 2: 修复运行时错误**

根据 Step 1 的测试结果，修复任何运行时错误。

- [ ] **Step 3: 构建 release**

```bash
cd /workspace/literature-app && npm run tauri build 2>&1
```

验证在 `src-tauri/target/release/bundle/` 下生成了可执行文件。

- [ ] **Step 4: Commit**

```bash
git add -A
git commit -m "feat: complete desktop app — build and verify"
```

---

## Self-Review

**1. Spec coverage:**
- ✅ Tauri v2 桌面应用 → Task 1, 9
- ✅ 纯文件系统存储 → Task 3, 4, 5
- ✅ 工作区 = 文件夹 → Task 4
- ✅ 论文 = Markdown + Frontmatter → Task 5
- ✅ 模糊搜索（keyword/author/content）→ Task 8
- ✅ 根目录选择 → Task 7, 11
- ✅ Crossref/arXiv 导入 → Task 6
- ✅ 作者关系图 → Task 8
- ✅ Markdown 导出 → Task 8
- ✅ 前端迁移 → Task 10, 12, 13

**2. Placeholder scan:** No TBD/TODO found. All code is complete.

**3. Type consistency:** All types match between Rust models and TypeScript types. Command parameter names use camelCase (Tauri convention) matching frontend invoke calls.
