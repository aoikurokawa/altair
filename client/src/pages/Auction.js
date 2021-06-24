import React, { useEffect } from 'react';
import { Typography } from '@material-ui/core';

import AuctionDetail from '../components/AuctionDetail';
import Carousel from '../components/Carouselmage';
import DisplayPrice from '../components/DisplayPrice';

const Auction = () => {
    return (
        <div style={{height: '60.4rem'}}>
            <Typography variant="h4" gutterBottom style={{ padding: '5px' }}>
                Attack on Titan
            </Typography>
            <div style={{ display: 'flex' }}>
                <Carousel />
                <DisplayPrice />
            </div>
            <AuctionDetail />
        </div>
    );
}

export default Auction;
