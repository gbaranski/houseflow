import React from 'react';
import {useTheme} from '@material-ui/core/styles';
import {
  LineChart,
  Line,
  XAxis,
  YAxis,
  Label,
  ResponsiveContainer,
} from 'recharts';
import Title from '../../components/title';

// Generate Sales Data
function createData(time: string, amount: number | undefined) {
  return {time, amount};
}

const data = [
  createData('00:00', 24.5),
  createData('03:00', 23.5),
  createData('06:00', 25.6),
  createData('09:00', 23),
  createData('12:00', 26.5),
  createData('15:00', 21.2),
  createData('18:00', 23.3),
  createData('21:00', 26.1),
  createData('24:00', 24.5),
];

export default function Chart() {
  const theme = useTheme();

  return (
    <React.Fragment>
      <Title>Last 24 hours temperature</Title>
      <ResponsiveContainer>
        <LineChart
          data={data}
          margin={{
            top: 16,
            right: 16,
            bottom: 0,
            left: 24,
          }}>
          <XAxis dataKey="time" stroke={theme.palette.text.secondary} />
          <YAxis stroke={theme.palette.text.secondary}>
            <Label
              angle={270}
              position="left"
              style={{textAnchor: 'middle', fill: theme.palette.text.primary}}>
              Temperature (°C)
            </Label>
          </YAxis>
          <Line
            type="monotone"
            dataKey="amount"
            stroke={theme.palette.primary.main}
            dot={false}
          />
        </LineChart>
      </ResponsiveContainer>
    </React.Fragment>
  );
}
