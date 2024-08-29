import { type DayRecord } from "$store/day-records"
import { sum } from 'lodash-es';
import type { Interval } from 'date-fns';
import { lightFormat } from 'date-fns/fp';

export const getRecordByDate = ($dayRecords: DayRecord[], date: Date) => $dayRecords.find(r => r.date === date.getTime())

export const getActiveHoursByRecord = (dayRecord?: DayRecord) => {
    if (!dayRecord) return 0
    const microseconds = dayRecord.records.reduce((acc, record) => {
        acc += record.duration
        return acc
    }, 0)
    return microseconds / 1000 / 60 / 60
}

export const getIntervalsByRecord = (dayRecord: DayRecord) => {
	return dayRecord.records.map((record) => {
		return {
			start: record.start_time,
			end: record.end_time,
		} as Interval
	})
}
export const getActivityScoreByHour = (hours: number) => {
    if (hours < 0) return 0;
    else if (hours < 0.5) return 2;
    else if (hours < 1) return 3;
    else if (hours < 1.5) return 4;
    else if (hours < 2) return 5;
    else if (hours < 4) return 6;
    else if (hours < 7) return 7;
    else if (hours < 12) return 8;
    else if (hours < 16) return 9;
    // else if (hours < 20) return 9.5;
    else return 9;
}

export const sumHoursInRecords = (dayRecords: DayRecord[]) => {
	return sum(dayRecords.map(getActiveHoursByRecord)).toFixed(1)
}

export const formatDayRecord = (dayRecord: DayRecord) => {
    const dateStr = lightFormat('yyyy-MM-dd')(dayRecord.date)

	return `${dateStr}: ${getActiveHoursByRecord(dayRecord).toFixed(1)}h`
}