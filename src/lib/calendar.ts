import { eachDayOfInterval, lastDayOfWeek, startOfWeek, subWeeks } from 'date-fns/fp';

export const allDaysOfWeeksBefore = (startDate: Date, weekNum: number) => {
	const lastDay = lastDayOfWeek(startDate)
	const firstDay = startOfWeek(subWeeks(weekNum - 1)(startDate))
	const allDays = eachDayOfInterval({ start: firstDay, end: lastDay })

	return allDays.reduce<Date[][]>((acc, day) => {
		const currentWeek = acc[0]

		if (!currentWeek || currentWeek.length === 7) {
			return [[day], ...acc]
		} else {
			currentWeek.push(day)
		}

		return acc;
	}, [] as Date[][])
}