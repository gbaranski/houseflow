import React from 'react';
import Typography from '@material-ui/core/Typography';
import Link from '@material-ui/core/Link';

function Copyright() {
  return (
    <Typography variant="body2" color="textSecondary" align="center">
      {'Made by '}
      <Link target="_blank" href="https://github.com/gbaranski">
        Grzegorz Baranski
      </Link>{' '}
    </Typography>
  );
}

export default Copyright;
