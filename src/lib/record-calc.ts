import { type DayRecord } from "$store/day-records"
import { last, sum } from 'lodash-es';
import { type Interval, isToday } from 'date-fns';
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
    else if (hours < 1) return 2;
    else if (hours < 2) return 3;
		else if (hours < 3) return 4;
    else if (hours < 4) return 5;
    else if (hours < 5) return 6;
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
    let activeHours = getActiveHoursByRecord(dayRecord);
    if (isToday(dayRecord.date) && last(dayRecord.records)?.end_time === 0) {
        const standingRecord = last(dayRecord.records)!
        activeHours += (Date.now() - standingRecord.start_time) / 1000 / 60 / 60;
    }

	return `${dateStr}: ${activeHours.toFixed(2)}h`
}
