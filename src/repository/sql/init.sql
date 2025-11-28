-- Role Table
CREATE TABLE IF NOT EXISTS public.tb_role (
    "role_id" INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    "role_name" VARCHAR(20) NOT NULL DEFAULT 0 UNIQUE,
    "right" SMALLINT NOT NULL DEFAULT 2,
    "enabled" SMALLINT NOT NULL DEFAULT 1,
    "remark" VARCHAR(1024) NOT NULL DEFAULT '',
    "created_time" TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    "updated_time" TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    "delete_flag" SMALLINT NOT NULL DEFAULT 0
);

ALTER TABLE public.tb_role OWNER TO "radar";

CREATE TRIGGER "update_tb_role_timestamp" BEFORE UPDATE ON public.tb_role
FOR EACH ROW
EXECUTE PROCEDURE "update_timestamp"();

COMMENT ON COLUMN public.tb_role."role_name" IS '角色名称';
COMMENT ON COLUMN public.tb_role."right" IS '权限 0 超级管理员 1 读写用户 2 只读用户';
COMMENT ON COLUMN public.tb_role."enabled" IS '是否启用:0 否 1 是';
COMMENT ON COLUMN public.tb_role."delete_flag" IS '删除标记:0 否 1 是';
COMMENT ON TABLE public.tb_role IS '角色表';

INSERT INTO public.tb_role ("role_name", "right", "enabled", "remark", "delete_flag") VALUES ('超级管理员', 0, 1, '超级管理员', 0);
INSERT INTO public.tb_role ("role_name", "right", "enabled", "remark", "delete_flag") VALUES ('读写用户', 1, 1, '读写用户', 0);
INSERT INTO public.tb_role ("role_name", "right", "enabled", "remark", "delete_flag") VALUES ('只读用户', 2, 1, '只读', 0);


-- Admin Table
CREATE TABLE IF NOT EXISTS public.tb_admin (
  "admin_id" INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  "role_id" int4 NOT NULL DEFAULT 3,
  "admin_name" VARCHAR(20) NOT NULL DEFAULT '' UNIQUE,
  "password" VARCHAR(100) NOT NULL DEFAULT '',
  "email" VARCHAR(50) NOT NULL DEFAULT '' UNIQUE,
  "phone" VARCHAR(20) NOT NULL DEFAULT '' UNIQUE,
  "remark" VARCHAR(1024) NOT NULL DEFAULT '',
  "enabled" SMALLINT NOT NULL DEFAULT 0,
  "last_login_time" TIMESTAMP,
  "created_time" TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  "updated_time" TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  "delete_flag" SMALLINT NOT NULL  DEFAULT 0
);

ALTER TABLE public.tb_admin 
  OWNER TO "radar";

CREATE TRIGGER "update_tb_admin_timestamp" BEFORE UPDATE ON public.tb_admin
FOR EACH ROW
EXECUTE PROCEDURE "update_timestamp"();

COMMENT ON COLUMN public.tb_admin."admin_id" IS 'admin_id';
COMMENT ON COLUMN public.tb_admin."role_id" IS '角色id';
COMMENT ON COLUMN public.tb_admin."admin_name" IS '管理员名称';
COMMENT ON COLUMN public.tb_admin."password" IS '密码';
COMMENT ON COLUMN public.tb_admin."email" IS 'email';
COMMENT ON COLUMN public.tb_admin."phone" IS '电话';
COMMENT ON COLUMN public.tb_admin."remark" IS '备注';
COMMENT ON COLUMN public.tb_admin."enabled" IS '是否启用 0 否 1 是';
COMMENT ON COLUMN public.tb_admin."last_login_time" IS '最后登录时间';
COMMENT ON COLUMN public.tb_admin."created_time" IS '创建时间';
COMMENT ON COLUMN public.tb_admin."updated_time" IS '更新时间';
COMMENT ON COLUMN public.tb_admin."delete_flag" IS '是否删除: 0 否 1 是';

INSERT INTO public.tb_admin ("role_id", "admin_name", "password", "email", "phone", "remark", "enabled") VALUES (1, 'root', '', '00@00.com', '000000', '超级管理员', 1);

