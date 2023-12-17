"use client";

import { Line, LineChart, ResponsiveContainer, XAxis, YAxis } from "recharts";
import { TaskEvent } from "./TaskView";
interface EventDurationByDate {
  duration: number;
  date: string;
}

const getTaskDurationByDay = (events: TaskEvent[]): EventDurationByDate[] => {
  const durationByDay: { [index: string]: EventDurationByDate } = {};
  console.log({ events });
  events.forEach((taskEvent) => {
    const eventDate = new Date(taskEvent.date_began).toLocaleDateString();
    durationByDay[eventDate]
      ? (durationByDay[eventDate].duration += taskEvent.duration)
      : (durationByDay[eventDate] = {
          duration: taskEvent.duration,
          date: eventDate,
        });
  });
  return Object.values(durationByDay);
};

interface TimeChartProps {
  taskEvents: TaskEvent[];
}

const TaskEventLineChart = ({ taskEvents }: TimeChartProps) => {
  return (
    <ResponsiveContainer>
      <LineChart data={getTaskDurationByDay(taskEvents)}>
        <XAxis dataKey="date" />
        <YAxis tick={false} />
        <Line type="monotone" dataKey={"duration"} stroke="#8884d8" />
      </LineChart>
    </ResponsiveContainer>
  );
};

export default TaskEventLineChart;
