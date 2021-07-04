import React from 'react';
import { useDispatch, useSelector } from 'react-redux';
import Avatar from '@material-ui/core/Avatar';
import Button from '@material-ui/core/Button';
import TextField from '@material-ui/core/TextField';
import PhotoIcon from '@material-ui/icons/Photo';
import Typography from '@material-ui/core/Typography';
import { Link } from '@material-ui/core';
import { makeStyles } from '@material-ui/core/styles';

import { mint } from '../actions/artTokenAction';

const useStyles = makeStyles((theme) => ({
    paper: {
        margin: theme.spacing(8, 4),
        display: 'flex',
        flexDirection: 'column',
        alignItems: 'center',
    },
    avatar: {
        margin: theme.spacing(1),
        backgroundColor: theme.palette.success.light,
    },
    form: {
        width: '100%', // Fix IE 11 issue.
        marginTop: theme.spacing(1),
    },
    submit: {
        margin: theme.spacing(3, 0, 2),
    },
}));

const IpfsForm = () => {
    const classes = useStyles("");
    const dispatch = useDispatch();
    const { ipfsHash, ipfsUrl, name } = useSelector((state) => state.artToken);

    const mintHandler = () => {
        dispatch(mint(ipfsHash, ipfsUrl, name));
    }

    return (
        <div className={classes.paper}>
            <Avatar className={classes.avatar}>
                <PhotoIcon />
            </Avatar>
            <Typography component="h1" variant="h5">
                Your NFT
            </Typography>
            <form className={classes.form} noValidate>
                <TextField
                    variant="outlined"
                    margin="normal"
                    fullWidth
                    id="hash"
                    label="IPFS Hash"
                    name="hash"
                    autoComplete="hash"
                    value={ipfsHash}
                    disabled
                />
                <TextField
                    variant="outlined"
                    margin="normal"
                    fullWidth
                    name="url"
                    label="IPFS URL"
                    id="url"
                    autoComplete="url"
                    value={ipfsUrl}
                    disabled
                />
                <Typography component="p" variant="inherit">
                    <Link href={ipfsUrl} target="_blank" color="primary">Visit IPFS that your work uploaded</Link>
                </Typography>
                <Button
                    fullWidth
                    variant="contained"
                    color="primary"
                    className={classes.submit}
                    onClick={mintHandler}
                >
                    CREATE NFT
                </Button>
            </form>
        </div>
    );
}

export default IpfsForm;