-- Project Table
CREATE TABLE IF NOT EXISTS public.tb_project (
  "project_id" INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  "project_name" VARCHAR(20) NOT NULL DEFAULT '',
  "unit" VARCHAR(50) NOT NULL DEFAULT '',
  "address" VARCHAR(50) NOT NULL DEFAULT '',
  "type" int4 NOT NULL DEFAULT 0,
  "introduction" VARCHAR(100) NOT NULL DEFAULT '',
  "content" VARCHAR(1024) NOT NULL DEFAULT '',
  "creator" VARCHAR(50) NOT NULL DEFAULT '',
  "creator_contact" VARCHAR(50) NOT NULL DEFAULT '',
  "displacement_speed_horizontal_threshold" numeric(10, 3) NOT NULL DEFAULT 0,
  "displacement_speed_vertical_threshold" numeric(10, 3) NOT NULL DEFAULT 0,
  "convergence_threshod" numeric(10, 3) NOT NULL DEFAULT 0,
  "fundamental_frequency_threshod" numeric(10, 3) NOT NULL DEFAULT 0,
  "disturbance_threshod" numeric(10, 3) NOT NULL DEFAULT 0,
  "cumulative_displacement_threshod" numeric(10, 3) NOT NULL DEFAULT 0,
  "collection_frequency" numeric(10, 3) NOT NULL DEFAULT 0,
  "status" SMALLINT NOT NULL DEFAULT 0,
  "created_time" TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  "updated_time" TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  "delete_flag" SMALLINT NOT NULL  DEFAULT 0
);

ALTER TABLE public.tb_project 
  OWNER TO "radar";

CREATE TRIGGER "update_tb_project_timestamp" BEFORE UPDATE ON public.tb_project
FOR EACH ROW
EXECUTE PROCEDURE "update_timestamp"();

COMMENT ON COLUMN public.tb_project."project_id" IS 'project_id';
COMMENT ON COLUMN public.tb_project."project_name" IS '项目名称';
COMMENT ON COLUMN public.tb_project."unit" IS '归属单位';
COMMENT ON COLUMN public.tb_project."address" IS '项目所在地';
COMMENT ON COLUMN public.tb_project."type" IS '项目类型';
COMMENT ON COLUMN public.tb_project."introduction" IS '项目简介';
COMMENT ON COLUMN public.tb_project."content" IS '监测内容';
COMMENT ON COLUMN public.tb_project."creator" IS '创建人';
COMMENT ON COLUMN public.tb_project."creator_contact" IS '创建人联系方式';
COMMENT ON COLUMN public.tb_project."displacement_speed_horizontal_threshold" IS '位移速度阈值水平(单位mm)';
COMMENT ON COLUMN public.tb_project."displacement_speed_vertical_threshold" IS '位移速度阈值垂直(单位mm)';
COMMENT ON COLUMN public.tb_project."convergence_threshod" IS '收敛(单位mm)';
COMMENT ON COLUMN public.tb_project."fundamental_frequency_threshod" IS '基频(单位hz)';
COMMENT ON COLUMN public.tb_project."disturbance_threshod" IS '扰度(单位mm)';
COMMENT ON COLUMN public.tb_project."cumulative_displacement_threshod" IS '累积位移阈值(单位mm/天)';
COMMENT ON COLUMN public.tb_project."collection_frequency" IS '采集频率(分钟)';
COMMENT ON COLUMN public.tb_project."status" IS ' 状态 0-未激活 1-已经激活 2-归档';
COMMENT ON COLUMN public.tb_project."created_time" IS '创建时间';
COMMENT ON COLUMN public.tb_project."updated_time" IS '更新时间';
COMMENT ON COLUMN public.tb_project."delete_flag" IS '是否删除: 0 否 1 是';

-- Device Model Table
CREATE TABLE IF NOT EXISTS public.tb_device_model (
  "device_model_id" INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  "device_model_no" VARCHAR(20) NOT NULL DEFAULT '',
  "device_model_name" VARCHAR(20) NOT NULL DEFAULT '',
  "status" SMALLINT NOT NULL DEFAULT 1,
  "created_time" TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  "updated_time" TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  "delete_flag" SMALLINT NOT NULL  DEFAULT 0
);

ALTER TABLE public.tb_device_model 
  OWNER TO "radar";

CREATE TRIGGER "update_tb_device_model_timestamp" BEFORE UPDATE ON public.tb_device_model
FOR EACH ROW
EXECUTE PROCEDURE "update_timestamp"();

COMMENT ON COLUMN public.tb_device_model."device_model_id" IS '序号';
COMMENT ON COLUMN public.tb_device_model."device_model_no" IS '设备型号编号';
COMMENT ON COLUMN public.tb_device_model."device_model_name" IS '设备型号名称';
COMMENT ON COLUMN public.tb_device_model."status" IS '设备状态 0-禁用 1-启用';
COMMENT ON COLUMN public.tb_device_model."created_time" IS '创建时间';
COMMENT ON COLUMN public.tb_device_model."updated_time" IS '更新时间';
COMMENT ON COLUMN public.tb_device_model."delete_flag" IS '是否删除: 0 否 1 是';

