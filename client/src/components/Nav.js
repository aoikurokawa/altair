import React from 'react';
import { Link } from 'react-router-dom';
import { makeStyles } from "@material-ui/core/styles"
import { AppBar } from '@material-ui/core';
import { Toolbar } from '@material-ui/core';
import { Typography } from '@material-ui/core';
import { Button } from '@material-ui/core';
import { Alert, AlertTitle } from '@material-ui/lab';
import { useMoralis } from 'react-moralis'


import styled from 'styled-components';

const useStyles = makeStyles((theme) => ({
    root: {
        flexGrow: 1,
        position: 'sticky',
        width: '100%',
        zIndex: '10',
        top: '0',
    },
    menuButton: {
        marginRight: theme.spacing(2),
    },
    title: {
        flexGrow: 1,
    }
}));


const Nav = ({ accounts }) => {
    const classes = useStyles();

    const { authenticate, isAuthenticated, authError, logout } = useMoralis();

    return (
        <div className={classes.root}>
            <AppBar position="static">
                <Toolbar>
                    <Typography variant="h5" className={classes.title}>
                        <Link to="/" style={{ color: 'white', textDecoration: 'none' }}>
                            Art Market
                        </Link>
                    </Typography>

                    {
                        isAuthenticated ?
                            <div>
                                <Typography variant="h6" component="h6">Accounts: {accounts}</Typography>
                                <Button variant="outlined" color="inherit" onClick={logout} onClose={() => { }}>
                                    Logout
                                </Button>
                            </div>
                            :
                            <div>
                                <Button variant="outlined" color="inherit" onClick={authenticate}>
                                    Login
                                </Button>
                                <Button variant="outlined" color="inherit">
                                    <Link to="/signup" style={{ color: 'white', textDecoration: 'none' }}>
                                        Sign Up
                                    </Link>
                                </Button>
                            </div>
                    }
                </Toolbar>
            </AppBar>
        </div>
    )
}

const NavStyled = styled.nav`
    
`;

const EthStyled = styled.div`

`;

export default Nav;
