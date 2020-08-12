import React from 'react';
// import Divider from '@material-ui/core/Divider';
// import Title from '../../components/title';
// import WarningRoundedIcon from '@material-ui/icons/WarningRounded';
// import CheckCircleIcon from '@material-ui/icons/CheckCircle';
// import List from '@material-ui/core/List';
// import ListItem from '@material-ui/core/ListItem';
// import ListItemAvatar from '@material-ui/core/ListItemAvatar';
// import ListItemText from '@material-ui/core/ListItemText';
// import { getDeviceStatus } from '../../requests';

export default function Status() {
  return <>To fix</>;
  // const [deviceStatus, setDeviceStatus] = React.useState<DeviceStatus>({
  //   alarmclock: false,
  //   watermixer: false,
  //   gate: false,
  //   garage: false,
  // });

  // useEffect(() => {
  //   getDeviceStatus().then((res: DeviceStatus) => {
  //     setDeviceStatus(res);
  //     console.log(res);
  //   });
  // }, []);

  // const GetAvatar = (props: {state: boolean}) => {
  //   if (props.state) {
  //     return <CheckCircleIcon style={{color: 'rgb(32, 199, 155)'}} />;
  //   } else {
  //     return <WarningRoundedIcon style={{color: 'rgb(244, 188, 58)'}} />;
  //   }
  // };
  // return (
  //   <React.Fragment>
  //     <Title>Devices Status</Title>
  //     <List>
  //       <ListItem key={1}>
  //         <ListItemText primary="Alarmclock" secondary="192.168.1.110" />
  //         <ListItemAvatar>
  //           <GetAvatar state={deviceStatus.alarmclock} />
  //         </ListItemAvatar>
  //       </ListItem>
  //       <ListItem key={2}>
  //         <ListItemText primary="Watermixer" secondary="192.168.1.120" />
  //         <ListItemAvatar>
  //           <GetAvatar state={deviceStatus.watermixer} />
  //         </ListItemAvatar>
  //       </ListItem>
  //       <Divider variant="inset" component="li" />
  //       <ListItem key={3}>
  //         <ListItemText primary="Gate" secondary="192.168.1.1xx" />
  //         <ListItemAvatar>
  //           <GetAvatar state={deviceStatus.gate} />
  //         </ListItemAvatar>
  //       </ListItem>
  //       <Divider variant="inset" component="li" />
  //       <ListItem key={4}>
  //         <ListItemText primary="Garage" secondary="192.168.1.1xx" />
  //         <ListItemAvatar>
  //           <GetAvatar state={deviceStatus.garage} />
  //         </ListItemAvatar>
  //       </ListItem>
  //       <Divider variant="inset" component="li" />
  //     </List>
  //   </React.Fragment>
  // );
}