-- Device Table
CREATE TABLE IF NOT EXISTS public.tb_device (
  "device_id" INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  "device_name" VARCHAR(20) NOT NULL DEFAULT '',
  "device_no" VARCHAR(20) NOT NULL DEFAULT '' UNIQUE,
  "project_id" int4 NOT NULL DEFAULT 0,
  "address" VARCHAR(300) NOT NULL DEFAULT '' ,
  "ip_address" VARCHAR(20) NOT NULL DEFAULT '' ,
  "device_model_id" int4 NOT NULL DEFAULT 0,
  "installation_height" numeric(10, 2) NOT NULL DEFAULT 0,
  "pitch_angle" numeric(10, 1) NOT NULL DEFAULT 0,
  "azimuth_angle" numeric(10, 1) NOT NULL DEFAULT 0,
  "abnormal_offline_duration" INT NOT NULL DEFAULT 0,
  "map_url" VARCHAR(300) NOT NULL DEFAULT '',
  "sampling_frequency" numeric(10, 2) NOT NULL DEFAULT 0,
  "polarization_mode" numeric(10, 2) NOT NULL DEFAULT 0,
  "shaking_amplitude" numeric(10, 2) NOT NULL DEFAULT 0,
  "rssi" numeric(10, 2) NOT NULL DEFAULT 0,
  "status" SMALLINT NOT NULL DEFAULT 0,
  "created_time" TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  "updated_time" TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  "delete_flag" SMALLINT NOT NULL  DEFAULT 0
);

ALTER TABLE public.tb_device 
  OWNER TO "radar";

CREATE TRIGGER "update_tb_device_timestamp" BEFORE UPDATE ON public.tb_device
FOR EACH ROW
EXECUTE PROCEDURE "update_timestamp"();

COMMENT ON COLUMN public.tb_device."device_id" IS '序号';
COMMENT ON COLUMN public.tb_device."device_name" IS '设备名称';
COMMENT ON COLUMN public.tb_device."device_no" IS '设备编号';
COMMENT ON COLUMN public.tb_device."project_id" IS '所属项目(只能属于一个未归档项目)';
COMMENT ON COLUMN public.tb_device."address" IS '安装地址';
COMMENT ON COLUMN public.tb_device."ip_address" IS 'ip地址';
COMMENT ON COLUMN public.tb_device."device_model_id" IS '设备型号id';
COMMENT ON COLUMN public.tb_device."installation_height" IS '安装高度 单位(米 精度0.01)';
COMMENT ON COLUMN public.tb_device."pitch_angle" IS '俯仰角  单位(度 精度0.1)';
COMMENT ON COLUMN public.tb_device."azimuth_angle" IS '方位角 单位(度 精度0.1)';
COMMENT ON COLUMN public.tb_device."abnormal_offline_duration" IS '异常离线时长 单位(分钟)';
COMMENT ON COLUMN public.tb_device."map_url" IS '设备地图地址';
COMMENT ON COLUMN public.tb_device."sampling_frequency" IS '采样频率 单位(分钟)';
COMMENT ON COLUMN public.tb_device."polarization_mode" IS '极化方式';
COMMENT ON COLUMN public.tb_device."shaking_amplitude" IS '晃动幅度 单位(度 精度0.01)';
COMMENT ON COLUMN public.tb_device."rssi" IS '信号强度';
COMMENT ON COLUMN public.tb_device."status" IS '设备状态 0-离线 1-在线';
COMMENT ON COLUMN public.tb_device."created_time" IS '创建时间';
COMMENT ON COLUMN public.tb_device."updated_time" IS '更新时间';
COMMENT ON COLUMN public.tb_device."delete_flag" IS '是否删除: 0 否 1 是';

-- Device Record Table
CREATE TABLE IF NOT EXISTS public.tb_device_record (
  "device_record_id" INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  "device_id" int4 NOT NULL DEFAULT 0,
  "device_no" VARCHAR(20) NOT NULL DEFAULT '',
  "content" text NOT NULL DEFAULT '',
  "created_time" TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  "updated_time" TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  "delete_flag" SMALLINT NOT NULL  DEFAULT 0
);

ALTER TABLE public.tb_device_record 
  OWNER TO "radar";

