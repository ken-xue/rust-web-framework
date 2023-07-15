-- MySQL dump 10.13  Distrib 5.7.28, for macos10.14 (x86_64)
--
-- Host: mysql-sz.makeblock.com    Database: rwf
-- ------------------------------------------------------
-- Server version	5.7.36-log

/*!40101 SET @OLD_CHARACTER_SET_CLIENT=@@CHARACTER_SET_CLIENT */;
/*!40101 SET @OLD_CHARACTER_SET_RESULTS=@@CHARACTER_SET_RESULTS */;
/*!40101 SET @OLD_COLLATION_CONNECTION=@@COLLATION_CONNECTION */;
/*!40101 SET NAMES utf8 */;
/*!40103 SET @OLD_TIME_ZONE=@@TIME_ZONE */;
/*!40103 SET TIME_ZONE='+00:00' */;
/*!40014 SET @OLD_UNIQUE_CHECKS=@@UNIQUE_CHECKS, UNIQUE_CHECKS=0 */;
/*!40014 SET @OLD_FOREIGN_KEY_CHECKS=@@FOREIGN_KEY_CHECKS, FOREIGN_KEY_CHECKS=0 */;
/*!40101 SET @OLD_SQL_MODE=@@SQL_MODE, SQL_MODE='NO_AUTO_VALUE_ON_ZERO' */;
/*!40111 SET @OLD_SQL_NOTES=@@SQL_NOTES, SQL_NOTES=0 */;

--
-- Table structure for table `sys_captcha`
--

