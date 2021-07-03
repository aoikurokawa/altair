import React, { useEffect } from 'react';
import { useDispatch, useSelector } from 'react-redux';

import Carousel from '../components/Carouselmage';
import DisplayPrice from '../components/DisplayPrice';

const AuctionDetail = () => {
    const { nftDetail } = useSelector((state) => state.artToken);

    // useEffect(() => {
    //     console.log(nftDetail);
    // }, []);

    return (
        <div style={{ height: '60.4rem' }}>
            <div style={{ display: 'flex' }}>
                <Carousel />
                <DisplayPrice />
            </div>
        </div>
    );
}

export default AuctionDetail;