CREATE TRIGGER "update_tb_device_record_timestamp" BEFORE UPDATE ON public.tb_device_record
FOR EACH ROW
EXECUTE PROCEDURE "update_timestamp"();

COMMENT ON COLUMN public.tb_device_record."device_record_id" IS '序号';
COMMENT ON COLUMN public.tb_device_record."device_id" IS '设备id';
COMMENT ON COLUMN public.tb_device_record."device_no" IS '设备编号';
COMMENT ON COLUMN public.tb_device_record."content" IS '内容';
COMMENT ON COLUMN public.tb_device_record."created_time" IS '创建时间';
COMMENT ON COLUMN public.tb_device_record."updated_time" IS '更新时间';
COMMENT ON COLUMN public.tb_device_record."delete_flag" IS '是否删除: 0 否 1 是';

-- Check Point Table
CREATE TABLE IF NOT EXISTS public.tb_check_point (
  "check_point_id" INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  "check_point_no" VARCHAR(20) NOT NULL DEFAULT '',
  "check_point_name" VARCHAR(20) NOT NULL DEFAULT '',
  "horizontal_spacing" numeric(10, 3) NOT NULL DEFAULT 0,
  "height_difference" numeric(10, 3) NOT NULL DEFAULT 0,
  "created_time" TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  "updated_time" TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  "delete_flag" SMALLINT NOT NULL  DEFAULT 0
);

ALTER TABLE public.tb_check_point 
  OWNER TO "radar";

CREATE TRIGGER "update_tb_check_point_timestamp" BEFORE UPDATE ON public.tb_check_point
FOR EACH ROW
EXECUTE PROCEDURE "update_timestamp"();

COMMENT ON COLUMN public.tb_check_point."check_point_id" IS '监测点id';
COMMENT ON COLUMN public.tb_check_point."check_point_no" IS '监测点编号';
COMMENT ON COLUMN public.tb_check_point."check_point_name" IS '监测点名称';
COMMENT ON COLUMN public.tb_check_point."horizontal_spacing" IS '监测水平间距';
COMMENT ON COLUMN public.tb_check_point."height_difference" IS '高层差';
COMMENT ON COLUMN public.tb_check_point."created_time" IS '创建时间';
COMMENT ON COLUMN public.tb_check_point."updated_time" IS '更新时间';
COMMENT ON COLUMN public.tb_check_point."delete_flag" IS '是否删除: 0 否 1 是';

-- Device Check Point Table
CREATE TABLE IF NOT EXISTS public.tb_device_check_point (
  "device_check_point_id" INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  "device_id" int4 NOT NULL DEFAULT 0,
  "check_point_id" int4 NOT NULL DEFAULT 0,
  "created_time" TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  "updated_time" TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  "delete_flag" SMALLINT NOT NULL  DEFAULT 0
);

ALTER TABLE public.tb_device_check_point 
  OWNER TO "radar";

CREATE TRIGGER "update_tb_device_check_point_timestamp" BEFORE UPDATE ON public.tb_device_check_point
FOR EACH ROW
EXECUTE PROCEDURE "update_timestamp"();

COMMENT ON COLUMN public.tb_device_check_point."device_check_point_id" IS '监测点id';
COMMENT ON COLUMN public.tb_device_check_point."device_id" IS '设备id';
COMMENT ON COLUMN public.tb_device_check_point."check_point_id" IS '监测点id';
COMMENT ON COLUMN public.tb_device_check_point."created_time" IS '创建时间';
COMMENT ON COLUMN public.tb_device_check_point."updated_time" IS '更新时间';
COMMENT ON COLUMN public.tb_device_check_point."delete_flag" IS '是否删除: 0 否 1 是';

-- Check Content Table
CREATE TABLE IF NOT EXISTS public.tb_check_content (
  "check_content_id" INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  "check_content_key" VARCHAR(50) NOT NULL DEFAULT '',
  "check_content_name" VARCHAR(50) NOT NULL DEFAULT '',
  "enabled" SMALLINT NOT NULL  DEFAULT 1,
  "created_time" TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  "updated_time" TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  "delete_flag" SMALLINT NOT NULL  DEFAULT 0
);

ALTER TABLE public.tb_check_content 
  OWNER TO "radar";

CREATE TRIGGER "update_tb_check_content_timestamp" BEFORE UPDATE ON public.tb_check_content
FOR EACH ROW
EXECUTE PROCEDURE "update_timestamp"();

