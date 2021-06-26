import React, { useEffect } from 'react';
import { useDispatch, useSelector } from 'react-redux';
import { makeStyles } from '@material-ui/core';

const useStyles = makeStyles({
    root: {
        height: '60.4rem',
    },
})

const Upload = () => {
    const { contractInstance } = useSelector((state) => state.artToken);

    useEffect(() => {
        console.log(contractInstance);
    }, [])

    const classes = useStyles();
    return(
        <div className={classes.root}>
            <h1>Upload page</h1>
        </div>
    );
}

export default Upload;
