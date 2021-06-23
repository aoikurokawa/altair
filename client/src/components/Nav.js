import React from 'react';
import { makeStyles } from "@material-ui/core/styles"
import { AppBar } from '@material-ui/core';
import { Toolbar } from '@material-ui/core';
import { Typography } from '@material-ui/core';
import { Button } from '@material-ui/core';

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
    
    const metamaskHandler = async () => {
        try {
            await window.ethereum.request({ method: "eth_requestAccounts" });
        } catch (error) {
            console.error(error);
        }
    };
    return (
        <div className={classes.root}>
            <AppBar position="static">
                <Toolbar>
                    
                    <Typography variant="h5" className={classes.title}>
                        Art Market
                    </Typography>
                    {
                        accounts === "" ?
                            <Button variant="outlined" color="inherit" onClick={metamaskHandler}>
                                Connect with MetaMask
                            </Button>
                            :
                            <Typography variant="h6" component="h6">Accounts: {accounts}</Typography>
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