DROP TABLE IF EXISTS `sys_captcha`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `sys_captcha` (
  `uuid` char(36) NOT NULL COMMENT 'uuid',
  `code` varchar(6) NOT NULL COMMENT '验证码',
  `expire_time` datetime DEFAULT NULL COMMENT '过期时间',
  `id` bigint(20) NOT NULL AUTO_INCREMENT,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='系统验证码';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `sys_captcha`
--

LOCK TABLES `sys_captcha` WRITE;
/*!40000 ALTER TABLE `sys_captcha` DISABLE KEYS */;
/*!40000 ALTER TABLE `sys_captcha` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `sys_config`
--

DROP TABLE IF EXISTS `sys_config`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `sys_config` (
  `id` bigint(20) unsigned NOT NULL AUTO_INCREMENT COMMENT '主键',
  `uuid` varchar(32) DEFAULT NULL COMMENT '唯一标示做关联',
  `name` varchar(64) DEFAULT NULL COMMENT '名字',
  `config` text COMMENT '配置信息',
  `remark` varchar(128) DEFAULT NULL COMMENT '备注',
  `creator` varchar(64) DEFAULT NULL COMMENT '创建人',
  `modifier` varchar(64) DEFAULT NULL COMMENT '修改人',
  `gmt_create` datetime NOT NULL COMMENT '创建时间',
  `gmt_modified` datetime NOT NULL COMMENT '修改时间',
  `deleted` char(1) NOT NULL DEFAULT '0' COMMENT '逻辑删除',
  `deletable` char(1) NOT NULL DEFAULT '1' COMMENT '是否可删除',
  `editable` char(1) DEFAULT '1' COMMENT '是否可编辑',
  PRIMARY KEY (`id`),
  UNIQUE KEY `name` (`name`)
) ENGINE=InnoDB AUTO_INCREMENT=14 DEFAULT CHARSET=utf8mb4 COMMENT='系统配置表';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `sys_config`
--

LOCK TABLES `sys_config` WRITE;
/*!40000 ALTER TABLE `sys_config` DISABLE KEYS */;
INSERT INTO `sys_config` VALUES (1,'b8c1e673060c437ba0925f119538bdb6','DEFAULT_OBS','{\"type\":\"minio\",\"url\":\"http://192.168.100.22:30001\",\"username\":\"minioadmin\",\"password\":\"minioadmin\"}','缺省OBS实例配置',NULL,NULL,'2022-12-08 19:31:22','2022-12-08 19:31:20','0','0','1'),(2,'94d48125e9f246d3a5e999a277985270','BUILD_SERVER','{\n  \"hostname\":\"192.168.100.21\",\n  \"port\":\"22\",\n  \"username\":\"root\",\n  \"password\":\"123456\"\n}','构建服务器','','','2022-12-08 16:24:05','2022-12-08 16:24:05','0','0','1');
/*!40000 ALTER TABLE `sys_config` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `sys_dictionary`
--

DROP TABLE IF EXISTS `sys_dictionary`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `sys_dictionary` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT,
  `key` varchar(200) DEFAULT NULL COMMENT '查询的key',
  `value` text COMMENT '字典值',
  `fixed` tinyint(1) DEFAULT '1' COMMENT '是否可更改',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=latin1;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `sys_dictionary`
--

LOCK TABLES `sys_dictionary` WRITE;
/*!40000 ALTER TABLE `sys_dictionary` DISABLE KEYS */;
/*!40000 ALTER TABLE `sys_dictionary` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `sys_log`
--

DROP TABLE IF EXISTS `sys_log`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `sys_log` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT,
  `username` varchar(50) DEFAULT NULL COMMENT '用户名',
  `operation` varchar(50) DEFAULT NULL COMMENT '用户操作',
  `method` varchar(200) DEFAULT NULL COMMENT '请求方法',
  `params` varchar(5000) DEFAULT NULL COMMENT '请求参数',
  `execute_time` bigint(20) NOT NULL COMMENT '执行时长(毫秒)',
  `ip` varchar(64) DEFAULT NULL COMMENT 'IP地址',
  `occur_time` timestamp NULL DEFAULT NULL COMMENT '发生时间',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=11 DEFAULT CHARSET=utf8mb4 COMMENT='系统日志';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `sys_log`
--

LOCK TABLES `sys_log` WRITE;
/*!40000 ALTER TABLE `sys_log` DISABLE KEYS */;
INSERT INTO `sys_log` VALUES (1,'mikey','修改菜单','io.dev.admin.adapter.rest.sys.MenuController.update()','[{\"menuCO\":{\"creator\":\"admin\",\"deleted\":false,\"gmtCreate\":1638345181000,\"gmtModified\":1638345181000,\"id\":50,\"menuIcon\":\"excel\",\"menuName\":\"表单生成\",\"menuOrder\":0,\"menuParentName\":\"开发工具\",\"menuParentUuid\":\"93f101be493d11ecb7c2543344556776\",\"menuPerms\":\"aaaaa\",\"menuType\":1,\"menuUrl\":\"dev/form\",\"modifier\":\"admin\",\"uuid\":\"4a6ddb331d464050ac87f8c13f57facc\"},\"needsOperator\":false}]',76,'127.0.0.1','2021-12-08 22:42:39'),(2,'mikey','修改菜单','io.dev.admin.adapter.rest.sys.MenuController.update()','[{\"menuCO\":{\"deleted\":false,\"gmtCreate\":1637357736000,\"gmtModified\":1637357736000,\"id\":23,\"menuIcon\":\"list\",\"menuName\":\"菜单管理\",\"menuOrder\":1,\"menuParentName\":\"系统管理\",\"menuParentUuid\":\"93f101be493d11ecb7c254334455677\",\"menuType\":1,\"menuUrl\":\"sys/menu\",\"uuid\":\"93ebbcb2493d11ecb7c254e1ad134d77\"},\"needsOperator\":false}]',26,'127.0.0.1','2021-12-08 22:42:41'),(3,'mikey','修改菜单','io.dev.admin.adapter.rest.sys.MenuController.update()','[{\"menuCO\":{\"deleted\":false,\"gmtCreate\":1638316144000,\"gmtModified\":1638316144000,\"id\":45,\"menuIcon\":\"user\",\"menuName\":\"用户管理\",\"menuOrder\":6,\"menuParentName\":\"系统管理\",\"menuParentUuid\":\"93f101be493d11ecb7c254334455677\",\"menuType\":1,\"menuUrl\":\"sys/user\",\"uuid\":\"0bb9b97e51f511eca3306106f8cc1608\"},\"needsOperator\":false}]',8,'127.0.0.1','2021-12-08 22:42:43'),(4,'mikey','修改菜单','io.dev.admin.adapter.rest.sys.MenuController.update()','[{\"menuCO\":{\"deleted\":false,\"gmtCreate\":1638374380000,\"gmtModified\":1638374380000,\"id\":51,\"menuIcon\":\"documentation\",\"menuName\":\"系统日志\",\"menuOrder\":10,\"menuParentName\":\"系统管理\",\"menuParentUuid\":\"93f101be493d11ecb7c254334455677\",\"menuType\":1,\"menuUrl\":\"sys/log\",\"uuid\":\"a2fd81e4527c11ecb2ccd76ea2e07fd8\"},\"needsOperator\":false}]',15,'127.0.0.1','2021-12-08 22:42:44'),(5,'mikey','修改菜单','io.dev.admin.adapter.rest.sys.MenuController.update()','[{\"menuCO\":{\"deleted\":false,\"gmtCreate\":1637357729000,\"gmtModified\":1637357729000,\"id\":18,\"menuIcon\":\"peoples\",\"menuName\":\"角色管理\",\"menuOrder\":0,\"menuParentName\":\"系统管理\",\"menuParentUuid\":\"93f101be493d11ecb7c254334455677\",\"menuType\":1,\"menuUrl\":\"sys/role\",\"uuid\":\"8fc52771493d11ecb7c254e1ad134d77\"},\"needsOperator\":false}]',9,'127.0.0.1','2021-12-08 22:42:46'),(6,'admin','添加菜单','io.dev.admin.adapter.rest.sys.MenuController.add()','[{\"menuDTO\":{\"menuIcon\":\"skill\",\"menuName\":\"代码生成\",\"menuOrder\":0,\"menuParentName\":\"开发工具\",\"menuParentUuid\":\"93f101be493d11ecb7c2543344556776\",\"menuPerms\":\"\",\"menuType\":1,\"menuUrl\":\"dev/code\",\"uuid\":\"\"},\"needsOperator\":false}]',7,'127.0.0.1','2021-12-08 22:42:48'),(7,'admin','添加菜单','io.ddd.framework.adapter.rest.sys.MenuController.add()','[{\"menuDTO\":{\"menuIcon\":\"money\",\"menuName\":\"目录1\",\"menuOrder\":0,\"menuParentName\":\"一级菜单\",\"menuParentUuid\":\"0\",\"menuPerms\":\"\",\"menuType\":0,\"menuUrl\":\"\",\"uuid\":\"\"},\"needsOperator\":false}]',5,'127.0.0.1','2021-12-11 17:07:30'),(8,'admin','添加菜单','io.ddd.framework.adapter.rest.sys.MenuController.add()','[{\"menuDTO\":{\"menuIcon\":\"bug\",\"menuName\":\"菜单1\",\"menuOrder\":1,\"menuParentName\":\"目录1\",\"menuParentUuid\":\"adf97607dbcc4de8b258ea17fc5af5ea\",\"menuPerms\":\"大的\",\"menuType\":1,\"menuUrl\":\"sys/helo\",\"uuid\":\"\"},\"needsOperator\":false}]',13,'127.0.0.1','2021-12-11 17:08:52'),(9,'admin','添加菜单','io.ddd.framework.adapter.rest.sys.MenuController.add()','[{\"menuDTO\":{\"menuIcon\":\"nested\",\"menuName\":\"目录2\",\"menuOrder\":3,\"menuParentName\":\"一级菜单\",\"menuParentUuid\":\"0\",\"menuPerms\":\"\",\"menuType\":0,\"menuUrl\":\"\",\"uuid\":\"\"},\"needsOperator\":false}]',7,'127.0.0.1','2021-12-11 17:30:14'),(10,'admin','添加菜单','io.ddd.framework.adapter.rest.sys.MenuController.add()','[{\"menuDTO\":{\"menuIcon\":\"excel\",\"menuName\":\"菜单2\",\"menuOrder\":0,\"menuParentName\":\"目录2\",\"menuParentUuid\":\"43061d67000e47de85346d3cc0b0a5b7\",\"menuPerms\":\"asdfa\",\"menuType\":1,\"menuUrl\":\"/hello\",\"uuid\":\"\"},\"needsOperator\":false}]',5,'127.0.0.1','2021-12-11 17:30:48');
/*!40000 ALTER TABLE `sys_log` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `sys_menu`
--

DROP TABLE IF EXISTS `sys_menu`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `sys_menu` (
  `id` bigint(20) unsigned NOT NULL AUTO_INCREMENT COMMENT '主键',
  `uuid` varchar(32) NOT NULL COMMENT 'uuid',
  `parent_uuid` varchar(32) DEFAULT NULL COMMENT '父菜单uuid',
  `name` varchar(64) DEFAULT NULL COMMENT '菜单名',
  `path` varchar(200) DEFAULT NULL COMMENT '前端路由路径',
  `component` varchar(255) DEFAULT NULL,
  `redirect` varchar(255) DEFAULT NULL,
  `title` varchar(255) DEFAULT NULL,
  `icon` varchar(50) DEFAULT NULL COMMENT '图标',
  `api` varchar(200) DEFAULT NULL COMMENT '后端接口api',
  `method` varchar(50) DEFAULT NULL COMMENT 'api方法',
  `menu_type` varchar(1) DEFAULT NULL COMMENT '0:目录 1:菜单 2:按钮',
  `order` int(11) DEFAULT NULL COMMENT '排序',
  `remark` varchar(64) DEFAULT NULL COMMENT '备注',
  `creator` varchar(64) DEFAULT NULL COMMENT '创建人',
  `modifier` varchar(64) DEFAULT NULL COMMENT '修改人',
  `gmt_create` datetime NOT NULL COMMENT '创建时间',
  `gmt_modified` datetime NOT NULL COMMENT '修改时间',
  `deleted` tinyint(1) NOT NULL DEFAULT '0' COMMENT '逻辑删除',
  PRIMARY KEY (`id`),
  KEY `idx_menu_id` (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=25 DEFAULT CHARSET=utf8mb4 COMMENT='菜单表';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `sys_menu`
--

LOCK TABLES `sys_menu` WRITE;
/*!40000 ALTER TABLE `sys_menu` DISABLE KEYS */;
INSERT INTO `sys_menu` VALUES (1,'87383f93493d11ecb7c254e1ad134d71',NULL,'Dashboard','/dashboard','LAYOUT','/dashboard/analysis','routes.dashboard.dashboard','bx:bx-home','/api/login','POST',NULL,1,NULL,NULL,NULL,'2023-07-10 15:27:49','2023-07-10 15:27:49',0),(4,'7A8590DB23A44159B4266F4E76C609E7',NULL,'Permission','/permission','LAYOUT','/permission/front/page','routes.demo.permission.permission','ion:tv-outline','/permission','POST',NULL,2,NULL,NULL,NULL,'2023-07-10 15:27:52','2023-07-10 15:27:52',0),(5,'7A8590DB23A44159B4266F4E76C609E6','7A8590DB23A44159B4266F4E76C609E4','BackAuthBtn','btn','/demo/permission/back/index','','routes.demo.permission.backPage',NULL,'btn','POST',NULL,1,NULL,NULL,NULL,'2023-07-10 15:27:52','2023-07-10 15:27:52',0),(6,'7A8590DB23A44159B4266F4E76C609E5','7A8590DB23A44159B4266F4E76C609E4','BackAuthPage','page','/demo/permission/back/Btn','','routes.demo.permission.backBtn',NULL,'page','POST',NULL,2,NULL,NULL,NULL,'2023-07-10 15:27:52','2023-07-10 15:27:52',0),(7,'7A8590DB23A44159B4266F4E76C609E4','7A8590DB23A44159B4266F4E76C609E7','PermissionBackDemo','back','','/permission/front/page','routes.demo.permission.back','','/api/v1/system/user/info','GET',NULL,NULL,NULL,NULL,NULL,'2023-07-10 15:27:52','2023-07-10 15:27:52',0),(8,'7A8590DB23A44159B4266F4E76C609E3','87383f93493d11ecb7c254e1ad134d71','Workbench','workbench','/dashboard/workbench/index','','routes.dashboard.workbench','','workbench','POST',NULL,NULL,NULL,NULL,NULL,'2023-07-10 15:27:52','2023-07-10 15:27:52',0),(9,'7A8590DB23A44159B4266F4E76C609E2','87383f93493d11ecb7c254e1ad134d71','Analysis','analysis','/dashboard/analysis/index','','routes.dashboard.analysis','','analysis','POST',NULL,NULL,NULL,NULL,NULL,'2023-07-10 15:27:52','2023-07-10 15:27:52',0),(10,'7A8590DB23A44159B4266F4E76C60921','7A8590DB23A44159B4266F4E76C60917','DeptManagement','dept','/demo/system/dept/index','','routes.demo.system.dept',NULL,'dept','POST',NULL,NULL,NULL,NULL,NULL,'2023-07-10 15:27:52','2023-07-10 15:27:52',0),(11,'7A8590DB23A44159B4266F4E76C60920','7A8590DB23A44159B4266F4E76C60917','MenuManagement','menu','/demo/system/menu/index','','routes.demo.system.menu','','/api/v1/system/menu/page','POST',NULL,NULL,NULL,NULL,NULL,'2023-07-10 15:27:52','2023-07-10 15:27:52',0),(12,'7A8590DB23A44159B4266F4E76C60919','7A8590DB23A44159B4266F4E76C60917','RoleManagement','role','/demo/system/role/index','','routes.demo.system.role',NULL,'/api/v1/system/role/page','POST',NULL,NULL,NULL,NULL,NULL,'2023-07-10 15:27:52','2023-07-10 15:27:52',0),(13,'7A8590DB23A44159B4266F4E76C60918','7A8590DB23A44159B4266F4E76C60917','AccountManagement','account','/demo/system/account/index','','routes.demo.system.account',NULL,'account','POST',NULL,NULL,NULL,NULL,NULL,'2023-07-10 15:27:52','2023-07-10 15:27:52',0),(14,'7A8590DB23A44159B4266F4E76C60917',NULL,'System','/system','LAYOUT','/system/account','routes.demo.system.moduleName','ion:settings-outline','/system','POST',NULL,99999,NULL,NULL,NULL,'2023-07-10 15:27:52','2023-07-10 15:27:52',0),(15,'7A8590DB23A44159B4266F4E76C60916','7A8590DB23A44159B4266F4E76C60911','Menu2Demo','menu2','/demo/level/Menu2','','Menu2',NULL,'menu2','POST',NULL,NULL,NULL,NULL,NULL,'2023-07-10 15:27:52','2023-07-10 15:27:52',0),(16,'7A8590DB23A44159B4266F4E76C60915','7A8590DB23A44159B4266F4E76C60912','Menu12Demo','menu1-2','/demo/level/Menu12','','Menu1-2',NULL,'menu1-2','POST',NULL,NULL,NULL,NULL,NULL,'2023-07-10 15:27:52','2023-07-10 15:27:52',0),(17,'7A8590DB23A44159B4266F4E76C60914','7A8590DB23A44159B4266F4E76C60913','Menu111Demo','menu1-1-1','/demo/level/Menu111','','Menu111','','menu1-1-1','POST',NULL,NULL,NULL,NULL,NULL,'2023-07-10 15:27:52','2023-07-10 15:27:52',0),(18,'7A8590DB23A44159B4266F4E76C60913','7A8590DB23A44159B4266F4E76C60912','Menu11Demo','menu1-1','/demo/system/account/index','','Menu1-1',NULL,'menu1-1','POST',NULL,NULL,NULL,NULL,NULL,'2023-07-10 15:27:52','2023-07-10 15:27:52',0),(19,'7A8590DB23A44159B4266F4E76C60912','7A8590DB23A44159B4266F4E76C60911','Menu1Demo','menu1','/demo/system/role/index','','menu1',NULL,'menu1','POST',NULL,NULL,NULL,NULL,NULL,'2023-07-10 15:27:52','2023-07-10 15:27:52',0),(20,'7A8590DB23A44159B4266F4E76C60911',NULL,'Level','/level','LAYOUT','','routes.demo.level.level','carbon:user-role','/level','POST',NULL,4,NULL,NULL,NULL,'2023-07-10 15:27:52','2023-07-10 15:27:52',0),(22,'7A8590DB23A44159B4266F4E76C60922',NULL,'Link','/link','LAYOUT',NULL,'routes.demo.iframe.frame','ion:tv-outline',NULL,NULL,NULL,5,NULL,NULL,NULL,'2023-07-14 10:54:49','2023-07-14 10:54:51',0),(23,'7A8590DB23A44159B4266F4E76C60923','7A8590DB23A44159B4266F4E76C60922','Doc','doc','',NULL,'routes.demo.iframe.doc','',NULL,NULL,NULL,NULL,NULL,NULL,NULL,'2023-07-14 10:54:49','2023-07-14 10:54:51',0),(24,'7A8590DB23A44159B4266F4E76C60924','7A8590DB23A44159B4266F4E76C60922','DocExternal','https://doc.vvbin.cn/','LAYOUT',NULL,'routes.demo.iframe.docExternal','',NULL,NULL,NULL,NULL,NULL,NULL,NULL,'2023-07-14 10:54:49','2023-07-14 10:54:51',0);
/*!40000 ALTER TABLE `sys_menu` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `sys_role`
--

DROP TABLE IF EXISTS `sys_role`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `sys_role` (
  `id` bigint(20) unsigned NOT NULL AUTO_INCREMENT COMMENT '主键',
  `uuid` varchar(32) NOT NULL COMMENT 'uuid',
  `name` varchar(64) DEFAULT NULL COMMENT '角色名',
  `remark` varchar(64) DEFAULT NULL COMMENT '备注',
  `creator` varchar(64) DEFAULT NULL COMMENT '创建人',
  `modifier` varchar(64) DEFAULT NULL COMMENT '修改人',
  `gmt_create` datetime NOT NULL COMMENT '创建时间',
  `gmt_modified` datetime NOT NULL COMMENT '修改时间',
  `deleted` tinyint(1) NOT NULL DEFAULT '0' COMMENT '逻辑删除',
  PRIMARY KEY (`id`),
  KEY `idx_role_id` (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=3 DEFAULT CHARSET=utf8mb4 COMMENT='角色表';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `sys_role`
--

LOCK TABLES `sys_role` WRITE;
/*!40000 ALTER TABLE `sys_role` DISABLE KEYS */;
INSERT INTO `sys_role` VALUES (1,'93f101be493d11ecb7c254e1ad134d77','管理员','管理员','SYS','admin','2021-11-19 13:51:37','2021-11-19 13:51:39',0),(2,'0caadea8ae91427eb3d869df43d0fe41','开发人员','开发人员','admin','admin','2021-12-01 08:57:50','2021-12-01 08:57:50',0);
/*!40000 ALTER TABLE `sys_role` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `sys_role_of_menu`
--

DROP TABLE IF EXISTS `sys_role_of_menu`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `sys_role_of_menu` (
  `id` bigint(20) unsigned NOT NULL AUTO_INCREMENT COMMENT '主键',
  `uuid` varchar(32) NOT NULL COMMENT 'uuid',
  `role_uuid` varchar(32) NOT NULL COMMENT '角色UUID',
  `menu_uuid` varchar(32) NOT NULL COMMENT '菜单UUID',
  `creator` varchar(64) DEFAULT NULL COMMENT '创建人',
  `modifier` varchar(64) DEFAULT NULL COMMENT '修改人',
  `gmt_create` datetime NOT NULL COMMENT '创建时间',
  `gmt_modified` datetime NOT NULL COMMENT '修改时间',
  `deleted` tinyint(1) NOT NULL DEFAULT '0' COMMENT '逻辑删除',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=215 DEFAULT CHARSET=utf8mb4 COMMENT='角色关联菜单表';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `sys_role_of_menu`
--

LOCK TABLES `sys_role_of_menu` WRITE;
/*!40000 ALTER TABLE `sys_role_of_menu` DISABLE KEYS */;
INSERT INTO `sys_role_of_menu` VALUES (191,'87383f93493d11ecb7c254e1ad134d71','93f101be493d11ecb7c254e1ad134d77','87383f93493d11ecb7c254e1ad134d71',NULL,NULL,'2023-07-10 21:54:28','2023-07-10 21:54:28',0),(192,'7A8590DB23A44159B4266F4E76C609E9','93f101be493d11ecb7c254e1ad134d77','7A8590DB23A44159B4266F4E76C609E9',NULL,NULL,'2023-07-10 21:54:28','2023-07-10 21:54:28',0),(193,'7A8590DB23A44159B4266F4E76C609E8','93f101be493d11ecb7c254e1ad134d77','7A8590DB23A44159B4266F4E76C609E8',NULL,NULL,'2023-07-10 21:54:28','2023-07-10 21:54:28',0),(194,'7A8590DB23A44159B4266F4E76C609E7','93f101be493d11ecb7c254e1ad134d77','7A8590DB23A44159B4266F4E76C609E7',NULL,NULL,'2023-07-10 21:54:28','2023-07-10 21:54:28',0),(195,'7A8590DB23A44159B4266F4E76C609E6','93f101be493d11ecb7c254e1ad134d77','7A8590DB23A44159B4266F4E76C609E6',NULL,NULL,'2023-07-10 21:54:28','2023-07-10 21:54:28',0),(196,'7A8590DB23A44159B4266F4E76C609E5','93f101be493d11ecb7c254e1ad134d77','7A8590DB23A44159B4266F4E76C609E5',NULL,NULL,'2023-07-10 21:54:28','2023-07-10 21:54:28',0),(197,'7A8590DB23A44159B4266F4E76C609E4','93f101be493d11ecb7c254e1ad134d77','7A8590DB23A44159B4266F4E76C609E4',NULL,NULL,'2023-07-10 21:54:28','2023-07-10 21:54:28',0),(198,'7A8590DB23A44159B4266F4E76C609E3','93f101be493d11ecb7c254e1ad134d77','7A8590DB23A44159B4266F4E76C609E3',NULL,NULL,'2023-07-10 21:54:28','2023-07-10 21:54:28',0),(199,'7A8590DB23A44159B4266F4E76C609E2','93f101be493d11ecb7c254e1ad134d77','7A8590DB23A44159B4266F4E76C609E2',NULL,NULL,'2023-07-10 21:54:28','2023-07-10 21:54:28',0),(200,'7A8590DB23A44159B4266F4E76C60921','93f101be493d11ecb7c254e1ad134d77','7A8590DB23A44159B4266F4E76C60921',NULL,NULL,'2023-07-10 21:54:28','2023-07-10 21:54:28',0),(201,'7A8590DB23A44159B4266F4E76C60920','93f101be493d11ecb7c254e1ad134d77','7A8590DB23A44159B4266F4E76C60920',NULL,NULL,'2023-07-10 21:54:28','2023-07-10 21:54:28',0),(202,'7A8590DB23A44159B4266F4E76C60919','93f101be493d11ecb7c254e1ad134d77','7A8590DB23A44159B4266F4E76C60919',NULL,NULL,'2023-07-10 21:54:28','2023-07-10 21:54:28',0),(203,'7A8590DB23A44159B4266F4E76C60918','93f101be493d11ecb7c254e1ad134d77','7A8590DB23A44159B4266F4E76C60918',NULL,NULL,'2023-07-10 21:54:28','2023-07-10 21:54:28',0),(204,'7A8590DB23A44159B4266F4E76C60917','93f101be493d11ecb7c254e1ad134d77','7A8590DB23A44159B4266F4E76C60917',NULL,NULL,'2023-07-10 21:54:28','2023-07-10 21:54:28',0),(205,'7A8590DB23A44159B4266F4E76C60916','93f101be493d11ecb7c254e1ad134d77','7A8590DB23A44159B4266F4E76C60916',NULL,NULL,'2023-07-10 21:54:28','2023-07-10 21:54:28',0),(206,'7A8590DB23A44159B4266F4E76C60915','93f101be493d11ecb7c254e1ad134d77','7A8590DB23A44159B4266F4E76C60915',NULL,NULL,'2023-07-10 21:54:28','2023-07-10 21:54:28',0),(207,'7A8590DB23A44159B4266F4E76C60914','93f101be493d11ecb7c254e1ad134d77','7A8590DB23A44159B4266F4E76C60914',NULL,NULL,'2023-07-10 21:54:28','2023-07-10 21:54:28',0),(208,'7A8590DB23A44159B4266F4E76C60913','93f101be493d11ecb7c254e1ad134d77','7A8590DB23A44159B4266F4E76C60913',NULL,NULL,'2023-07-10 21:54:28','2023-07-10 21:54:28',0),(209,'7A8590DB23A44159B4266F4E76C60912','93f101be493d11ecb7c254e1ad134d77','7A8590DB23A44159B4266F4E76C60912',NULL,NULL,'2023-07-10 21:54:28','2023-07-10 21:54:28',0),(210,'7A8590DB23A44159B4266F4E76C60911','93f101be493d11ecb7c254e1ad134d77','7A8590DB23A44159B4266F4E76C60911',NULL,NULL,'2023-07-10 21:54:28','2023-07-10 21:54:28',0),(211,'7A8590DB23A44159B4266F4E76C60910','93f101be493d11ecb7c254e1ad134d77','7A8590DB23A44159B4266F4E76C60910',NULL,NULL,'2023-07-10 21:54:28','2023-07-10 21:54:28',0),(212,'7A8590DB23A44159B4266F4E76C60922','93f101be493d11ecb7c254e1ad134d77','7A8590DB23A44159B4266F4E76C60922',NULL,NULL,'2023-07-10 21:54:28','2023-07-10 21:54:28',0),(213,'7A8590DB23A44159B4266F4E76C60923','93f101be493d11ecb7c254e1ad134d77','7A8590DB23A44159B4266F4E76C60923',NULL,NULL,'2023-07-10 21:54:28','2023-07-10 21:54:28',0),(214,'7A8590DB23A44159B4266F4E76C60924','93f101be493d11ecb7c254e1ad134d77','7A8590DB23A44159B4266F4E76C60924',NULL,NULL,'2023-07-10 21:54:28','2023-07-10 21:54:28',0);
/*!40000 ALTER TABLE `sys_role_of_menu` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `sys_user`
--

DROP TABLE IF EXISTS `sys_user`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `sys_user` (
  `id` bigint(20) unsigned NOT NULL AUTO_INCREMENT COMMENT '主键',
  `uuid` varchar(32) NOT NULL COMMENT 'uuid',
  `username` varchar(64) NOT NULL COMMENT '账号',
  `password` varchar(64) DEFAULT NULL COMMENT '密码',
  `name` varchar(64) DEFAULT NULL COMMENT '名字',
  `email` varchar(128) DEFAULT NULL COMMENT '邮箱',
  `status` int(1) DEFAULT '0' COMMENT '角色',
  `creator` varchar(64) DEFAULT NULL COMMENT '创建人',
  `modifier` varchar(64) DEFAULT NULL COMMENT '修改人',
  `gmt_create` datetime NOT NULL COMMENT '创建时间',
  `gmt_modified` datetime NOT NULL COMMENT '修改时间',
  `deleted` tinyint(1) NOT NULL DEFAULT '0' COMMENT '逻辑删除',
  `avatar` varchar(100) DEFAULT NULL COMMENT '头像',
  PRIMARY KEY (`id`),
  UNIQUE KEY `sys_user_user_id_uindex` (`username`),
  UNIQUE KEY `sys_user_pk` (`username`)
) ENGINE=InnoDB AUTO_INCREMENT=11 DEFAULT CHARSET=utf8mb4 COMMENT='用户表';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `sys_user`
--

LOCK TABLES `sys_user` WRITE;
/*!40000 ALTER TABLE `sys_user` DISABLE KEYS */;
INSERT INTO `sys_user` VALUES (1,'9df9de08354f456c97fae0cdb3df214f','admin','$2b$12$bRzARfFsrM1wAdNmJJP6ROpmCttzEH66AC9lapR9j4Hx//PMP9auW','admin','devcloudadmin@qq.com',0,NULL,'admin','2021-11-12 22:48:15','2021-11-12 22:48:15',0,NULL),(2,'26220d63bfd345dabb2f114287965313','mikey','$2a$10$ZnuPGCEusk5tiKxAB/1lreynJxuvh4mqu8So6vUok/PCBCYP34.gi','mikey','mikey',0,NULL,NULL,'2021-12-01 09:39:53','2021-12-01 09:39:53',0,NULL),(3,'b8c1e673060c437ba0925f119538bdb6','fasf','$2a$10$ILTzdnK9HtI4BiTXneTWOOgiLid.QHTuDK0iV42EvH142xhuSheqy','fasdf','asfd@qq.com',0,'mikey','admin','2021-12-01 09:46:07','2021-12-01 09:46:07',1,NULL),(6,'841bf88b99ac45c4ba6a1189abe6bea8','adsfasdf','xxxxxxx','hhhh','dasfa@qq.com',0,NULL,NULL,'1970-01-01 00:00:00','1970-01-01 00:00:00',0,NULL),(9,'e4ea0c9574e1477091577c29a4a149a2','adsfafsdf','xxxxxxx','hhfhh','dafsfa@qq.com',0,NULL,NULL,'1970-01-01 00:00:00','1970-01-01 00:00:00',0,NULL),(10,'9ae284e537494f1088a342d380624614','adminkk','$2b$12$bRzARfFsrM1wAdNmJJP6ROpmCttzEH66AC9lapR9j4Hx//PMP9auW','hhhadsffffddah','dasfffd@qq.com',0,NULL,NULL,'1970-01-01 00:00:00','1970-01-01 00:00:00',0,NULL);
/*!40000 ALTER TABLE `sys_user` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `sys_user_of_role`
--

DROP TABLE IF EXISTS `sys_user_of_role`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `sys_user_of_role` (
  `id` bigint(20) unsigned NOT NULL AUTO_INCREMENT COMMENT '主键',
  `uuid` varchar(32) NOT NULL COMMENT 'uuid',
  `user_uuid` varchar(32) NOT NULL COMMENT '用户UUID',
  `role_uuid` varchar(32) NOT NULL COMMENT '角色UUID',
  `creator` varchar(64) DEFAULT NULL COMMENT '创建人',
  `modifier` varchar(64) DEFAULT NULL COMMENT '修改人',
  `gmt_create` datetime NOT NULL COMMENT '创建时间',
  `gmt_modified` datetime NOT NULL COMMENT '修改时间',
  `deleted` tinyint(1) NOT NULL DEFAULT '0' COMMENT '逻辑删除',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=15 DEFAULT CHARSET=utf8mb4 COMMENT='用户关联角色表';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `sys_user_of_role`
--

LOCK TABLES `sys_user_of_role` WRITE;
/*!40000 ALTER TABLE `sys_user_of_role` DISABLE KEYS */;
INSERT INTO `sys_user_of_role` VALUES (1,'93f101be493d11ecb7c254e1ad134d78','9df9de08354f456c97fae0cdb3df214f','93f101be493d11ecb7c254e1ad134d77','SYS','SYS','2021-11-19 13:52:12','2021-11-19 13:52:13',0),(4,'dbd90a6a2c824d6284651e609aed8d80','b8c1e673060c437ba0925f119538bdb6','93f101be493d11ecb7c254e1ad134d77','mikey','mikey','2021-12-01 09:46:07','2021-12-01 09:46:07',0),(5,'5a157d23f39c435faffc36c81758d660','b8c1e673060c437ba0925f119538bdb6','0caadea8ae91427eb3d869df43d0fe41','mikey','mikey','2021-12-01 09:46:07','2021-12-01 09:46:07',0),(12,'9f35f44349074d05b476257e140978f4','b8c1e673060c437ba0925f119538bdb6','0caadea8ae91427eb3d869df43d0fe41','mikey','mikey','2021-12-02 03:42:46','2021-12-02 03:42:46',0),(13,'8a9551b14bb347cd9f7112a4928848bb','b8c1e673060c437ba0925f119538bdb6','0caadea8ae91427eb3d869df43d0fe41','mikey','mikey','2021-12-02 03:42:57','2021-12-02 03:42:57',0),(14,'2b2f93dbdee74266b4db6cf633d2d49a','26220d63bfd345dabb2f114287965313','0caadea8ae91427eb3d869df43d0fe41','mikey','mikey','2021-12-02 03:49:31','2021-12-02 03:49:31',0);
/*!40000 ALTER TABLE `sys_user_of_role` ENABLE KEYS */;
UNLOCK TABLES;
/*!40103 SET TIME_ZONE=@OLD_TIME_ZONE */;

/*!40101 SET SQL_MODE=@OLD_SQL_MODE */;
/*!40014 SET FOREIGN_KEY_CHECKS=@OLD_FOREIGN_KEY_CHECKS */;
/*!40014 SET UNIQUE_CHECKS=@OLD_UNIQUE_CHECKS */;
/*!40101 SET CHARACTER_SET_CLIENT=@OLD_CHARACTER_SET_CLIENT */;
/*!40101 SET CHARACTER_SET_RESULTS=@OLD_CHARACTER_SET_RESULTS */;
/*!40101 SET COLLATION_CONNECTION=@OLD_COLLATION_CONNECTION */;
/*!40111 SET SQL_NOTES=@OLD_SQL_NOTES */;

-- Dump completed on 2023-07-15 12:53:40
