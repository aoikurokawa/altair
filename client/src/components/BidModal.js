import React, { useState } from 'react';
import { useDispatch, useSelector } from 'react-redux';
import { makeStyles } from '@material-ui/core';
import { Modal } from '@material-ui/core';
import { Backdrop } from '@material-ui/core';
import { Fade } from '@material-ui/core';
import { Button } from '@material-ui/core';
import { Container } from '@material-ui/core';
import { Grid } from '@material-ui/core';

import { updateIsSell } from '../actions/artTokenAction';
import { bidHandlerAction } from '../actions/loadBlockchainAction';

const useStyles = makeStyles((theme) => ({
    modal: {
        display: 'flex',
        alignItems: 'center',
        justifyContent: 'center',

    },
    paper: {
        backgroundColor: theme.palette.background.paper,
        border: '2px solid #000',
        boxShadow: theme.shadows[5],
        padding: theme.spacing(2, 4, 3),
        width: '50%',
        height: '30%',
    },
    inputDiv: {
        display: 'flex',
        alignContent: 'center',
        alignItems: 'center',
    },
    input: {
        width: '80%',
        padding: '5px',
        fontSize: '20px',
    },

    buttonContainer: {
        textAlign: 'center',
        padding: '20px',
    },
}))

const BidModal = () => {
    const [price, setPrice] = useState(0);
    const classes = useStyles();
    const { isModalVisible, functionType, title, objectId } = useSelector((state) => state.modal);
    const { nftDetail } = useSelector((state) => state.artToken);
    const dispatch = useDispatch();


    const handleClose = () => {
        dispatch({
            type: "CLOSE_MODAL",
        });
    };

    const bidHandler = () => {
        dispatch(bidHandlerAction(nftDetail.attributes["TokenId"], price));
        dispatch({
            type: 'CLOSE_MODAL'
        });
    }

    const handleAuction = () => {
        dispatch(updateIsSell(objectId));
    }

    return (
        <Modal
            aria-labelledby="transition-modal-title"
            aria-describedby="transition-modal-description"
            className={classes.modal}
            open={isModalVisible}
            onClose={handleClose}
            closeAfterTransition
            BackdropComponent={Backdrop}
            BackdropProps={{
                timeout: 500,
            }}
        >
            <Fade in={isModalVisible}>
                <div className={classes.paper}>
                    <h3 id="transition-modal-title">{title}</h3>
                    {functionType === "Bid" &&
                        <>
                            <div className={classes.inputDiv}>
                                <input type="number" className={classes.input} onChange={(e) => setPrice(e.target.value)} value={price} />
                                <p>ETH</p>
                            </div>
                            <Container component="div" maxWidth="lg" className={classes.buttonContainer}>
                                <Button variant="contained" color="primary" disableElevation onClick={() => bidHandler()}>BID</Button>
                            </Container>
                            <p id="transition-modal-description">react-transition-group animates me.</p>
                        </>
                    }
                    {functionType === "MyPage" &&
                        <Grid container spacing={3} alignItems="center" style={{ height: "100%", textAlign: "center" }}>
                            <Grid item xs={6}>
                                <Button variant="outlined" color="primary" onClick={handleAuction}>
                                    OK
                                </Button>
                            </Grid>
                            <Grid item xs={6}>
                                <Button variant="outlined" color="secondary" onClick={handleClose}>
                                    CANCEL
                                </Button>
                            </Grid>
                        </Grid>
                    }
                </div>
            </Fade>
        </Modal>
    );
}

export default BidModal;
