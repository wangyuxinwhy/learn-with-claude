# Pre-commit

Git hooks 框架：自动化代码提交前的检查与格式化。

## 核心概念

### 为什么需要 pre-commit 框架？

Git 原生 `.git/hooks/` 有三大痛点：

1. **不能共享** — `.git/` 目录不被版本控制追踪
2. **难以维护** — 多工具混在一个 shell 脚本里，越来越臃肿
3. **环境管理麻烦** — 各工具的安装和版本要自己搞定

pre-commit 框架通过 `.pre-commit-config.yaml` 配置文件解决共享问题，通过 `language` 机制解决环境管理问题。

### 工作机制

```
git commit
  → .git/hooks/pre-commit（桥接脚本）
    → pre-commit 读取 .pre-commit-config.yaml
      → 对每个 hook：过滤文件 → 执行 {entry} {args} {files}
        → 退出码 0：通过 / 非 0：阻止提交
```

安装流程：`pre-commit install` 往 `.git/hooks/pre-commit` 写入桥接脚本。团队新成员 clone 后需执行一次此命令。

### 配置结构

```yaml
repos:
- repo: https://github.com/pre-commit/pre-commit-hooks  # hook 来源仓库
  rev: v3.2.0                                            # 锁定版本
  hooks:
  - id: trailing-whitespace                              # 选用的 hook
```

- **repos** — 所有 hook 来源仓库列表
- **repo** — 具体仓库地址（或 `local`）
- **rev** — 版本锁定（类似 lock 文件）
- **hooks** — 从该仓库选用的 hooks

远程 hook 只需写 `id`，因为仓库方在 `.pre-commit-hooks.yaml` 里已定义了 `entry`、`language`、`types` 等所有细节。

### language 的作用

类似 shebang，告诉 pre-commit 用什么环境运行 `entry` 命令：

| language | 行为 | 适用场景 |
|----------|------|---------|
| `python` | 创建隔离 venv + `pip install` 包 | 远程 hook 仓库 |
| `system` | 直接用系统命令 | 快速本地 hook |
| `script` | 执行仓库里的脚本（需 shebang + 可执行权限） | 本地脚本 |
| `pygrep` | 正则匹配文件内容，匹配到就失败 | 简单的文本检查 |
| `fail` | 匹配到文件就直接失败 | 禁止某类文件 |
| `node`/`rust`/`golang`... | 对应语言的隔离环境 | 各语言生态的工具 |

### language: python 的隔离环境流程

以 ruff 为例：

```
首次运行:
  git clone ruff-pre-commit → checkout rev
    → 读取 .pre-commit-hooks.yaml
    → 创建隔离 venv (~/.cache/pre-commit/repoXXX/)
    → pip install .（安装 ruff 包）
    → 缓存复用

后续运行:
  直接复用已有环境 → 执行 entry 命令
```

### 文件传递机制

- 默认把匹配文件**追加为命令行参数**：`{entry} {args} {files}`
- `pass_filenames: false` — 不传文件名
- 文件量大时自动分批，避免超出系统命令行长度限制
- `require_serial: true` — 禁止并行执行

### Hook 两种行为模式

- **纯检查**（如 `check-yaml`）— 报错阻止提交
- **自动修复**（如 `trailing-whitespace`、`ruff-format`）— 修改文件但不自动 stage，需要手动 `git add` 后重新提交

### 常用 Hook Schema

| 字段 | 默认值 | 作用 |
|------|--------|------|
| `id` | 必填 | hook 标识 |
| `name` | 继承 | 显示名称 |
| `entry` | 继承 | 要执行的命令 |
| `language` | 继承 | 运行环境 |
| `args` | `[]` | 使用方传给 hook 的额外参数 |
| `files` | `''` | 正则匹配文件 |
| `exclude` | `^$` | 正则排除文件 |
| `types` | `[file]` | 文件类型过滤（AND 逻辑） |
| `types_or` | `[]` | 文件类型过滤（OR 逻辑） |
| `stages` | 所有 | 在哪个 git hook 阶段运行 |
| `pass_filenames` | `true` | 是否传文件名 |
| `always_run` | `false` | 无匹配文件也运行 |
| `require_serial` | `false` | 禁止并行 |
| `additional_dependencies` | `[]` | 额外安装到隔离环境的依赖 |
| `language_version` | `default` | 指定语言版本 |

`args` 是给使用方覆盖用的，`entry` 是仓库方定义的默认行为——关注点分离。

`types` 用 AND 逻辑（如 `[text, executable]` = 同时满足），`types_or` 用 OR 逻辑（如 `[python, pyi, jupyter]` = 任一匹配）。

## 写自己的 Hook

### 最简方式：language: system

```yaml
- repo: local
  hooks:
  - id: no-todo
    name: Check for TODO comments
    entry: TODO
    language: pygrep     # 正则匹配，匹配到就失败
    types: [python]
```

### 带外部依赖：language: python（需打包）

需要完整的 Python 包结构：

```
my-hook/
├── .pre-commit-hooks.yaml   # 声明 hook 的 id、entry、language
├── pyproject.toml            # 包含 console_scripts 入口
└── main.py
```

`entry` 写的是 `console_scripts` 里的命令名，不是包名。

### Monorepo 下的实用方式

```yaml
- repo: local
  hooks:
  - id: my-check
    entry: uv run python scripts/my_check.py
    language: system
```

直接使用项目自身的 `pyproject.toml` 和 `uv.lock` 解析依赖，monorepo 的 workspace 依赖自动生效。

## prek：Rust 原生替代品

[prek](https://prek.j178.dev/) 是 pre-commit 的 Rust 重写，兼容 `.pre-commit-config.yaml`。

### 核心优势

| | pre-commit | prek |
|--|-----------|------|
| 依赖 | 需要 Python | 单一二进制，无依赖 |
| 环境管理 | pip（慢） | uv 原生集成（快） |
| hook 执行 | 串行 | 并行 |
| 内置 hooks | 无 | 16 个 Rust 原生实现 |
| 配置格式 | YAML | TOML（也兼容 YAML） |

### builtin hooks

```toml
# prek.toml — 不需要 clone，离线可用
[[repos]]
repo = "builtin"
hooks = [
  { id = "trailing-whitespace" },
  { id = "end-of-file-fixer" },
  { id = "check-yaml" },
]
```

### PEP 723 支持（prek 独有）

无需打包，直接在脚本头部声明依赖：

```python
# /// script
# requires-python = ">=3.10"
# dependencies = ["ast-grep-py"]
# ///
```

配置里只需 `language = "python"`，prek 自动读取元数据并用 uv 安装依赖。

注意：如果同时写了 `additional_dependencies`，PEP 723 元数据会被忽略——两者互斥。

### 声明依赖的方式总结

| 方式 | 适用场景 |
|------|---------|
| PEP 723 `# /// script` | 只依赖 PyPI 包，最简洁（prek 独有） |
| `additional_dependencies` | 需要本地路径依赖，或兼容 pre-commit |
| `language: system` + `uv run` | monorepo，用项目自身环境解析依赖 |
| 独立 repo + `console_scripts` | 发布给社区使用 |

## 实践产出

- [precommit-demo](https://github.com/wangyuxinwhy/learn-with-claude) 实验仓库：包含 pre-commit 和 prek 的各种 hook 配置示例
