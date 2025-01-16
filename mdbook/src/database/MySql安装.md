# MySql安装

## 安装

```shell
sudo pacman -S mysql mysql-clients
```

## 卸载

```shell
sudo pacman -Rnsc mysql mysql-clients

sudo rm -rf /var/lib/mysql
```

## 关闭密码验证组件

> sudo vim /etc/mysql/my.cnf

```shell
# 8.0 之前的版本
[mysqld]
validate_password.policy = 0
```

```shell
# 8.0 之后的版本
[mysqld]
# 关闭密码验证策略
disable_validate_password=ON
# 设置默认字符集
character-set-server=utf8mb4
collation-server=utf8mb4_unicode_ci
# 设置最大连接数
max_connections=250

# 设置服务器时区
default-time-zone='+08:00'
```

## 初始化Mysql数据库(自动) - 推荐

```shell
sudo mysql_secure_installation
```

## 初始化Mysql数据库(手动)

```shell
# 旧版本
sudo mysql_install_db --user=mysql --basedir=/usr --datadir=/var/lib/mysql

# 创建一个包含随机密码的数据目录
sudo mysqld --initialize --user=mysql --basedir=/usr --datadir=/var/lib/mysql

# 在没有密码的情况下初始化数据库（不推荐用于生产环境）
sudo mysqld --initialize-insecure --user=mysql --basedir=/usr --datadir=/var/lib/mysql
```

## 启动服务

```shell
sudo systemctl start mysqld.service
```

## 修改密码

MySql 从8.0开始修改密码有了变化，在user表加了字 authentication_string, 修改密码前先检查authentication_string是否为空

```sql
# 切换数据库
use mysql;

# 查看用户
select User, host, plugin, authentication_string from user;
 
-- authentication_string 如果不为空, 如果为空，直接修改
update user set authentication_string='' where user='root'; --将字段置为空

ALTER user 'root'@'localhost' IDENTIFIED BY 'root'; --修改密码为root

flush privileges;
```

## 创建用户

```sql
-- 切换数据库
use mysql;

-- 查看用户
select User, host, plugin from user;

-- 更改密码策略为 LOW 级别（允许使用简单的密码）
SET GLOBAL validate_password.policy = 0;

-- 创建用户
CREATE USER 'one'@'localhost' IDENTIFIED BY 'pass';

-- 指定IP创建用户
CREATE USER 'one'@'192.168.3.10' IDENTIFIED BY 'pass';

-- MySQL8报错：Public Key Retrieval is not allowed
ALTER USER 'one'@'%' IDENTIFIED WITH mysql_native_password BY 'pass';

-- 所有地址皆可以访问
CREATE USER 'one'@'%' IDENTIFIED BY 'pass';

-- 设置所有权限
GRANT ALL PRIVILEGES ON *.* TO 'one'@'%';

-- 设置指定DB权限
GRANT ALL PRIVILEGES ON database_name.* TO 'newuser'@'%';

-- 删除用户
DELETE FROM user WHERE user="one";

-- 更新密码
ALTER USER 'one'@'localhost' IDENTIFIED BY 'new_password';

-- 刷新权限表
FLUSH PRIVILEGES;

-- 退出
quit
```
