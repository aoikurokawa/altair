import React from 'react';
import HomeImage from '../assets/home.jpg';
import { Container } from '@material-ui/core';
import { makeStyles } from '@material-ui/core';
import { Button } from '@material-ui/core';

const useStyles = makeStyles({
    root: {
        position: 'relative',
        textAlign: 'center',
        color: 'white',
    },
    textContainer: {
        position: 'absolute',
        top: '50%',
        left: '50%',
        transform: 'translate(-50%, -50%)',
        color: 'black',
        fontSize: '6rem',
    }, 
    buttonContainer: {
        position: 'absolute',
        top: '65%',
        left: '50%',
        transform: 'translate(-50%, -50%)',
        color: 'black',
        fontSize: '6rem',
    },
    button: {
        width: '30%',
        padding: '1rem',
    }
})

const Home = () => {
    const classes = useStyles();
    return (
        <div className={classes.root}>
            <img src={HomeImage} alt="painter" style={{ height: '61rem', width: '100%' }} />
            <Container className={classes.textContainer}>Auction DApp</Container>
            <Container className={classes.buttonContainer}>
                <Button variant="contained" className={classes.button}>Auction</Button>
            </Container>
        </div>
    );
}

export default Home;

