"use client";

import { Task } from "@/app/bandit/dashboard/page";
import { BarChart, ResponsiveContainer } from "recharts";

const DashBoardRecentTaskBarChart = ({ tasks }: { tasks: Task[] }) => {
  <ResponsiveContainer>
    <BarChart></BarChart>
  </ResponsiveContainer>;
};

export default DashBoardRecentTaskBarChart;
