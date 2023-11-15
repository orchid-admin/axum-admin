# axum-admin

[English](https://github.com/orchid-admin/axum-admin/blob/main/README.md)&nbsp;&nbsp;
[中文简体](https://github.com/orchid-admin/axum-admin/blob/main/README.zh-hans.md)&nbsp;&nbsp;

`axum-admin` is a backend management system server implementation developed based on [axum](https://github.com/tokio-rs/axum) + [prisma-client-rust](https://github.com/Brendonovich/prisma-client-rust).

## Instructions

1. Install rust environment

2. Generate prisma client and create database tables

   ```rust
   cargo prisma db push
   ```

3. Create super administrator role, account and initialization menu data

   ```rust
   cargo cli init <DEPT_NAME> <USERNAME_PASSWORD>
   ```

4. Run Admin Gateway

   ```rust
   cargo run -p admin
   ```

## function list

- [x] Username + Password login
- [x] Menu management
- [x] Role management
- [x] Department management
- [x] Administrator management
- [x] Dictionary management
- [x] Login log management
- [x] Operation log management
- [x] User Management
- [x] Caching system（Memory/Database）

## Front-end implementation list

- [orchid-admin-vue3](https://github.com/orchid-admin/orchid-admin-vue3): Based on [vue-next-admin](https://gitee.com/lyt-top/vue-next-admin) secondary development.
  - Technology stack: vue3.x + CompositionAPI setup syntax sugar + typescript + vite + element plus + vue-router-next + pinia.
  - Adaptation: mobile phones, tablets, PCs
