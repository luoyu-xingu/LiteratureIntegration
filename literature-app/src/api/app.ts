import { tauriInvoke } from './client';

export const selectRootDir = () => tauriInvoke<string>('select_root_dir');
export const getRootDir = () => tauriInvoke<string>('get_root_dir');
export const setRootDir = (path: string) => tauriInvoke<string>('set_root_dir', { path });
