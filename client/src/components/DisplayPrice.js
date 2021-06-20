import React from 'react';
import { useDispatch, useSelector } from "react-redux";
import { makeStyles } from '@material-ui/core';
import { Container } from '@material-ui/core';
import { Typography } from '@material-ui/core';
import { Card } from '@material-ui/core';
import { CardContent } from '@material-ui/core';
import { Button } from '@material-ui/core';

const useStyles = makeStyles({
    cardRoot: {
        color: 'white',
        background: 'black',
        marginBottom: '10px',

    },
    title: {
        fontSize: 14,
    },
    buttonContainer: {
        textAlign: 'center',

    },
    button: {
        width: '50%',
        padding: '10px',
    }
})

const DisplayPrice = () => {
    const classes = useStyles();
    const dispatch = useDispatch();
    const { highestBid } = useSelector((state) => state.loadBlockchain);
    
    console.log(highestBid);
    const bidHandler = () => {

    }
    // console.log(contractInstance);
    return (
        <div style={{ width: '30%', padding: '5px' }}>
            <Container maxWidth="lg" component="div">
                <Card className={classes.cardRoot}>
                    <CardContent>
                        <Typography variant="h3" component="p" className={classes.title}>Total: </Typography>
                        <Typography variant="h3" component="p" className={classes.title}>Rest time: </Typography>
                    </CardContent>
                </Card>
            </Container>
            <Container maxWidth="lg" component="div">
                <Card className={classes.cardRoot}>
                    <CardContent>
                        <Typography variant="h3" component="p" className={classes.title}>Price: {highestBid} ETH</Typography>
                        <Typography variant="h3" component="p" className={classes.title}>$580 </Typography>
                    </CardContent>
                </Card>
            </Container>
            <Container maxWidth="lg" component="div" className={classes.buttonContainer}>
                <Button variant="contained" size="large" color="primary" className={classes.button} onClick={bidHandler}>BID</Button>
            </Container>
        </div>
    );
}

export default DisplayPrice;
