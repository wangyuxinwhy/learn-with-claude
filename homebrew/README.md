# Homebrew 学习笔记

## 核心概念

### Homebrew 解决的痛点

| 传统方式               | brew 方式                      |
| ---------------------- | ------------------------------ |
| 手动找官网、下载、拖拽 | `brew install xxx`             |
| 不知道哪些软件有更新   | `brew upgrade` 批量更新        |
| 换电脑要重装 20 个软件 | `brew bundle install` 一键恢复 |

### 核心机制

- **brew 的本质**：用 Git 管理的软件包管理器
- **Formula/Cask**：软件的"安装配方"，存在 Git 仓库里
- **`brew update`**：本质是 `git pull`，拉取最新配方

### 关键术语

| 术语        | 含义                                 |
| ----------- | ------------------------------------ |
| **tap**     | 第三方软件源（任何人可以创建）       |
| **formula** | 命令行工具的配方                     |
| **cask**    | GUI 应用的配方（装到 /Applications） |

### 命令解析

```bash
brew install --cask steipete/tap/repobar
            │       │            │
            │       │            └── 软件名
            │       └── 第三方仓库（GitHub: steipete/homebrew-tap）
            └── 类型标志（GUI 应用）
```

## 发布自己的 brew 软件

### 需要两个仓库

| 仓库                  | 存放内容            | 作用                     |
| --------------------- | ------------------- | ------------------------ |
| `用户名/软件名`       | 软件源码 + Release  | 软件的"家"，提供下载     |
| `用户名/homebrew-tap` | Formula 文件（.rb） | 告诉 brew 怎么下载和安装 |

### Formula 示例

```ruby
class Catsay < Formula
  desc "A cat that says things in your terminal"
  homepage "https://github.com/wangyuxinwhy/catsay"
  url "https://github.com/wangyuxinwhy/catsay/releases/download/v1.0.0/catsay"
  sha256 "bee029d7ff9624dfc65e575d6703076332b16f65076700b94fe0ff7384fbc602"
  version "1.0.0"
  license "MIT"

  def install
    bin.install "catsay"
  end
end
```

### 发布流程

```
1. 写工具 → 推到 GitHub → 创建 Release
2. 创建 homebrew-tap 仓库（必须以 homebrew- 开头）
3. 写 Formula（url + sha256 + install 指令）
4. 任何人都可以 brew install 用户名/tap/软件名
```

### 更新版本

```
1. 修改代码
2. 创建新 Release（新 tag）
3. 更新 Formula（url、sha256、version）
4. 用户 brew update && brew upgrade
```

## Python CLI 发布方案对比

| 方案                | 大小   | 启动速度 | 用户依赖          | Formula 复杂度 |
| ------------------- | ------ | -------- | ----------------- | -------------- |
| Python + virtualenv | 几百KB | 快       | Python + 所有依赖 | 复杂           |
| PyInstaller 打包    | ~13MB  | 慢(3-5s) | 无                | 简单           |
| uv script           | 几KB   | 快       | uv                | 简单           |
| Rust 重写           | 几MB   | 瞬时     | 无                | 简单           |

## GitHub Actions 自动化

### 触发条件

```yaml
on:
  push:
    tags:
      - "v*"
```

### 自动化流程

```
git tag v1.2.0 && git push --tags
       ↓
CI 自动执行：
  1. PyInstaller 打包
  2. 创建 GitHub Release
  3. 计算 SHA256
  4. 更新 homebrew-tap Formula
  5. 推送 Formula
```

### 需要配置

1. **仓库 Workflow 权限**：Settings → Actions → General → Workflow permissions → Read and write
2. **Fine-grained Token**：
   - Repository access: 选择 homebrew-tap
   - Permissions: Contents → Read and write
3. **Repository Secret**：`HOMEBREW_TAP_TOKEN`

## Brewfile

用一个文件管理所有开发环境软件：

```bash
# 导出当前软件清单
brew bundle dump

# 在新电脑恢复
brew bundle install
```

Brewfile 示例：

```ruby
tap "wangyuxinwhy/tap"
brew "bat"
brew "gh"
brew "wangyuxinwhy/tap/catsay"
cask "firefox"
vscode "github.copilot"
```

建议放在 dotfiles 仓库管理。

## 实践成果

- https://github.com/wangyuxinwhy/catsay - Shell 版 catsay
- https://github.com/wangyuxinwhy/pycatsay - Python 版（PyInstaller 打包）
- https://github.com/wangyuxinwhy/homebrew-tap - 个人 Homebrew tap

```bash
# 安装
brew install wangyuxinwhy/tap/catsay
brew install wangyuxinwhy/tap/pycatsay

# 使用
catsay "Hello Homebrew!"
pycatsay "Hello from Python!"
```

## 设计思考

### Homebrew vs PyPI

| 维度     | PyPI           | Homebrew                    |
| -------- | -------------- | --------------------------- |
| 架构     | 中心化服务器   | 去中心化（Git 仓库）        |
| 账号     | 需要注册       | 只需 GitHub 账号            |
| 命名空间 | 扁平（易抢注） | 用户名/tap/包名（天然隔离） |
| 发现性   | 中心化搜索     | 官方仓库 + 分散 tap         |

### Homebrew 的两层结构

| 层级       | 特点                         |
| ---------- | ---------------------------- |
| 官方仓库   | 中心化、有审核、易发现       |
| 第三方 tap | 去中心化、无门槛、需自己传播 |

兼顾了"发现性"和"开放性"。

---

_学习日期：2026-01-23_
