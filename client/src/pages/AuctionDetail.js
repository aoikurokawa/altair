import React from 'react';

import Carousel from '../components/Carouselmage';
import DisplayPrice from '../components/DisplayPrice';


const AuctionDetail = () => {

    return (
        <div style={{ height: '60.4rem', padding: '1rem 5rem' }}>
            <div style={{ display: 'flex' }}>
                <Carousel />
                <DisplayPrice />
            </div>
        </div>
    );
}

export default AuctionDetail;
