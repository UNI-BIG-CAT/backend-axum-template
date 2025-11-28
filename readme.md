# 基于 Axum + SeaORM 的 Rust 后端项目模板

一个功能完整、开箱即用的 Rust 后端项目模板，基于 Axum Web 框架和 SeaORM ORM，提供了现代化的后端开发所需的核心功能。

## ✨ 主要特性

- 🚀 **高性能 Web 框架**: 基于 Axum，提供异步、类型安全的 HTTP 服务
- 🗄️ **ORM 支持**: 使用 SeaORM 进行数据库操作，支持 PostgreSQL
- 🔐 **JWT 认证**: 基于 RSA 密钥的 JWT 令牌认证系统
- 📦 **Redis 缓存**: 集成 Redis 支持，提供高效的缓存服务
- 📝 **结构化日志**: 基于 tracing 的日志系统，支持日志级别和文件输出
- 🛡️ **中间件支持**: 内置认证、CORS、错误处理、日志等中间件
- ⚙️ **配置管理**: 基于 YAML 的多环境配置（开发/生产）
- ✅ **数据验证**: 使用 validator 进行请求参数验证
- 📊 **错误码管理**: 统一的错误码管理系统
- 🏗️ **清晰架构**: 采用 Controller-Service-Repository 分层架构

## 🛠️ 技术栈

- **Web 框架**: Axum 0.8.4
- **ORM**: SeaORM 1.1.19 (PostgreSQL)
- **异步运行时**: Tokio
- **缓存**: Redis (deadpool-redis)
- **认证**: JWT (jsonwebtoken + RSA)
- **日志**: tracing + tracing-subscriber
- **序列化**: serde + serde_json
- **配置**: serde_yaml
- **验证**: validator
- **其他**: uuid, chrono, anyhow, utoipa

## 📁 项目结构

```
backend-axum-template/
├── config/                 # 配置文件目录
│   ├── develop.yaml        # 开发环境配置
│   ├── production.yaml     # 生产环境配置
│   └── errcodes/          # 错误码定义
├── src/
│   ├── config/            # 配置模块
│   ├── constant/          # 常量定义
│   ├── controllers/       # 控制器层（路由处理）
│   ├── service/           # 业务逻辑层
│   ├── repository/        # 数据访问层
│   │   ├── entity/        # 数据库实体（由 SeaORM 生成）
│   │   ├── redis/         # Redis 操作
│   │   └── sql/           # SQL 脚本
│   ├── dto/               # 数据传输对象
│   ├── middleware/        # 中间件
│   │   ├── app_middleware.rs  # 认证、日志等中间件
│   │   ├── app_response.rs    # 统一响应格式
│   │   └── app_state.rs       # 应用状态
│   ├── logging/           # 日志初始化
│   └── main.rs            # 应用入口
└── Cargo.toml             # 项目依赖配置
```

## 🚀 快速开始

### 环境要求

- Rust 1.70+ (推荐使用 rustup 安装)
- PostgreSQL 数据库
- Redis 服务器
- OpenSSL (用于生成 RSA 密钥)

### 安装步骤

1. **克隆项目**
```bash
git clone <repository-url>
cd backend-axum-template
```

2. **配置环境变量**
```bash
# 设置运行环境（dev 或 pro）
export APP_ENV=dev  # Windows: set APP_ENV=dev
```

3. **配置数据库连接**
编辑 `config/develop.yaml` 或 `config/production.yaml`，修改数据库和 Redis 配置：
```yaml
postgres:
  ip: 127.0.0.1
  port: 5432
  db: template_db
  admin: template
  password: template123

main_redis:
  ip: 127.0.0.1
  port: 6380
  auth: your_redis_password
  db: 1
```

4. **生成 RSA 密钥对**
```bash
# 生成密码加密密钥对
openssl genpkey -algorithm RSA -out config/key/password-private-key.pem -pkeyopt rsa_keygen_bits:2048
openssl rsa -in config/key/password-private-key.pem -pubout -out config/key/password-public-key.pem

# 生成 JWT 签名密钥对
openssl genpkey -algorithm RSA -out config/key/jwt-private-key.pem -pkeyopt rsa_keygen_bits:2048
openssl rsa -pubout -in config/key/jwt-private-key.pem -out config/key/jwt-public-key.pem
```

5. **生成数据库实体**
```bash
# 设置数据库连接环境变量
export DATABASE_URL=postgres://template:template123@localhost/template_db

# 生成实体文件
sea-orm-cli generate entity --with-serde both --model-extra-attributes 'serde(rename_all = "camelCase")' -o ./src/repository/entity
```

6. **运行项目**
```bash
cargo run
```

项目将在 `http://0.0.0.0:3000` 启动（端口可在配置文件中修改）。

## 📖 使用说明

### 配置说明

项目支持多环境配置，通过 `APP_ENV` 环境变量切换：
- `dev` 或未设置: 使用 `config/develop.yaml`
- `pro`: 使用 `config/production.yaml`

### API 路由

项目采用分层路由设计，支持：
- **公开路由**: 无需认证即可访问（如注册、登录）
- **受保护路由**: 需要 JWT 认证（通过 `auth_middleware`）

示例路由结构：
```rust
// 公开路由
POST /admin/register      # 用户注册
GET  /admin/activeEmailCode  # 激活邮箱验证码
POST /admin/login         # 用户登录

// 需要认证的路由
GET  /admin/my           # 获取当前用户信息
POST /admin/logout       # 用户登出
```

### 中间件

项目内置以下中间件（按执行顺序）：
1. **日志中间件**: 记录所有请求日志
2. **Trace 中间件**: 请求追踪
3. **CORS 中间件**: 跨域资源共享
4. **错误处理中间件**: 统一错误响应格式
5. **认证中间件**: JWT 令牌验证（仅受保护路由）

### 数据库实体生成

使用 SeaORM CLI 从数据库生成实体：

```bash
# 设置数据库连接
export DATABASE_URL=postgres://user:password@localhost/dbname

# 生成实体（支持 serde 序列化，使用 camelCase 命名）
sea-orm-cli generate entity \
  --with-serde both \
  --model-extra-attributes 'serde(rename_all = "camelCase")' \
  -o ./src/repository/entity
```

### 开发指南

1. **添加新路由**:
   - 在 `src/controllers/` 创建新的控制器文件
   - 在 `src/controllers/mod.rs` 中注册路由
   - 在 `src/service/` 中实现业务逻辑

2. **添加新服务**:
   - 在 `src/service/` 创建服务文件
   - 在 `src/service/mod.rs` 中导出模块

3. **数据库操作**:
   - 使用 SeaORM 进行数据库操作
   - 实体文件位于 `src/repository/entity/`
   - 数据库连接通过 `AppState` 传递

4. **错误处理**:
   - 使用 `ApiResponse` 统一响应格式
   - 错误码定义在 `config/errcodes/` 目录

## 📝 开发工具

- **SeaORM CLI**: 数据库迁移和实体生成
- **OpenSSL**: RSA 密钥生成
- **Cargo**: Rust 包管理和构建工具

## 🔒 安全特性

- RSA 加密的密码存储
- JWT 令牌认证
- 请求参数验证
- CORS 配置
- 统一的错误处理（避免信息泄露）

## 📄 许可证

查看 [LICENSE](LICENSE) 文件了解详情。

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！

---

**注意**: 这是一个项目模板，使用前请根据实际需求修改配置和代码。
