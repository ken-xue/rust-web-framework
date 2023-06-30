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
INSERT INTO `sys_log` VALUES (1,'mikey','修改菜单','io.dev.admin.adapter.rest.sys.MenuController.update()','[{\"menuCO\":{\"creator\":\"admin\",\"deleted\":false,\"gmtCreate\":1638345181000,\"gmtModified\":1638345181000,\"id\":50,\"menuIcon\":\"excel\",\"menuName\":\"表单生成\",\"menuOrder\":0,\"menuParentName\":\"开发工具\",\"menuParentUuid\":\"93f101be493d11ecb7c2543344556776\",\"menuPerms\":\"aaaaa\",\"menuType\":1,\"menuUrl\":\"dev/form\",\"modifier\":\"admin\",\"uuid\":\"4a6ddb331d464050ac87f8c13f57facc\"},\"needsOperator\":false}]',76,'127.0.0.1','2021-12-09 06:42:39'),(2,'mikey','修改菜单','io.dev.admin.adapter.rest.sys.MenuController.update()','[{\"menuCO\":{\"deleted\":false,\"gmtCreate\":1637357736000,\"gmtModified\":1637357736000,\"id\":23,\"menuIcon\":\"list\",\"menuName\":\"菜单管理\",\"menuOrder\":1,\"menuParentName\":\"系统管理\",\"menuParentUuid\":\"93f101be493d11ecb7c254334455677\",\"menuType\":1,\"menuUrl\":\"sys/menu\",\"uuid\":\"93ebbcb2493d11ecb7c254e1ad134d77\"},\"needsOperator\":false}]',26,'127.0.0.1','2021-12-09 06:42:41'),(3,'mikey','修改菜单','io.dev.admin.adapter.rest.sys.MenuController.update()','[{\"menuCO\":{\"deleted\":false,\"gmtCreate\":1638316144000,\"gmtModified\":1638316144000,\"id\":45,\"menuIcon\":\"user\",\"menuName\":\"用户管理\",\"menuOrder\":6,\"menuParentName\":\"系统管理\",\"menuParentUuid\":\"93f101be493d11ecb7c254334455677\",\"menuType\":1,\"menuUrl\":\"sys/user\",\"uuid\":\"0bb9b97e51f511eca3306106f8cc1608\"},\"needsOperator\":false}]',8,'127.0.0.1','2021-12-09 06:42:43'),(4,'mikey','修改菜单','io.dev.admin.adapter.rest.sys.MenuController.update()','[{\"menuCO\":{\"deleted\":false,\"gmtCreate\":1638374380000,\"gmtModified\":1638374380000,\"id\":51,\"menuIcon\":\"documentation\",\"menuName\":\"系统日志\",\"menuOrder\":10,\"menuParentName\":\"系统管理\",\"menuParentUuid\":\"93f101be493d11ecb7c254334455677\",\"menuType\":1,\"menuUrl\":\"sys/log\",\"uuid\":\"a2fd81e4527c11ecb2ccd76ea2e07fd8\"},\"needsOperator\":false}]',15,'127.0.0.1','2021-12-09 06:42:44'),(5,'mikey','修改菜单','io.dev.admin.adapter.rest.sys.MenuController.update()','[{\"menuCO\":{\"deleted\":false,\"gmtCreate\":1637357729000,\"gmtModified\":1637357729000,\"id\":18,\"menuIcon\":\"peoples\",\"menuName\":\"角色管理\",\"menuOrder\":0,\"menuParentName\":\"系统管理\",\"menuParentUuid\":\"93f101be493d11ecb7c254334455677\",\"menuType\":1,\"menuUrl\":\"sys/role\",\"uuid\":\"8fc52771493d11ecb7c254e1ad134d77\"},\"needsOperator\":false}]',9,'127.0.0.1','2021-12-09 06:42:46'),(6,'admin','添加菜单','io.dev.admin.adapter.rest.sys.MenuController.add()','[{\"menuDTO\":{\"menuIcon\":\"skill\",\"menuName\":\"代码生成\",\"menuOrder\":0,\"menuParentName\":\"开发工具\",\"menuParentUuid\":\"93f101be493d11ecb7c2543344556776\",\"menuPerms\":\"\",\"menuType\":1,\"menuUrl\":\"dev/code\",\"uuid\":\"\"},\"needsOperator\":false}]',7,'127.0.0.1','2021-12-09 06:42:48'),(7,'admin','添加菜单','io.ddd.framework.adapter.rest.sys.MenuController.add()','[{\"menuDTO\":{\"menuIcon\":\"money\",\"menuName\":\"目录1\",\"menuOrder\":0,\"menuParentName\":\"一级菜单\",\"menuParentUuid\":\"0\",\"menuPerms\":\"\",\"menuType\":0,\"menuUrl\":\"\",\"uuid\":\"\"},\"needsOperator\":false}]',5,'127.0.0.1','2021-12-12 01:07:30'),(8,'admin','添加菜单','io.ddd.framework.adapter.rest.sys.MenuController.add()','[{\"menuDTO\":{\"menuIcon\":\"bug\",\"menuName\":\"菜单1\",\"menuOrder\":1,\"menuParentName\":\"目录1\",\"menuParentUuid\":\"adf97607dbcc4de8b258ea17fc5af5ea\",\"menuPerms\":\"大的\",\"menuType\":1,\"menuUrl\":\"sys/helo\",\"uuid\":\"\"},\"needsOperator\":false}]',13,'127.0.0.1','2021-12-12 01:08:52'),(9,'admin','添加菜单','io.ddd.framework.adapter.rest.sys.MenuController.add()','[{\"menuDTO\":{\"menuIcon\":\"nested\",\"menuName\":\"目录2\",\"menuOrder\":3,\"menuParentName\":\"一级菜单\",\"menuParentUuid\":\"0\",\"menuPerms\":\"\",\"menuType\":0,\"menuUrl\":\"\",\"uuid\":\"\"},\"needsOperator\":false}]',7,'127.0.0.1','2021-12-12 01:30:14'),(10,'admin','添加菜单','io.ddd.framework.adapter.rest.sys.MenuController.add()','[{\"menuDTO\":{\"menuIcon\":\"excel\",\"menuName\":\"菜单2\",\"menuOrder\":0,\"menuParentName\":\"目录2\",\"menuParentUuid\":\"43061d67000e47de85346d3cc0b0a5b7\",\"menuPerms\":\"asdfa\",\"menuType\":1,\"menuUrl\":\"/hello\",\"uuid\":\"\"},\"needsOperator\":false}]',5,'127.0.0.1','2021-12-12 01:30:48');
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
                            `uuid` varchar(32) DEFAULT NULL COMMENT 'uuid',
                            `parent_uuid` varchar(32) DEFAULT NULL COMMENT '父菜单uuid',
                            `name` varchar(64) DEFAULT NULL COMMENT '菜单名',
                            `url` varchar(200) DEFAULT NULL COMMENT '菜单url',
                            `perms` varchar(200) DEFAULT NULL COMMENT '授权标识',
                            `menu_type` varchar(1) DEFAULT NULL COMMENT '0:目录 1:菜单 2:按钮',
                            `icon` varchar(20) DEFAULT NULL COMMENT '图标',
                            `order` int(11) DEFAULT NULL COMMENT '排序',
                            `remark` varchar(64) DEFAULT NULL COMMENT '备注',
                            `creator` varchar(64) DEFAULT NULL COMMENT '创建人',
                            `modifier` varchar(64) DEFAULT NULL COMMENT '修改人',
                            `gmt_create` datetime NOT NULL COMMENT '创建时间',
                            `gmt_modified` datetime NOT NULL COMMENT '修改时间',
                            `deleted` tinyint(1) NOT NULL DEFAULT '0' COMMENT '逻辑删除',
                            PRIMARY KEY (`id`),
                            KEY `idx_menu_id` (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=61 DEFAULT CHARSET=utf8mb4 COMMENT='菜单表';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `sys_menu`
--

LOCK TABLES `sys_menu` WRITE;
/*!40000 ALTER TABLE `sys_menu` DISABLE KEYS */;
INSERT INTO `sys_menu` VALUES (8,'87351d5e493d11ecb7c254e1ad134d77','93f101be493d11ecb7c254334455677','用户关联角色','sys/userofrole',NULL,'1','icon',6,NULL,NULL,'admin','2021-11-19 21:35:15','2021-11-19 21:35:15',1),(9,'87362493493d11ecb7c254e1ad134d77','87351d5e493d11ecb7c254e1ad134d77','查看',NULL,'sys:userofrole:info,sys:userofrole:page','2',NULL,6,NULL,NULL,NULL,'2021-11-19 21:35:15','2021-11-19 21:35:15',1),(10,'87383f93493d11ecb7c254e1ad134d77','87351d5e493d11ecb7c254e1ad134d77','新增',NULL,'sys:userofrole:add','2',NULL,6,NULL,NULL,NULL,'2021-11-19 21:35:15','2021-11-19 21:35:15',1),(11,'87393320493d11ecb7c254e1ad134d77','87351d5e493d11ecb7c254e1ad134d77','修改',NULL,'sys:userofrole:update','2',NULL,6,NULL,NULL,NULL,'2021-11-19 21:35:15','2021-11-19 21:35:15',1),(12,'873a1d50493d11ecb7c254e1ad134d77','87351d5e493d11ecb7c254e1ad134d77','删除',NULL,'sys:userofrole:delete','2',NULL,6,NULL,NULL,NULL,'2021-11-19 21:35:15','2021-11-19 21:35:15',1),(13,'8b916d0f493d11ecb7c254e1ad134d77','93f101be493d11ecb7c254334455677','角色关联菜单','sys/roleofmenu',NULL,'1','icon',6,NULL,NULL,'admin','2021-11-19 21:35:22','2021-11-19 21:35:22',1),(14,'8b93f7dc493d11ecb7c254e1ad134d77','8b916d0f493d11ecb7c254e1ad134d77','查看',NULL,'sys:roleofmenu:info,sys:roleofmenu:page','2',NULL,6,NULL,NULL,NULL,'2021-11-19 21:35:22','2021-11-19 21:35:22',1),(15,'8b94d734493d11ecb7c254e1ad134d77','8b916d0f493d11ecb7c254e1ad134d77','新增',NULL,'sys:roleofmenu:add','2',NULL,6,NULL,NULL,NULL,'2021-11-19 21:35:22','2021-11-19 21:35:22',1),(16,'8b95bc40493d11ecb7c254e1ad134d77','8b916d0f493d11ecb7c254e1ad134d77','修改',NULL,'sys:roleofmenu:update','2',NULL,6,NULL,NULL,NULL,'2021-11-19 21:35:22','2021-11-19 21:35:22',1),(17,'8b96995a493d11ecb7c254e1ad134d77','8b916d0f493d11ecb7c254e1ad134d77','删除',NULL,'sys:roleofmenu:delete','2',NULL,6,NULL,NULL,NULL,'2021-11-19 21:35:22','2021-11-19 21:35:22',1),(18,'8fc52771493d11ecb7c254e1ad134d77','93f101be493d11ecb7c254334455677','角色管理','sys/role',NULL,'1','peoples',0,NULL,NULL,NULL,'2021-11-19 21:35:29','2021-11-19 21:35:29',0),(19,'8fc6eb1a493d11ecb7c254e1ad134d77','8fc52771493d11ecb7c254e1ad134d77','查看',NULL,'sys:role:info,sys:role:page,sys:role:list','2',NULL,6,NULL,NULL,NULL,'2021-11-19 21:35:29','2021-11-19 21:35:29',0),(20,'8fc89835493d11ecb7c254e1ad134d77','8fc52771493d11ecb7c254e1ad134d77','新增',NULL,'sys:role:add','2',NULL,6,NULL,NULL,NULL,'2021-11-19 21:35:29','2021-11-19 21:35:29',0),(21,'8fc99a2d493d11ecb7c254e1ad134d77','8fc52771493d11ecb7c254e1ad134d77','修改',NULL,'sys:role:update','2',NULL,6,NULL,NULL,NULL,'2021-11-19 21:35:29','2021-11-19 21:35:29',0),(22,'8fcaa825493d11ecb7c254e1ad134d77','8fc52771493d11ecb7c254e1ad134d77','删除',NULL,'sys:role:delete','2',NULL,6,NULL,NULL,NULL,'2021-11-19 21:35:29','2021-11-19 21:35:29',0),(23,'93ebbcb2493d11ecb7c254e1ad134d77','93f101be493d11ecb7c254334455677','菜单管理','sys/menu',NULL,'1','list',1,NULL,NULL,NULL,'2021-11-19 21:35:36','2021-11-19 21:35:36',0),(24,'93ed48f8493d11ecb7c254e1ad134d77','93ebbcb2493d11ecb7c254e1ad134d77','查看',NULL,'sys:menu:info,sys:menu:page,sys:menu:list,sys:menu:select','2',NULL,6,NULL,NULL,NULL,'2021-11-19 21:35:36','2021-11-19 21:35:36',0),(25,'93ee29b6493d11ecb7c254e1ad134d77','93ebbcb2493d11ecb7c254e1ad134d77','新增',NULL,'sys:menu:add','2',NULL,6,NULL,NULL,NULL,'2021-11-19 21:35:36','2021-11-19 21:35:36',0),(26,'93f00a10493d11ecb7c254e1ad134d77','93ebbcb2493d11ecb7c254e1ad134d77','修改',NULL,'sys:menu:update','2',NULL,6,NULL,NULL,NULL,'2021-11-19 21:35:36','2021-11-19 21:35:36',0),(27,'93f101be493d11ecb7c254e1ad134d77','93ebbcb2493d11ecb7c254e1ad134d77','删除',NULL,'sys:menu:delete','2',NULL,6,NULL,NULL,NULL,'2021-11-19 21:35:36','2021-11-19 21:35:36',0),(28,'93f101be493d11ecb7c254334455677','0','系统管理',NULL,NULL,'0','example',1,NULL,NULL,NULL,'2021-11-19 21:35:36','2021-11-19 21:35:36',0),(29,'93f101be493d11ecb7c2543344556776','0','开发工具',NULL,NULL,'0','bug',2,NULL,NULL,NULL,'2021-11-19 21:35:36','2021-11-19 21:35:36',0),(45,'0bb9b97e51f511eca3306106f8cc1608','93f101be493d11ecb7c254334455677','用户管理','sys/user',NULL,'1','user',6,NULL,NULL,NULL,'2021-11-30 23:49:04','2021-11-30 23:49:04',0),(46,'0bbe311651f511eca3306106f8cc1608','0bb9b97e51f511eca3306106f8cc1608','查看',NULL,'sys:user:page,sys:user:info','2',NULL,6,NULL,NULL,NULL,'2021-11-30 23:49:04','2021-11-30 23:49:04',0),(47,'0bbf3e9e51f511eca3306106f8cc1608','0bb9b97e51f511eca3306106f8cc1608','新增',NULL,'sys:user:add','2',NULL,6,NULL,NULL,NULL,'2021-11-30 23:49:04','2021-11-30 23:49:04',0),(48,'0bc18ab451f511eca3306106f8cc1608','0bb9b97e51f511eca3306106f8cc1608','修改',NULL,'sys:user:update','2',NULL,6,NULL,NULL,NULL,'2021-11-30 23:49:04','2021-11-30 23:49:04',0),(49,'0bc60ce251f511eca3306106f8cc1608','0bb9b97e51f511eca3306106f8cc1608','删除',NULL,'sys:user:delete','2',NULL,6,NULL,NULL,NULL,'2021-11-30 23:49:04','2021-11-30 23:49:04',0),(50,'4a6ddb331d464050ac87f8c13f57facc','93f101be493d11ecb7c2543344556776','表单生成','dev/form','aaaaa','1','excel',0,NULL,'admin','admin','2021-12-01 07:53:01','2021-12-01 07:53:01',0),(51,'a2fd81e4527c11ecb2ccd76ea2e07fd8','93f101be493d11ecb7c254334455677','系统日志','sys/log',NULL,'1','documentation',10,NULL,NULL,NULL,'2021-12-01 15:59:40','2021-12-01 15:59:40',0),(52,'a329b368527c11ecb2ccd76ea2e07fd8','a2fd81e4527c11ecb2ccd76ea2e07fd8','查看',NULL,'sys:log:page,sys:log:info','2',NULL,6,NULL,NULL,NULL,'2021-12-01 15:59:40','2021-12-01 15:59:40',0),(53,'a32e88ca527c11ecb2ccd76ea2e07fd8','a2fd81e4527c11ecb2ccd76ea2e07fd8','新增',NULL,'sys:log:save','2',NULL,6,NULL,NULL,NULL,'2021-12-01 15:59:40','2021-12-01 15:59:40',0),(54,'a330ec3c527c11ecb2ccd76ea2e07fd8','a2fd81e4527c11ecb2ccd76ea2e07fd8','修改',NULL,'sys:log:update','2',NULL,6,NULL,NULL,NULL,'2021-12-01 15:59:40','2021-12-01 15:59:40',0),(55,'a3332a60527c11ecb2ccd76ea2e07fd8','a2fd81e4527c11ecb2ccd76ea2e07fd8','删除',NULL,'sys:log:delete','2',NULL,6,NULL,NULL,NULL,'2021-12-01 15:59:40','2021-12-01 15:59:40',0),(56,'8331d5f0900444109edc6cd83e54032e','93f101be493d11ecb7c2543344556776','代码生成','dev/code','','1','skill',0,NULL,'admin','admin','2021-12-09 06:21:28','2021-12-09 06:21:28',0),(57,'adf97607dbcc4de8b258ea17fc5af5ea','0','目录1','','','0','money',0,NULL,'admin','admin','2021-12-12 09:07:30','2021-12-12 09:07:30',0),(58,'d8c39450c345451cae92ff6fd82b5637','adf97607dbcc4de8b258ea17fc5af5ea','菜单1','sys/helo','大的','1','bug',1,NULL,'admin','admin','2021-12-12 09:08:52','2021-12-12 09:08:52',0),(59,'43061d67000e47de85346d3cc0b0a5b7','0','目录2','','','0','nested',3,NULL,'admin','admin','2021-12-12 09:30:14','2021-12-12 09:30:14',0),(60,'914552cf998c4624a3bb980169c2d015','43061d67000e47de85346d3cc0b0a5b7','菜单2','hello','asdfa','1','excel',0,NULL,'admin','admin','2021-12-12 09:30:48','2021-12-12 09:30:48',0);
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
                            `uuid` varchar(32) DEFAULT NULL COMMENT 'uuid',
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
INSERT INTO `sys_role` VALUES (1,'93f101be493d11ecb7c254e1ad134d77','管理员','管理员','SYS','SYS','2021-11-19 13:51:37','2021-11-19 13:51:39',0),(2,'0caadea8ae91427eb3d869df43d0fe41','开发人员','开发人员','admin','admin','2021-12-01 08:57:50','2021-12-01 08:57:50',0);
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
                                    `uuid` varchar(32) DEFAULT NULL COMMENT '唯一标示做关联',
                                    `role_uuid` varchar(32) DEFAULT NULL COMMENT '角色UUID',
                                    `menu_uuid` varchar(32) DEFAULT NULL COMMENT '菜单UUID',
                                    `creator` varchar(64) DEFAULT NULL COMMENT '创建人',
                                    `modifier` varchar(64) DEFAULT NULL COMMENT '修改人',
                                    `gmt_create` datetime NOT NULL COMMENT '创建时间',
                                    `gmt_modified` datetime NOT NULL COMMENT '修改时间',
                                    `deleted` tinyint(1) NOT NULL DEFAULT '0' COMMENT '逻辑删除',
                                    PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=191 DEFAULT CHARSET=utf8mb4 COMMENT='角色关联菜单表';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `sys_role_of_menu`
--

LOCK TABLES `sys_role_of_menu` WRITE;
/*!40000 ALTER TABLE `sys_role_of_menu` DISABLE KEYS */;
INSERT INTO `sys_role_of_menu` VALUES (85,'edc36fbe44b346309633f6a63540eeaa','0caadea8ae91427eb3d869df43d0fe41','93f101be493d11ecb7c254334455677','admin','admin','2021-12-01 08:57:50','2021-12-01 08:57:50',0),(86,'341f066eb9b74c3ca91525eb51ff3349','0caadea8ae91427eb3d869df43d0fe41','8fc52771493d11ecb7c254e1ad134d77','admin','admin','2021-12-01 08:57:50','2021-12-01 08:57:50',0),(87,'8382665c44704e2ba00e5716e530350f','0caadea8ae91427eb3d869df43d0fe41','8fc6eb1a493d11ecb7c254e1ad134d77','admin','admin','2021-12-01 08:57:50','2021-12-01 08:57:50',0),(88,'7582c9a49261487891bb3d357c20369a','0caadea8ae91427eb3d869df43d0fe41','8fc89835493d11ecb7c254e1ad134d77','admin','admin','2021-12-01 08:57:50','2021-12-01 08:57:50',0),(89,'6e50c43e8a934bd78116d7dafe763fd3','0caadea8ae91427eb3d869df43d0fe41','8fc99a2d493d11ecb7c254e1ad134d77','admin','admin','2021-12-01 08:57:50','2021-12-01 08:57:50',0),(90,'353de15fd3b942238e24b067c43a64e5','0caadea8ae91427eb3d869df43d0fe41','8fcaa825493d11ecb7c254e1ad134d77','admin','admin','2021-12-01 08:57:50','2021-12-01 08:57:50',0),(91,'32be576f04d54f9a8d0723b66991d259','0caadea8ae91427eb3d869df43d0fe41','93ebbcb2493d11ecb7c254e1ad134d77','admin','admin','2021-12-01 08:57:50','2021-12-01 08:57:50',0),(92,'00935ece12a248dda6f4c45cb8d78548','0caadea8ae91427eb3d869df43d0fe41','93ed48f8493d11ecb7c254e1ad134d77','admin','admin','2021-12-01 08:57:50','2021-12-01 08:57:50',0),(93,'ab645494c8334636bcc31d4011ab5d27','0caadea8ae91427eb3d869df43d0fe41','93ee29b6493d11ecb7c254e1ad134d77','admin','admin','2021-12-01 08:57:50','2021-12-01 08:57:50',0),(94,'31d1250e836449f484826b8fee0b91db','0caadea8ae91427eb3d869df43d0fe41','93f00a10493d11ecb7c254e1ad134d77','admin','admin','2021-12-01 08:57:50','2021-12-01 08:57:50',0),(95,'1de65d702d734f9fb53591f1f71e5c35','0caadea8ae91427eb3d869df43d0fe41','93f101be493d11ecb7c254e1ad134d77','admin','admin','2021-12-01 08:57:50','2021-12-01 08:57:50',0),(96,'72af74fd48f84871a3daab03a187eda0','0caadea8ae91427eb3d869df43d0fe41','0bb9b97e51f511eca3306106f8cc1608','admin','admin','2021-12-01 08:57:50','2021-12-01 08:57:50',0),(97,'b69b420baf4a4ecfb022dd6182337058','0caadea8ae91427eb3d869df43d0fe41','0bbe311651f511eca3306106f8cc1608','admin','admin','2021-12-01 08:57:50','2021-12-01 08:57:50',0),(98,'f50c633ba6034537895f074d9e53ef1f','0caadea8ae91427eb3d869df43d0fe41','0bbf3e9e51f511eca3306106f8cc1608','admin','admin','2021-12-01 08:57:50','2021-12-01 08:57:50',0),(99,'747994f0108c48e2bb700f755a3ec9dd','0caadea8ae91427eb3d869df43d0fe41','0bc18ab451f511eca3306106f8cc1608','admin','admin','2021-12-01 08:57:50','2021-12-01 08:57:50',0),(100,'fab2d2a63f8f43ff873fd6c6c75c932d','0caadea8ae91427eb3d869df43d0fe41','0bc60ce251f511eca3306106f8cc1608','admin','admin','2021-12-01 08:57:50','2021-12-01 08:57:50',0),(101,'850daeaac10b4cca89efe67673d61580','0caadea8ae91427eb3d869df43d0fe41','a2fd81e4527c11ecb2ccd76ea2e07fd8','admin','admin','2021-12-01 08:57:50','2021-12-01 08:57:50',0),(102,'ffe45ebe32ed4cb48a0e0108cc5d713b','0caadea8ae91427eb3d869df43d0fe41','a329b368527c11ecb2ccd76ea2e07fd8','admin','admin','2021-12-01 08:57:50','2021-12-01 08:57:50',0),(103,'3207f4b395d6439eba466e4ded9b01ed','0caadea8ae91427eb3d869df43d0fe41','a32e88ca527c11ecb2ccd76ea2e07fd8','admin','admin','2021-12-01 08:57:50','2021-12-01 08:57:50',0),(104,'33d27be81f68459e86adc90f9f493c3e','0caadea8ae91427eb3d869df43d0fe41','a330ec3c527c11ecb2ccd76ea2e07fd8','admin','admin','2021-12-01 08:57:50','2021-12-01 08:57:50',0),(105,'eb951d36ebab4cfab0b7c446856900c8','0caadea8ae91427eb3d869df43d0fe41','a3332a60527c11ecb2ccd76ea2e07fd8','admin','admin','2021-12-01 08:57:50','2021-12-01 08:57:50',0),(107,'cdd7be6839c141118c7374d237e08445','0caadea8ae91427eb3d869df43d0fe41','638a1e2c527d11ecb2ccd76ea2e07fd8','admin','admin','2021-12-01 08:57:50','2021-12-01 08:57:50',0),(108,'a5e2ea99983d4f109e825ca88c4c58f0','0caadea8ae91427eb3d869df43d0fe41','638b5742527d11ecb2ccd76ea2e07fd8','admin','admin','2021-12-01 08:57:50','2021-12-01 08:57:50',0),(109,'a6e057847282429f82478c989b9138e4','0caadea8ae91427eb3d869df43d0fe41','638c3d9c527d11ecb2ccd76ea2e07fd8','admin','admin','2021-12-01 08:57:50','2021-12-01 08:57:50',0),(110,'5cdf81bcf80041f4a735aefb55b45d66','0caadea8ae91427eb3d869df43d0fe41','638d62b2527d11ecb2ccd76ea2e07fd8','admin','admin','2021-12-01 08:57:50','2021-12-01 08:57:50',0),(111,'81ac9f6f388741a398d70f7f0ac48b85','0caadea8ae91427eb3d869df43d0fe41','93f101be493d11ecb7c2543344556776','admin','admin','2021-12-01 08:57:50','2021-12-01 08:57:50',0),(112,'22d909855d374be69c473da2bf71eb38','0caadea8ae91427eb3d869df43d0fe41','4a6ddb331d464050ac87f8c13f57facc','admin','admin','2021-12-01 08:57:50','2021-12-01 08:57:50',0),(163,'80cbf05467cd4bf594344c5e494051b5','93f101be493d11ecb7c254e1ad134d77','93f101be493d11ecb7c254334455677','admin','admin','2021-12-12 09:31:04','2021-12-12 09:31:04',0),(164,'ce8a9d96da254d29a373e97d78dc2838','93f101be493d11ecb7c254e1ad134d77','8fc52771493d11ecb7c254e1ad134d77','admin','admin','2021-12-12 09:31:04','2021-12-12 09:31:04',0),(165,'ca750ff1c334454c9bb8b123ad95fe12','93f101be493d11ecb7c254e1ad134d77','8fc6eb1a493d11ecb7c254e1ad134d77','admin','admin','2021-12-12 09:31:04','2021-12-12 09:31:04',0),(166,'bd47e86f1f25429ebdeae74504127d79','93f101be493d11ecb7c254e1ad134d77','8fc89835493d11ecb7c254e1ad134d77','admin','admin','2021-12-12 09:31:04','2021-12-12 09:31:04',0),(167,'89d9392ab0ff43018411e86e46a54dd9','93f101be493d11ecb7c254e1ad134d77','8fc99a2d493d11ecb7c254e1ad134d77','admin','admin','2021-12-12 09:31:04','2021-12-12 09:31:04',0),(168,'d505cbfd94d14853b721d3f99c535184','93f101be493d11ecb7c254e1ad134d77','8fcaa825493d11ecb7c254e1ad134d77','admin','admin','2021-12-12 09:31:04','2021-12-12 09:31:04',0),(169,'dace8fb7675c494cb956702fe9405d24','93f101be493d11ecb7c254e1ad134d77','93ebbcb2493d11ecb7c254e1ad134d77','admin','admin','2021-12-12 09:31:04','2021-12-12 09:31:04',0),(170,'bb2e6d1917d54a589b4106c9a4a7ba5a','93f101be493d11ecb7c254e1ad134d77','93ed48f8493d11ecb7c254e1ad134d77','admin','admin','2021-12-12 09:31:04','2021-12-12 09:31:04',0),(171,'8a5a9df38192478aafdd56b84e4975c9','93f101be493d11ecb7c254e1ad134d77','93ee29b6493d11ecb7c254e1ad134d77','admin','admin','2021-12-12 09:31:04','2021-12-12 09:31:04',0),(172,'c26ee2d6823643efa97f9710b1dba3ae','93f101be493d11ecb7c254e1ad134d77','93f00a10493d11ecb7c254e1ad134d77','admin','admin','2021-12-12 09:31:04','2021-12-12 09:31:04',0),(173,'37c45a01cb144279871634b22f7f75a5','93f101be493d11ecb7c254e1ad134d77','93f101be493d11ecb7c254e1ad134d77','admin','admin','2021-12-12 09:31:04','2021-12-12 09:31:04',0),(174,'1b45ffe0f5ca4de89d215578b7bab8d4','93f101be493d11ecb7c254e1ad134d77','0bb9b97e51f511eca3306106f8cc1608','admin','admin','2021-12-12 09:31:04','2021-12-12 09:31:04',0),(175,'59d739c7331a4008848a0bdb06fc40ee','93f101be493d11ecb7c254e1ad134d77','0bbe311651f511eca3306106f8cc1608','admin','admin','2021-12-12 09:31:04','2021-12-12 09:31:04',0),(176,'026424ff7fd342e08b05da07f370ff81','93f101be493d11ecb7c254e1ad134d77','0bbf3e9e51f511eca3306106f8cc1608','admin','admin','2021-12-12 09:31:04','2021-12-12 09:31:04',0),(177,'872a5245bd3845328fa975525adf2e64','93f101be493d11ecb7c254e1ad134d77','0bc18ab451f511eca3306106f8cc1608','admin','admin','2021-12-12 09:31:04','2021-12-12 09:31:04',0),(178,'88feb60ae0104c888a9ec498f1a90595','93f101be493d11ecb7c254e1ad134d77','0bc60ce251f511eca3306106f8cc1608','admin','admin','2021-12-12 09:31:04','2021-12-12 09:31:04',0),(179,'4b03bd4e40c843a98a0d201206751b59','93f101be493d11ecb7c254e1ad134d77','a2fd81e4527c11ecb2ccd76ea2e07fd8','admin','admin','2021-12-12 09:31:04','2021-12-12 09:31:04',0),(180,'e62255c4066d4c11990e218ab081dde7','93f101be493d11ecb7c254e1ad134d77','a329b368527c11ecb2ccd76ea2e07fd8','admin','admin','2021-12-12 09:31:04','2021-12-12 09:31:04',0),(181,'dfb2ee5a96584b1181d491bfb28c5f12','93f101be493d11ecb7c254e1ad134d77','a32e88ca527c11ecb2ccd76ea2e07fd8','admin','admin','2021-12-12 09:31:04','2021-12-12 09:31:04',0),(182,'1cfd682e16e948609d97293f20162ac8','93f101be493d11ecb7c254e1ad134d77','a330ec3c527c11ecb2ccd76ea2e07fd8','admin','admin','2021-12-12 09:31:04','2021-12-12 09:31:04',0),(183,'e8bf80898a444b7a883907c2dc016a67','93f101be493d11ecb7c254e1ad134d77','a3332a60527c11ecb2ccd76ea2e07fd8','admin','admin','2021-12-12 09:31:04','2021-12-12 09:31:04',0),(184,'e5ab5b9500304a72814d57b21282e861','93f101be493d11ecb7c254e1ad134d77','93f101be493d11ecb7c2543344556776','admin','admin','2021-12-12 09:31:04','2021-12-12 09:31:04',0),(185,'026c9a49e26148ef82ed605fc58f78ac','93f101be493d11ecb7c254e1ad134d77','4a6ddb331d464050ac87f8c13f57facc','admin','admin','2021-12-12 09:31:04','2021-12-12 09:31:04',0),(186,'59abc1b069474bfd9b71366ba8ef52f6','93f101be493d11ecb7c254e1ad134d77','8331d5f0900444109edc6cd83e54032e','admin','admin','2021-12-12 09:31:04','2021-12-12 09:31:04',0),(187,'251b930e6d1143a98a9205f7d1fec868','93f101be493d11ecb7c254e1ad134d77','adf97607dbcc4de8b258ea17fc5af5ea','admin','admin','2021-12-12 09:31:04','2021-12-12 09:31:04',0),(188,'660ea7ef39a44eb9ac5cbcc376f6da50','93f101be493d11ecb7c254e1ad134d77','d8c39450c345451cae92ff6fd82b5637','admin','admin','2021-12-12 09:31:04','2021-12-12 09:31:04',0),(189,'7379d22bc3dd47128bf17c72c7514aef','93f101be493d11ecb7c254e1ad134d77','43061d67000e47de85346d3cc0b0a5b7','admin','admin','2021-12-12 09:31:04','2021-12-12 09:31:04',0),(190,'0b2e62e7d5a842c7b790ac1860528ef4','93f101be493d11ecb7c254e1ad134d77','914552cf998c4624a3bb980169c2d015','admin','admin','2021-12-12 09:31:04','2021-12-12 09:31:04',0);
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
                            `uuid` varchar(32) DEFAULT NULL COMMENT '唯一标示做关联',
                            `username` varchar(64) DEFAULT NULL COMMENT '账号',
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
                            UNIQUE KEY `sys_user_user_id_uindex` (`account`)
) ENGINE=InnoDB AUTO_INCREMENT=11 DEFAULT CHARSET=utf8mb4 COMMENT='用户表';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `sys_user`
--

