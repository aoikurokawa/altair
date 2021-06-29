import React from 'react';
import { useSelector } from 'react-redux';
import { makeStyles } from '@material-ui/core/styles';
import CircularProgress from '@material-ui/core/CircularProgress';
import { Container } from '@material-ui/core';
import { PulseLoader } from 'react-spinners';

const useStyles = makeStyles((theme) => ({
  root: {
    display: 'flex',
    '& > * + *': {
      marginLeft: theme.spacing(2),

    },
  },
  loader: {
    top: '50%',
    left: '44%', 
    position: 'absolute',
  },
}));

const Loader = () => {
  const { isLoading } = useSelector((state) => state.progress);
  const classes = useStyles();

  const cssOfRoot = document.getElementById('root');
  if (isLoading) {
    cssOfRoot.style.opacity = "0.6";
  } else {
    cssOfRoot.style.opacity = "";    
  }
  
  return (
    <>
      {
        isLoading ?
          <div className={classes.loader}>
            <PulseLoader size="30" margin="2" color="#62DEC2" />
          </div>
          :
          <></>
      }
    </>
  );
}

export default Loader;
