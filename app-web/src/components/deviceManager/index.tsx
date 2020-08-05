import React from 'react';
import Button from '@material-ui/core/Button';
import Title from '../title';
import ButtonGroup from '@material-ui/core/ButtonGroup';

interface DeviceManagerButton {
  onClick: any;
  innerText: string;
}

function CreateButtons(props: any) {
  return props.data.map((button: DeviceManagerButton) => (
    <Button color="primary" onClick={button.onClick} variant="outlined">
      {button.innerText}
    </Button>
  ));
}

function DeviceManager(props: { data: DeviceManagerButton[] }) {
  return (
    <React.Fragment>
      <Title>Manage Device</Title>
      <ButtonGroup
        color="primary"
        aria-label="outlined primary button group grouped"
      >
        <CreateButtons data={props.data} root grouped />
      </ButtonGroup>
    </React.Fragment>
  );
}

export default DeviceManager;