LOCK TABLES `sys_user` WRITE;
/*!40000 ALTER TABLE `sys_user` DISABLE KEYS */;
INSERT INTO `sys_user` VALUES (1,'9df9de08354f456c97fae0cdb3df214f','admin','$2a$10$7CQ/9w8BucRDnS3/cG2lXOA3y.9eEJUY6pidZGiNFQCU5Vu2q6ZwO','admin','devcloudadmin@qq.com',0,NULL,'admin','2021-11-12 22:48:15','2021-11-12 22:48:15',0,NULL),(2,'26220d63bfd345dabb2f114287965313','mikey','$2a$10$ZnuPGCEusk5tiKxAB/1lreynJxuvh4mqu8So6vUok/PCBCYP34.gi','mikey','mikey',0,NULL,NULL,'2021-12-01 09:39:53','2021-12-01 09:39:53',0,NULL),(3,'b8c1e673060c437ba0925f119538bdb6','fasf','$2a$10$ILTzdnK9HtI4BiTXneTWOOgiLid.QHTuDK0iV42EvH142xhuSheqy','fasdf','asfd@qq.com',0,'mikey','admin','2021-12-01 09:46:07','2021-12-01 09:46:07',1,NULL),(6,'841bf88b99ac45c4ba6a1189abe6bea8','adsfasdf','xxxxxxx','hhhh','dasfa@qq.com',0,NULL,NULL,'1970-01-01 00:00:00','1970-01-01 00:00:00',0,NULL),(9,'e4ea0c9574e1477091577c29a4a149a2','adsfafsdf','xxxxxxx','hhfhh','dafsfa@qq.com',0,NULL,NULL,'1970-01-01 00:00:00','1970-01-01 00:00:00',0,NULL);
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
                                    `uuid` varchar(32) DEFAULT NULL COMMENT '唯一标示做关联',
                                    `user_uuid` varchar(32) DEFAULT NULL COMMENT '用户UUID',
                                    `role_uuid` varchar(32) DEFAULT NULL COMMENT '角色UUID',
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

-- Dump completed on 2023-06-21 22:43:07
