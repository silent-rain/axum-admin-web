# PostgreSQL安装

## 安装

```shell
# pgvector 向量数据库插件
sudo pacman -S postgresql
yay -S pgvector
```

## 设置用户密码

安装时系统会默认创建一个系统用户 postgres，密码为空，设置密码：

```shell
sudo passwd postgres
```

## 初始化数据库

```shell
sudo su - postgres -c "initdb --locale en_US.UTF-8 -E UTF8 -D '/var/lib/postgres/data'"
```

## 启动数据库

```shell
# 启动数据库
sudo systemctl start postgresql.service

# 设置开机自动启动
sudo systemctl enable postgresql.service
```

## 登录

### 本机用户登录

在数据库中创建同名的用户和数据库

```shell
➜  ~ su - postgres
Password: 

[postgres@one ~]$ psql
psql (16.3)
Type "help" for help.

postgres=# 
```

### psql 登录

```text
psql -h 127.0.0.1 -p 5432 -U one -d postgres

-h   数据库IP
-p   数据库端口
-U   用户名
-d   需要访问的数据库名称
```

## 创建数据库新用户

```shell
postgres=# create user one with password 'mypass';
CREATE ROLE
```

```shell
postgres=# CREATE ROLE one LOGIN PASSWORD 'mypass';
CREATE ROLE
```

## 创建用户数据库

```shell
postgres=# CREATE DATABASE axum_admin_web OWNER one;
```

## 查看已经存在的数据库

```sql
postgres=# \l
```

## 进入数据库

```shell
postgres=# \c axum_admin_web


# or

# 系统命令行进入
psql -h localhost -p 5432 -U one axum_admin_web
```
