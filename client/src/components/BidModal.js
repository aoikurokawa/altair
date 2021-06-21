import React, { useState } from 'react';
import Web3 from 'web3';
import { useDispatch, useSelector } from 'react-redux';
import { makeStyles } from '@material-ui/core';
import { Modal } from '@material-ui/core';
import { Backdrop } from '@material-ui/core';
import { Fade } from '@material-ui/core';
import { Button } from '@material-ui/core';
import { Container } from '@material-ui/core';

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
    const { contractInstance, accounts } = useSelector((state) => state.loadBlockchain);
    const { isModalVisible, } = useSelector((state) => state.modal);
    const dispatch = useDispatch();

    const handleClose = () => {
        dispatch({
            type: "CLOSE_MODAL",
        });
    };

    const bidHandler = () => {
        if (price <= 0) {
            alert("Please put more than 0");
        }

        let config = {
            value: Web3.utils.toWei(price, "ether"),
            from: accounts[0],
        };
        contractInstance.methods
            .bid()
            .send(config)
            .on("transactionHash", (hash) => {
                console.log(hash);
            })
            .on("receipt", (receipt) => {
                console.log(receipt);
                alert("Congratulations!");
            });
        dispatch({
            type: 'CLOSE_MODAL'
        })
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
                    <h2 id="transition-modal-title">Input Price</h2>
                    <div className={classes.inputDiv}>
                        <input type="number" className={classes.input} onChange={(e) => setPrice(e.target.value)} value={price} />
                        <p>ETH</p>
                    </div>
                    <Container component="div" maxWidth="lg" className={classes.buttonContainer}>
                        <Button variant="contained" color="primary" disableElevation onClick={bidHandler}>BID</Button>
                    </Container>
                    <p id="transition-modal-description">react-transition-group animates me.</p>
                </div>
            </Fade>
        </Modal>
    );
}

export default BidModal;