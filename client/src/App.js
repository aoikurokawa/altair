import Web3 from 'web3';
import { useDispatch, useSelector } from 'react-redux';
import styled from 'styled-components';
import { Typography } from '@material-ui/core';

import Nav from './components/Nav';
import MediaCard from './components/MediaCard';
import AuctionDetail from './components/AuctionDetail';
import Carousel from './components/Carouselmage';
import DisplayPrice from './components/DisplayPrice';
import GlobalStyle from './components/GlobalStyle';
import BidModal from './components/BidModal';
import { loadBlockchain } from './actions/loadBlockchainAction';
import { useEffect } from 'react';

function App() {

  const dispatch = useDispatch();
  const { isModalVisible } = useSelector((state) => state.modal);
  const { contractInstance, accounts } = useSelector((state) => state.loadBlockchain);

  useEffect(() => {
    dispatch(loadBlockchain());
  }, []);

  return (
    <div>
      <GlobalStyle />
      <Nav accounts={accounts} />
      <Typography variant="h4" gutterBottom style={{ padding: '5px' }}>
        Attack on Titan
      </Typography>
      <div style={{ display: 'flex' }}>
        <Carousel />
        <DisplayPrice />
      </div>
      <AuctionDetail />
      <BidModal isModalVisible={isModalVisible} />
    </div>
  );
}

export default App;
