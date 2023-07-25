import {
  AccountParams,
  DeptListItem,
  MenuParams,
  RoleParams,
  RolePageParams,
  MenuListGetResultModel,
  DeptListGetResultModel,
  AccountListGetResultModel,
  RolePageListGetResultModel,
  RoleListGetResultModel,
  DeleteParam,
} from './model/systemModel';
import { defHttp } from '/@/utils/http/axios';

enum Api {
  AccountList = '/system/getAccountList',
  UserPage = '/v1/system/user/page',
  UpdatePassword = '/v1/system/user/password',
  IsAccountExist = '/system/accountExist',
  DeptList = '/v1/system/dept/list',
  setRoleStatus = '/system/setRoleStatus',
  MenuList = '/v1/system/menu/list',
  MenuDelete = '/v1/system/menu/delete',
  MenuAdd = '/v1/system/menu/add',
  MenuUpdate = '/v1/system/menu/update',
  RolePageList = '/v1/system/role/page',
  GetAllRoleList = '/system/getAllRoleList',
}

export const getAccountList = (params: AccountParams) =>
  defHttp.get<AccountListGetResultModel>({ url: Api.AccountList, params });

export const getUserPage = (params: AccountParams) =>
  defHttp.post<AccountListGetResultModel>({ url: Api.UserPage, params });

export const getDeptList = (params?: DeptListItem) =>
  defHttp.post<DeptListGetResultModel>({ url: Api.DeptList, params });

export const getMenuList = (params?: MenuParams) =>
  defHttp.post<MenuListGetResultModel>({ url: Api.MenuList, params });
export const deleteMenu = (params?: DeleteParam) => defHttp.delete({ url: Api.MenuDelete, params });

export const addMenu = (params?: MenuParams) => defHttp.post({ url: Api.MenuAdd, params });
export const updateMenu = (params?: MenuParams) => defHttp.put({ url: Api.MenuUpdate, params });

export const getRoleListByPage = (params?: RolePageParams) =>
  defHttp.post<RolePageListGetResultModel>({ url: Api.RolePageList, params });

export const getAllRoleList = (params?: RoleParams) =>
  defHttp.get<RoleListGetResultModel>({ url: Api.GetAllRoleList, params });

export const setRoleStatus = (id: number, status: string) =>
  defHttp.post({ url: Api.setRoleStatus, params: { id, status } });

export const isAccountExist = (account: string) =>
  defHttp.post({ url: Api.IsAccountExist, params: { account } }, { errorMessageMode: 'none' });

export const updatePassword = (params) => defHttp.post({ url: Api.UpdatePassword, params });
