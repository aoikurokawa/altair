import React from 'react';
import { useHistory } from "react-router-dom";
import { makeStyles } from '@material-ui/core/styles';
import SpeedDial from '@material-ui/lab/SpeedDial';
import SpeedDialIcon from '@material-ui/lab/SpeedDialIcon';
import SpeedDialAction from '@material-ui/lab/SpeedDialAction';
import FileCopyIcon from '@material-ui/icons/FileCopyOutlined';
import SaveIcon from '@material-ui/icons/Save';
import PrintIcon from '@material-ui/icons/Print';
import ShareIcon from '@material-ui/icons/Share';
import FavoriteIcon from '@material-ui/icons/Favorite';
import PublishIcon from '@material-ui/icons/Publish';
import GavelIcon from '@material-ui/icons/Gavel';
import { useMoralis } from 'react-moralis';

const useStyles = makeStyles((theme) => ({
  root: {
    transform: 'translateZ(0px)',
    flexGrow: 1,
  },
  exampleWrapper: {
    position: 'relative',
  },
  radioGroup: {
    margin: theme.spacing(1, 0),
  },
  speedDial: {
    position: 'absolute',
    '&.MuiSpeedDial-directionUp, &.MuiSpeedDial-directionLeft': {
      bottom: theme.spacing(2),
      right: theme.spacing(2),
    },
    '&.MuiSpeedDial-directionDown, &.MuiSpeedDial-directionRight': {
      top: theme.spacing(2),
      left: theme.spacing(2),
    },
  },
}));

const actions = [
  { icon: <PublishIcon />, name: 'Upload', pathname: 'upload' },
  { icon: <GavelIcon />, name: 'Auction', pathname: 'auction' },
  { icon: <FileCopyIcon />, name: 'Copy', pathname: '' },
  { icon: <SaveIcon />, name: 'Save', pathname: '' },
  { icon: <PrintIcon />, name: 'Print', pathname: '' },
  { icon: <ShareIcon />, name: 'Share', pathname: '' },
  { icon: <FavoriteIcon />, name: 'Like', pathname: '' },
];

const SpeedDials = () => {
  let history = useHistory();
  const classes = useStyles();
  const [direction, setDirection] = React.useState('up');
  const [open, setOpen] = React.useState(false);
  const [hidden, setHidden] = React.useState(false);
  const { isAuthenticated } = useMoralis();

  const handleClose = () => {
    setOpen(false);
  }

  const handleOpen = () => {
    setOpen(true);
  };

  return (
    <div className={classes.root}>
      <div className={classes.exampleWrapper}>
        <SpeedDial
          ariaLabel="SpeedDial example"
          className={classes.speedDial}
          hidden={hidden}
          icon={<SpeedDialIcon />}
          onClose={handleClose}
          onOpen={handleOpen}
          open={open}
          direction={direction}
        >
          {actions.map((action) => (
            <SpeedDialAction
              key={action.name}
              icon={action.icon}
              tooltipTitle={action.name}
              onClick={() => {
                if (!isAuthenticated) {
                  alert("Please Login");
                } else {
                  history.push(`/${action.pathname}`)
                }
              }}
            />
          ))}
        </SpeedDial>
      </div>
    </div>
  );
}

export default SpeedDials;
