-- MySQL dump 10.13  Distrib 5.7.28, for macos10.14 (x86_64)
--
-- Host: 127.0.0.1    Database: rwf
-- ------------------------------------------------------
-- Server version	5.7.28

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
-- Table structure for table `menu`
--

DROP TABLE IF EXISTS `menu`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `menu` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `parent_id` int(11) DEFAULT NULL,
  `path` varchar(255) DEFAULT NULL,
  `name` varchar(255) DEFAULT NULL,
  `component` varchar(255) DEFAULT NULL,
  `redirect` varchar(255) DEFAULT NULL,
  `title` varchar(255) DEFAULT NULL,
  `hideChildrenInMenu` tinyint(1) DEFAULT NULL,
  `icon` varchar(255) DEFAULT NULL,
  `hideMenu` tinyint(1) DEFAULT NULL,
  `hideBreadcrumb` tinyint(1) DEFAULT NULL,
  `currentActiveMenu` varchar(255) DEFAULT NULL,
  `ignoreKeepAlive` tinyint(1) DEFAULT NULL,
  `showMenu` tinyint(1) DEFAULT NULL,
  `frameSrc` varchar(255) DEFAULT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=22 DEFAULT CHARSET=utf8;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `menu`
--

LOCK TABLES `menu` WRITE;
/*!40000 ALTER TABLE `menu` DISABLE KEYS */;
INSERT INTO `menu` VALUES (1,NULL,' /dashboard           ',' Dashboard           ',' LAYOUT                     ',' /dashboard/analysis    ',' routes.dashboard.dashboard       ',NULL,' bx:bx-home      	   ',NULL,NULL,'                   ',NULL,NULL,'                    '),(2,1,' analysis             ',' Analysis            ',' /dashboard/analysis/index  ','                        ',' routes.dashboard.analysis        ',NULL,' bx:bx-home      	   ',NULL,NULL,' /dashboard        ',NULL,NULL,'                    '),(3,1,' workbench            ',' Workbench           ',' /dashboard/workbench/index ','                        ',' routes.dashboard.workbench       ',NULL,' bx:bx-home      	   ',NULL,NULL,' /dashboard        ',NULL,NULL,'                    '),(4,NULL,' /back                ',' PermissionBackDemo  ','                            ','                        ',' routes.demo.permission.back      ',NULL,'                 	   ',NULL,NULL,'                   ',NULL,NULL,'                    '),(5,4,' page                 ',' BackAuthPage        ',' /demo/permission/back/index','                        ',' routes.demo.permission.backPage  ',NULL,'                 	   ',NULL,NULL,'                   ',NULL,NULL,'                    '),(6,4,' btn                  ',' BackAuthBtn         ',' /demo/permission/back/Btn  ','                        ',' routes.demo.permission.backBtn   ',NULL,'                 	   ',NULL,NULL,'                   ',NULL,NULL,'                    '),(7,NULL,' /permission          ',' Permission          ',' LAYOUT                     ',' /permission/front/page ',' routes.demo.permission.permission',NULL,' carbon:user-role	   ',NULL,NULL,'                   ',NULL,NULL,'                    '),(8,7,' back                 ',' PermissionBackDemo  ','                            ','                        ',' routes.demo.permission.back  	  ',NULL,'                 	   ',NULL,NULL,'                   ',NULL,NULL,'                    '),(9,8,' page                 ',' BackAuthPage        ',' /demo/permission/back/index','                        ',' routes.demo.permission.backPage  ',NULL,'                 	   ',NULL,NULL,'                   ',NULL,NULL,'                    '),(10,8,' btn                  ',' BackAuthBtn         ',' /demo/permission/back/Btn  ','                        ',' routes.demo.permission.backBtn   ',NULL,'                 	   ',NULL,NULL,'                   ',NULL,NULL,'                    '),(11,NULL,' /level               ',' Level               ',' LAYOUT                     ',' /level/menu1/menu1-1   ',' routes.demo.level.level          ',NULL,' carbon:user-role	   ',NULL,NULL,'                   ',NULL,NULL,'                    '),(12,11,' menu1                ',' Menu1Demo           ','                            ','                        ',' Menu1                            ',NULL,'                 	   ',NULL,NULL,'                   ',NULL,NULL,'                    '),(13,12,' menu1-1              ',' Menu11Demo          ','                            ','                        ',' Menu1-1                          ',NULL,'                 	   ',NULL,NULL,'                   ',NULL,NULL,'                    '),(14,13,' menu1-1-1            ',' Menu111Demo         ',' /demo/level/Menu111        ','                        ',' Menu111                          ',NULL,'                 	   ',NULL,NULL,'                   ',NULL,NULL,'                    '),(15,12,' menu1-2              ',' Menu12Demo          ',' /demo/level/Menu12         ','                        ',' Menu1-2                          ',NULL,'                       ',NULL,NULL,'                   ',NULL,NULL,'                    '),(16,11,' menu2                ',' Menu2Demo           ',' /demo/level/Menu2          ','                        ',' Menu2                            ',NULL,'                 	   ',NULL,NULL,'                   ',NULL,NULL,'                    '),(17,NULL,' /system              ',' System              ',' LAYOUT                     ',' /system/account        ',' routes.demo.system.moduleName    ',NULL,' ion:settings-outline  ',NULL,NULL,'                   ',NULL,NULL,'                    '),(18,17,' account              ',' AccountManagement   ',' /demo/system/account/index ','                        ',' routes.demo.system.account       ',NULL,'               		   ',NULL,NULL,'                   ',NULL,NULL,'                    '),(19,17,' role                 ',' RoleManagement      ',' /demo/system/role/index    ','                        ',' routes.demo.system.role          ',NULL,'               		   ',NULL,NULL,'                   ',NULL,NULL,'                    '),(20,17,' menu                 ',' MenuManagement      ',' /demo/system/menu/index    ','                        ',' routes.demo.system.menu          ',NULL,'               		   ',NULL,NULL,'                   ',NULL,NULL,'                    '),(21,17,' dept                 ',' DeptManagement      ',' /demo/system/dept/index    ','                        ',' routes.demo.system.dept          ',NULL,'               		   ',NULL,NULL,'                   ',NULL,NULL,'                    ');
/*!40000 ALTER TABLE `menu` ENABLE KEYS */;
UNLOCK TABLES;
/*!40103 SET TIME_ZONE=@OLD_TIME_ZONE */;

/*!40101 SET SQL_MODE=@OLD_SQL_MODE */;
/*!40014 SET FOREIGN_KEY_CHECKS=@OLD_FOREIGN_KEY_CHECKS */;
/*!40014 SET UNIQUE_CHECKS=@OLD_UNIQUE_CHECKS */;
/*!40101 SET CHARACTER_SET_CLIENT=@OLD_CHARACTER_SET_CLIENT */;
/*!40101 SET CHARACTER_SET_RESULTS=@OLD_CHARACTER_SET_RESULTS */;
/*!40101 SET COLLATION_CONNECTION=@OLD_COLLATION_CONNECTION */;
/*!40111 SET SQL_NOTES=@OLD_SQL_NOTES */;

-- Dump completed on 2023-07-09 23:48:39
