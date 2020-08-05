import React, { useEffect } from 'react';
import Table from '@material-ui/core/Table';
import TableBody from '@material-ui/core/TableBody';
import TableCell from '@material-ui/core/TableCell';
import TableHead from '@material-ui/core/TableHead';
import TableRow from '@material-ui/core/TableRow';
import Title from '../../components/title';
import { RequestHistory } from '@gbaranski/types';
import { parseDateToDateString } from '../../utils';
import { getRequestHistory } from '../../services/firebase';

export default function Requests() {
  const [requestHistory, setRequestHistory] = React.useState<any>([{}]);

  useEffect(() => {
    getRequestHistory().then((reqHistory: RequestHistory[]) => {
      setRequestHistory(
        reqHistory.sort(
          (a: RequestHistory, b: RequestHistory) => b.unixTime - a.unixTime,
        ),
      );
    });
  }, []);

  return (
    <React.Fragment>
      <Title>Recent Requests</Title>
      <Table size="small">
        <TableHead>
          <TableRow>
            <TableCell>Date</TableCell>
            <TableCell>User</TableCell>
            <TableCell>Request</TableCell>
            <TableCell>IP</TableCell>
            <TableCell>Country</TableCell>
            <TableCell>User-agent</TableCell>
          </TableRow>
        </TableHead>
        <TableBody>
          {requestHistory.map((row: RequestHistory, index: number) => (
            <TableRow key={index}>
              <TableCell>
                {parseDateToDateString(new Date(row.unixTime))}
              </TableCell>
              <TableCell>{row.user}</TableCell>
              <TableCell>{row.requestPath}</TableCell>
              <TableCell>{row.ip}</TableCell>
              <TableCell>{row.country}</TableCell>
              <TableCell>{row.userAgent}</TableCell>
            </TableRow>
          ))}
        </TableBody>
      </Table>
    </React.Fragment>
  );
}
