import React, { useEffect, useState } from 'react';
import { Grid, CircularProgress } from '@material-ui/core';
import { useDispatch, useSelector } from 'react-redux';
import { makeStyles } from '@material-ui/core';
import { useMoralis } from 'react-moralis';

import { artTokenAction } from '../actions/artTokenAction';
import UploadArt from '../components/UploadArt';
import IpfsForm from '../components/IpfsForm';


const useStyles = makeStyles({
    root: {
        height: '60.4rem',
        flexGrow: 1,
    },
    uploadContainer: {
        padding: '2rem',
    },
    circular: {
        height: '61rem',
        textAlign: 'center',
        paddingTop: '50%',
    },
})

const UploadPage = () => {
    const dispatch = useDispatch();
    const { isUploading } = useMoralis();

    useEffect(() => {
        dispatch(artTokenAction());
    }, []);

    const classes = useStyles();
    return (
        <div>
            {isUploading ?
                <div className={classes.circular}>
                    <CircularProgress />
                </div>
                :
                <Grid className={classes.root}>
                    <Grid container>
                        <Grid item xs={6} className={classes.uploadContainer}>
                            <UploadArt />
                        </Grid>
                        <Grid item xs={6}>
                            <IpfsForm />
                        </Grid>
                    </Grid>
                </Grid>
            }
        </div>



    );
}

export default UploadPage;
