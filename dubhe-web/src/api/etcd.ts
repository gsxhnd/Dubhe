import http from "@/utils/http";
import { AxiosResponse } from "axios";

interface EtcdConfigTag {
  key: string;
  value: string;
}

interface EtcdConfig {
  name: string;
  addr: Array<string>;
  tag?: Array<EtcdConfigTag>;
}

export const getConfigEtcd = (): Promise<EtcdConfig[]> =>
  http.get("/api/v1/etcd/config");

export const saveConfigEtcd = (
  data: EtcdConfig[]
): Promise<AxiosResponse<any, any>> => http.get("/api/v1/etcd/config");

export const getEctdList = (): Promise<AxiosResponse<any, any>> =>
  http.get("/api/v1/etcd/list");

export const getEtcdKeys = (
  server_name: string
): Promise<AxiosResponse<any, any>> =>
  http.get(`api/v1/etcd/query/${server_name}/keys`);

export const getEtcdValueByKey = (
  server_name: string,
  key: string
): Promise<AxiosResponse<any, any>> =>
  http.get(`/api/v1/etcd/query/${server_name}/${key}`);

export const saveEtcdValueByKey = (
  server_name: string,
  key: string,
  value: string
): Promise<AxiosResponse<any, any>> => {
  return http.put(
    `/api/v1/etcd/query/${server_name}/${key}`,
    { data: value },
    {
      headers: { "Content-Type": "text/plain" },
    }
  );
};

export const deleteEtcdValueByKey = (
  server_name: string,
  key: string
): Promise<AxiosResponse<any, any>> =>
  http.delete(`/api/v1/etcd/query/${server_name}/${key}`);
