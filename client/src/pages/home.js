import React from 'react';
import { Link } from 'react-router-dom'
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
        top: '30%',
        left: '50%',
        transform: 'translate(-50%, -50%)',
        color: 'black',
        fontSize: '6rem',
    },
    buttonContainer: {
        position: 'absolute',
        top: '55%',
        left: '50%',
        transform: 'translate(-50%, -50%)',
        color: 'black',
        fontSize: '6rem',
        display: 'flex',
        justifyContent: 'space-around',
    },
    button: {
        padding: '2rem 5rem',
    }
    
})

const Home = () => {
    const classes = useStyles();
    return (
        <div className={classes.root}>
            <img src={HomeImage} alt="painter" style={{ height: '60.4rem', width: '100%' }} />
            <Container>
                <Container className={classes.textContainer}>NFT Market</Container>
                <Container className={classes.buttonContainer}>
                    <Link to="/auction">
                        <Button variant="contained" className={classes.button}>Explore</Button>
                    </Link>
                    <Link>
                        <Button variant="contained" className={classes.button}>Support</Button>
                    </Link>

                </Container>
            </Container>
        </div>
    );
}

export default Home;

