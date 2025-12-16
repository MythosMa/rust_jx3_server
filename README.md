## 学习rust用的剑网三api调用，用来制作自己的剑网三相关查询app

### 项目结构
```
my_rust_server/
  ├── Cargo.toml               # 项目配置文件
  ├── .env                      # 环境变量
  ├── src/
  │   ├── config.rs             # 配置管理
  │   ├── db.rs                 # 数据库连接
  │   ├── main.rs               # 启动文件
  │   ├── routes/               # 路由模块
  │   │   ├── mod.rs            # 路由注册
  │   │   ├── api.rs            # API 路由
  │   ├── services/             # 业务逻辑层
  │   │   └── api_service.rs   # API 业务逻辑
  │   └── models/               # 数据模型层
  │       └── api.rs           # API 模型
  └── Dockerfile                # Docker 配置
```
