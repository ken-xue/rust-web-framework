import { BasicPageParams, BasicFetchResult } from '/@/api/model/baseModel';

export type AccountParams = BasicPageParams & {
  account?: string;
  nickname?: string;
};

export type RoleParams = {
  roleName?: string;
  status?: string;
};

export type RolePageParams = BasicPageParams & RoleParams;

export type DeptParams = {
  deptName?: string;
  status?: string;
};

export type MenuParams = {
  id?: number;
  parentUuid?: string;
  menuType?: string;
  name?: string;
  status?: string;
  path?: string;
  order?: number;
  permission?: string;
};

export type DeleteParam = {
  ids?: number[];
};

export interface AccountListItem {
  id: string;
  account: string;
  email: string;
  nickname: string;
  role: number;
  createTime: string;
  remark: string;
  status: number;
}

export interface DeptListItem {
  id: string;
  name: string;
  order: string;
  createTime: string;
  remark: string;
  status: number;
}

export interface MenuListItem {
  id: string;
  uuid: string;
  parentUuid: string;
  order: string;
  gmtCreate: string;
  status: number;
  icon: string;
  component: string;
  api: string;
  permission: string;
}

export interface RoleListItem {
  id: string;
  name: string;
  // roleValue: string;
  // status: number;
  // orderNo: string;
  // createTime: string;
}

/**
 * @description: Request list return value
 */
export type AccountListGetResultModel = BasicFetchResult<AccountListItem>;

export type DeptListGetResultModel = BasicFetchResult<DeptListItem>;

export type MenuListGetResultModel = BasicFetchResult<MenuListItem>;

export type RolePageListGetResultModel = BasicFetchResult<RoleListItem>;

export type RoleListGetResultModel = RoleListItem[];
