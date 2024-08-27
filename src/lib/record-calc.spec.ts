import { expect, test } from 'vitest';
import { getActiveHoursByRecord } from '$lib/record-calc';

test('getActiveHoursByRecord', () => {
	expect(getActiveHoursByRecord({
		date: 0,
		records: [
			{ start_time: 0, end_time: 0, duration: 1000 * 60 * 50 },
			{ start_time: 0, end_time: 0, duration: 1000 * 60 * 30 },
			{ start_time: 0, end_time: 0, duration: 1000 * 60 * 40 },
		]
	})).toBe((50 + 30 + 40) / 60)
})