COMMENT ON COLUMN public.tb_check_content."check_content_id" IS '监测内容id';
COMMENT ON COLUMN public.tb_check_content."check_content_key" IS '监测内容key';
COMMENT ON COLUMN public.tb_check_content."check_content_name" IS '监测内容名称';
COMMENT ON COLUMN public.tb_admin."enabled" IS '是否启用 0 否 1 是';
COMMENT ON COLUMN public.tb_check_content."created_time" IS '创建时间';
COMMENT ON COLUMN public.tb_check_content."updated_time" IS '更新时间';
COMMENT ON COLUMN public.tb_check_content."delete_flag" IS '是否删除: 0 否 1 是';

INSERT INTO "public"."tb_check_content" ("check_content_key", "check_content_name", "enabled", "delete_flag") VALUES ('radial_horizontal_displacement', '径向水平位移', 1, 0);
INSERT INTO "public"."tb_check_content" ("check_content_key", "check_content_name", "enabled", "delete_flag") VALUES ('vertical_displacement', '垂直位移', 0, 0);
INSERT INTO "public"."tb_check_content" ("check_content_key", "check_content_name", "enabled", "delete_flag") VALUES ('tunnel_convergence', '隧道收敛', 0, 0);
INSERT INTO "public"."tb_check_content" ("check_content_key", "check_content_name", "enabled", "delete_flag") VALUES ('fundamental_frequency', '基频', 0, 0);
INSERT INTO "public"."tb_check_content" ("check_content_key", "check_content_name", "enabled", "delete_flag") VALUES ('disturbance_degree', '扰动度', 0, 0);

-- Project Check Content Table
CREATE TABLE IF NOT EXISTS public.tb_project_check_content (
  "project_check_content_id" INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  "project" int4 NOT NULL DEFAULT 0,
  "check_content_id" int4 NOT NULL DEFAULT 0,
  "created_time" TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  "updated_time" TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  "delete_flag" SMALLINT NOT NULL  DEFAULT 0
);

ALTER TABLE public.tb_project_check_content 
  OWNER TO "radar";

CREATE TRIGGER "update_tb_project_check_content_timestamp" BEFORE UPDATE ON public.tb_project_check_content
FOR EACH ROW
EXECUTE PROCEDURE "update_timestamp"();

COMMENT ON COLUMN public.tb_project_check_content."project_check_content_id" IS '监测点id';
COMMENT ON COLUMN public.tb_project_check_content."project" IS '项目id';
COMMENT ON COLUMN public.tb_project_check_content."check_content_id" IS '检测内容id';
COMMENT ON COLUMN public.tb_project_check_content."created_time" IS '创建时间';
COMMENT ON COLUMN public.tb_project_check_content."updated_time" IS '更新时间';
COMMENT ON COLUMN public.tb_project_check_content."delete_flag" IS '是否删除: 0 否 1 是';


-- Alarm Type Table
CREATE TABLE IF NOT EXISTS public.tb_alarm_type (
  "alarm_type_id" INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  "alarm_type_key" VARCHAR(50) NOT NULL DEFAULT '',
  "alarm_type_name" VARCHAR(50) NOT NULL DEFAULT '',
  "enabled" SMALLINT NOT NULL  DEFAULT 1,
  "created_time" TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  "updated_time" TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  "delete_flag" SMALLINT NOT NULL  DEFAULT 0
);

ALTER TABLE public.tb_alarm_type 
  OWNER TO "radar";

CREATE TRIGGER "update_tb_alarm_type_timestamp" BEFORE UPDATE ON public.tb_alarm_type
FOR EACH ROW
EXECUTE PROCEDURE "update_timestamp"();

COMMENT ON COLUMN public.tb_alarm_type."alarm_type_id" IS '告警id';
COMMENT ON COLUMN public.tb_alarm_type."alarm_type_key" IS '告警key';
COMMENT ON COLUMN public.tb_alarm_type."alarm_type_name" IS '告警名称';
COMMENT ON COLUMN public.tb_admin."enabled" IS '是否启用 0 否 1 是';
COMMENT ON COLUMN public.tb_alarm_type."created_time" IS '创建时间';
COMMENT ON COLUMN public.tb_alarm_type."updated_time" IS '更新时间';
COMMENT ON COLUMN public.tb_alarm_type."delete_flag" IS '是否删除: 0 否 1 是';

INSERT INTO "public"."tb_alarm_type" ("alarm_type_key", "alarm_type_name", "enabled", "delete_flag") VALUES ('accumulation_alarm', '累积告警', 1, 0);