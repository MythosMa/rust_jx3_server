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

### 构建注意事项
1. 项目中使用了sqlx，构建时需要先运行 
```
cargo sqlx prepare
```
在根目录生成.sqlx目录，构建脚本会将其复制到docker中以完成离线构建

2. 项目中使用.env.development和.env.production两个环境变量文件，分别用于开发和生产环境，配置了端口、数据库地址和环境名称
```
ENV=development
DATABASE_URL=mysql://username:password@localhost:3306/rust_jx3_server
SERVER_PORT=3000
```
请根据实际情况添加和修改.env.development和.env.production文件中的内容