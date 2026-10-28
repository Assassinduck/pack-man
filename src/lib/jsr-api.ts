import { invoke } from "@tauri-apps/api/core";

export function requestPackage(value: string): Promise<string> {
  return invoke<string>("request_package", { value });
}
