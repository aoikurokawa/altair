import React from 'react';
import { useSelector } from 'react-redux';
import { makeStyles } from '@material-ui/core/styles';
import CircularProgress from '@material-ui/core/CircularProgress';
import { Container } from '@material-ui/core';

const useStyles = makeStyles((theme) => ({
  root: {
    display: 'flex',
    '& > * + *': {
      marginLeft: theme.spacing(2),
    },
  },
}));

const Progress = () => {
  const { isLoading } = useSelector((state) => state.progress);
  const classes = useStyles();

  return (
    <>
      {
        isLoading ?
          <Container className={classes.root}>
            <CircularProgress />
          </Container>
          :
          <></>
      }
    </>

  );
}

export default Progress;
