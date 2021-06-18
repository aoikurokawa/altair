import React from 'react';
import styled from 'styled-components';
import { Button } from '@material-ui/core';

const Nav = () => {
    const metamaskHandler = async () => {
        try {
            await window.ethereum.request({ method: "eth_requestAccounts" });
        } catch (error) {
            console.error(error);
        }
    };
    return (
        <NavStyled>
            <h1>Art Market</h1>
            <EthStyled>
                <Button variant="outlined" color="primary" onClick={metamaskHandler}>
                    Connect with MetaMask
                </Button>
            </EthStyled>
        </NavStyled>
    )
}

const NavStyled = styled.nav`

`;

const EthStyled = styled.div`

`;

export default Nav;
