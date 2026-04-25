# XY Backend

基于 **Axum + PostgreSQL** 的 Rust 后端服务，提供用户、社交、商品、帖子、消息等完整 REST API 及 WebSocket 实时通信。

---

## 技术栈

| 组件 | 版本 | 用途 |
|------|------|------|
| [Axum](https://github.com/tokio-rs/axum) | 0.7 | Web 框架 |
| [sqlx](https://github.com/launchbadge/sqlx) | 0.8 | 异步 PostgreSQL 驱动 |
| [Tokio](https://tokio.rs) | 1.37 | 异步运行时 |
| [jsonwebtoken](https://github.com/Keats/jsonwebtoken) | 9.2 | JWT 鉴权 |
| [bcrypt](https://github.com/Keats/rust-bcrypt) | 0.15 | 密码哈希 |
| [uuid](https://github.com/uuid-rs/uuid) | 1.8 | 主键生成 |
| [chrono](https://github.com/chronotope/chrono) | 0.4 | 时间处理 |
| [tower-http](https://github.com/tower-rs/tower-http) | 0.6 | CORS 中间件 |
| [dashmap](https://github.com/xacrimon/dashmap) | 6.1 | WebSocket 连接并发 Map |

---

## 快速启动

### 前置条件

- Rust 1.75+
- PostgreSQL 14+

### 1. 创建数据库

```sql
CREATE DATABASE xy;
```

### 2. 配置环境变量

```bash
export DATABASE_URL=postgres://postgres:password@localhost:5432/xy
```

默认值（无需设置）：`postgres://postgres:postgres@localhost:5432/xy`

### 3. 运行

```bash
cargo run
```

服务启动时会自动执行 `migrations/001_init.sql` 建表（幂等）。

服务监听：`http://0.0.0.0:8090`

---

## 项目结构

```
src/
├── main.rs                  # 入口：路由注册、DB 连接
├── database/
│   ├── mod.rs               # Database 结构体（PgPool + WS连接表）
│   └── models.rs            # 数据模型、请求/响应 DTO、行转换函数
├── verify/
│   ├── mod.rs
│   └── auth.rs              # 注册、登录、JWT 解析
├── user/
│   ├── mod.rs               # 用户 CRUD、收藏查询
│   └── search.rs            # 模糊搜索评分算法
└── social/
    ├── mod.rs               # 路由聚合、举报
    ├── follows.rs           # 关注 / 取关
    ├── conversations.rs     # 私聊会话 CRUD
    ├── messages.rs          # 消息获取 + WebSocket 实时消息
    ├── groups.rs            # 群组管理、群消息
    ├── posts.rs             # 帖子、评论、回复、收藏
    ├── products.rs          # 商品 CRUD、收藏、地理筛选
    └── categories.rs        # 分类列表

migrations/
└── 001_init.sql             # 完整建表脚本（17 张表）
```

---

## API 路由

### 认证 `/auth`

| 方法 | 路径 | 说明 |
|------|------|------|
| POST | `/auth/register` | 注册，返回 JWT |
| POST | `/auth/login` | 登录，返回 JWT |

所有需要鉴权的接口在请求头中携带：`Authorization: Bearer <token>`

### 用户 `/user`

| 方法 | 路径 | 说明 |
|------|------|------|
| GET | `/user` | 用户列表（支持 `role`/`sort`/`query` 筛选）|
| GET | `/user/:id` | 获取单个用户 |
| POST | `/user/:id` | 更新用户信息（本人或管理员）|
| DELETE | `/user/:id` | 删除用户 |
| GET | `/user/:id/favorites` | 获取用户收藏的商品和帖子 |

### 社交 `/social`

| 方法 | 路径 | 说明 |
|------|------|------|
| GET | `/social/follow` | 关注列表（`follower_id` / `following_id` 筛选）|
| POST | `/social/follow/:id` | 关注用户 |
| DELETE | `/social/follow/:id` | 取关用户 |
| POST | `/social/report` | 举报内容 |

### 私聊 `/conversation`

| 方法 | 路径 | 说明 |
|------|------|------|
| GET | `/conversation` | 获取我的会话列表 |
| POST | `/conversation` | 创建或恢复会话 |
| GET | `/conversation/:id` | 获取单个会话 |
| DELETE/POST | `/conversation/:id` | 软删除会话（置空一方）|
| GET | `/conversation/:id/messages` | 获取会话消息 |

### 群组 `/group`

| 方法 | 路径 | 说明 |
|------|------|------|
| GET | `/group` | 我参与的群列表 |
| POST | `/group` | 创建群组 |
| GET | `/group/:id` | 获取群信息 |
| PUT | `/group/:id` | 更新群名/头像（管理员）|
| DELETE | `/group/:id` | 解散群组（管理员）|
| POST | `/group/:id` | 邀请/踢出/退出成员 |
| GET | `/group/:id/members` | 群成员列表 |
| GET | `/group/:id/message` | 获取群消息 |
| POST | `/group/:id/message` | 发送群消息 |

### 商品 `/product`

| 方法 | 路径 | 说明 |
|------|------|------|
| GET | `/product` | 商品列表（分类/价格/关键词/地理范围筛选）|
| POST | `/product` | 发布商品 |
| GET | `/product/:id` | 获取商品详情 |
| POST/PUT | `/product/:id` | 更新商品 |
| DELETE | `/product/:id` | 删除商品 |
| GET | `/product/:id/favorite` | 查询是否已收藏 |
| POST | `/product/:id/favorite` | 收藏商品 |
| DELETE | `/product/:id/favorite` | 取消收藏 |

### 帖子 `/post`

| 方法 | 路径 | 说明 |
|------|------|------|
| GET | `/post` | 帖子列表（分类/作者/关键词筛选）|
| POST | `/post` | 发布帖子 |
| GET | `/post/:id` | 获取帖子详情 |
| PUT | `/post/:id` | 更新帖子 |
| DELETE | `/post/:id` | 删除帖子 |
| GET | `/post/:id/comments` | 获取评论（含回复和用户信息）|
| POST | `/post/:id/comments` | 发表评论 |
| DELETE/POST | `/post/:id/comments/:comment_id` | 删除评论 |
| POST | `/post/:id/reply` | 回复评论 |
| GET | `/post/comment/:comment_id/replies` | 获取某条评论的所有回复 |
| GET | `/post/:id/favorite` | 查询是否已收藏 |
| POST | `/post/:id/favorite` | 收藏帖子 |
| DELETE | `/post/:id/favorite` | 取消收藏 |

### 分类 `/category`

| 方法 | 路径 | 说明 |
|------|------|------|
| GET | `/category` | 获取所有分类 |

### WebSocket `/ws`

连接后第一条消息发送 JWT token（JSON 字符串）进行鉴权，之后收发消息格式：

```jsonc
// 发送消息
{ "type": "SEND_MESSAGE", "targetId": "uuid", "isGroup": false, "content": "hello", "msgType": "TEXT", "replyId": null }

// 撤回消息
{ "type": "RECALL_MESSAGE", "messageId": "uuid" }
```

---

## 数据库 Schema

17 张表，主要关系如下：

```
users ──< follows
users ──< conversations >── messages
users ──< groups >── group_members
                  └── group_messages
users ──< posts >── post_likes
                 ├── post_comments >── comment_likes
                 │                 └── comment_replies
                 └── post_favorites
users ──< products >── product_favorites
categories
reports
```

详见 `migrations/001_init.sql`。

---

## 开发记录

| 版本 | 说明 |
|------|------|
| v0.1 | 初始化 Axum 后端，用户注册/登录 JWT 鉴权 |
| v0.2 | 实现群组、私聊、消息、关注等核心社交功能 |
| v0.3 | 添加商品、帖子模块 |
| v0.4 | **将 JSON 文件存储全面迁移至 PostgreSQL（sqlx）** |
