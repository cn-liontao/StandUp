import { invoke } from '@tauri-apps/api/tauri';
import { listen } from '@tauri-apps/api/event';

interface SyncOptions<T> {
	initCmd: string;
	updateEvent: string;
	update: (this: void, payload: T) => void;
}

export function createSyncStore<T>(syncOptions: SyncOptions<T>) {
	return {
		init: () => {
			invoke<T>(syncOptions.initCmd).then((payload) => {
				syncOptions.update(payload)
			})
		},
		listen: () => {
			return listen<T>(syncOptions.updateEvent, (event) => {
				console.log('backend update:', event);
				syncOptions.update(event.payload);
			})
		}
	}
}