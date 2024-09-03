import { requestPermission, isPermissionGranted } from '@tauri-apps/api/notification';

export const checkNotificationPermission = async () => {
	let permissionGranted = await isPermissionGranted();
	if (!permissionGranted) {
		const permission = await requestPermission();
		permissionGranted = permission === 'granted';
	}
	return permissionGranted;
}