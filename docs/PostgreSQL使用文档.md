# PostgreSQL使用文档

## 建表示例

```sql
CREATE TABLE public.t_app_template (
 id serial4 NOT NULL, -- 模板ID
 user_id int4 NOT NULL, -- 用户ID
 "desc" varchar(200) NULL, -- 描述信息
 status int2 DEFAULT 0 NOT NULL, -- 状态(0:停用,1:正常)
 created_at timestamp DEFAULT CURRENT_TIMESTAMP  NOT NULL, -- 创建时间
 updated_at timestamp DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP  NOT NULL, -- 更新时间
 CONSTRAINT t_app_template2_pk PRIMARY KEY (id),
 CONSTRAINT t_app_template2_unique UNIQUE (user_id)
);
CREATE INDEX t_app_template2_user_id_idx ON public.t_app_template USING btree (user_id);
COMMENT ON TABLE public.t_app_template IS '应用模板';

-- Column comments

COMMENT ON COLUMN public.t_app_template.id IS '模板ID';
COMMENT ON COLUMN public.t_app_template.user_id IS '用户ID';
COMMENT ON COLUMN public.t_app_template."desc" IS '描述信息';
COMMENT ON COLUMN public.t_app_template.status IS '状态(0:停用,1:正常)';
COMMENT ON COLUMN public.t_app_template.created_at IS '创建时间';
COMMENT ON COLUMN public.t_app_template.updated_at IS '更新时间';

-- Table Triggers

create trigger t_app_template_updated_at before
insert
    or
update
    on
    public.t_app_template for each row execute function update_timestamp_column();
```

## 触发器自动更新时间

```sql
-- 使用触发器（Trigger） 适用更新时间
-- 创建一个触发器函数，该函数将在INSERT或UPDATE操作时被调用。
CREATE OR REPLACE FUNCTION update_timestamp_column() RETURNS TRIGGER AS $$
BEGIN
NEW.created_at := current_timestamp;
RETURN NEW;
END;
$$ LANGUAGE plpgsql;


-- 创建一个触发器，将该触发器与您的表和INSERT或UPDATE操作关联起来。
CREATE TRIGGER t_app_template_updated_at
BEFORE INSERT OR UPDATE ON t_app_template
FOR EACH ROW
EXECUTE FUNCTION update_timestamp_column();
```

## 借助存储列的功能实现自动更新时间

- [Q2.PG里如何自动记录行版本的修改时间？](https://www.modb.pro/db/559116)

```sql
CREATE OR REPLACE FUNCTION public.wrapper_im_now(text)
RETURNS timestamp with time zone
LANGUAGE sql
IMMUTABLE
AS $function$  
  SELECT CURRENT_TIMESTAMP;
$function$;


CREATE TABLE public.t_app_template (
 id serial NOT NULL,
 user_id int NOT NULL,
 "desc" varchar(200) NULL,
 status int DEFAULT 0 NOT NULL,
 created_at timestamp DEFAULT CURRENT_TIMESTAMP NOT NULL,
 updated_at timestamp  GENERATED ALWAYS AS (wrapper_im_now("desc")) stored,
 CONSTRAINT t_app_template3_pk PRIMARY KEY (id)
);


t_app_template 表 desc 字段修改会触发自动更新存储列 updated_at，如果还有其它字段也希望触发，可以微调修改上面的接口定义。
```

## 变更

```sql
-- 设置默认值
ALTER TABLE public.t_app_template ALTER COLUMN created_at SET DEFAULT CURRENT_TIMESTAMP;

```

## 参考文档

- [PostgreSQL中如何实现时间字段自动更新](https://www.jianshu.com/p/f0396898f2dc)
- [【Postgresql】创建时间、更新时间数据库端自动实现更新,postgresql修改表id字段为自增,创建序列，查询全部序列，删除序列，生成序列创建sql语句](https://www.cnblogs.com/Chary/p/18499601)
