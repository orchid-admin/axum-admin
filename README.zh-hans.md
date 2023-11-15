# axum-admin

[English](https://github.com/orchid-admin/axum-admin/blob/main/README.md)&nbsp;&nbsp;
[中文简体](https://github.com/orchid-admin/axum-admin/blob/main/README.zh-hans.md)&nbsp;&nbsp;

`axum-admin` 是基于[axum](https://github.com/tokio-rs/axum) + [prisma-client-rust](https://github.com/Brendonovich/prisma-client-rust). 开发的后台管理系统服务端实现。

## 使用方法

1. 安装 rust 环境

2. 生成 prisma 客户端并创建数据库表

   ```rust
   cargo prisma db push
   ```

3. 创建超级管理员角色、账号和初始化菜单数据

   ```rust
   cargo cli init <DEPT_NAME> <USERNAME_PASSWORD>
   ```

   - DEPT_NAME 部门名称
   - USERNAME_PASSWORD 超级管理员密码

4. 运行

   ```rust
   cargo run -p admin
   ```

## 功能列表

- [x] 用户名 + 密码登录
- [x] 菜单管理
- [x] 角色管理
- [x] 部门管理
- [x] 管理员管理
- [x] 字典管理
- [x] 登录日志管理
- [x] 操作日志管理
- [x] 用户管理
- [x] 缓存系统（Memory/Database）

## 前端实现列表

- [orchid-admin-vue3](https://github.com/orchid-admin/orchid-admin-vue3)：基于[vue-next-admin](https://gitee.com/lyt-top/vue-next-admin) 二次开发。
  - 技术栈：vue3.x + CompositionAPI setup + typescript + vite + element plus + vue-router-next + pinia 技术.
  - 适配：手机、平板、pc
