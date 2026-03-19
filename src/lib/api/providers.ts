import { invoke } from '@tauri-apps/api/core';
import type { Provider, CreateProviderRequest, UpdateProviderRequest } from '../types/provider';

export const providerApi = {
  async list(): Promise<Provider[]> {
    return invoke<Provider[]>('list_providers');
  },

  async get(id: string): Promise<Provider> {
    return invoke<Provider>('get_provider', { id });
  },

  async create(request: CreateProviderRequest): Promise<Provider> {
    return invoke<Provider>('create_provider', { request });
  },

  async update(request: UpdateProviderRequest): Promise<Provider> {
    return invoke<Provider>('update_provider', { request });
  },

  async delete(id: string): Promise<void> {
    return invoke('delete_provider', { id });
  },

  async getApiKey(id: string): Promise<string> {
    return invoke<string>('get_api_key', { id });
  },

  async copyToClipboard(text: string): Promise<void> {
    return invoke('copy_to_clipboard', { text });
  },
};