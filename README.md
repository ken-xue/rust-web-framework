# rust-web-framework
> 一款基于rust快速开发的web应用脚手架
# 技术选型
## 后端
- [axum](https://github.com/tokio-rs/axum)
- [redis](https://github.com/redis/redis)
- [mysql](https://github.com/mysql/mysql-server)
- [diesel](https://github.com/SeaQL/sea-orm)
## 前端
- [vue-vben-admin](https://vben.vvbin.cn/)
# 快速开始

## 配置数据库

## 生成模块

1. 生成schema
```bash
# 缺省全表
diesel print-schema > src/schema.rs
# 指定表名
diesel print-schema -o sys_user_of_role
```
2. 生成实体
```bash
diesel  
```

## 代码编译
```bash
cargo build
```

### 代码编译(正式环境)
```bash
cargo build --release
```

### 单元测试
```bash
cargo test
```

### 性能基准测试
```bash
cargo bench
```