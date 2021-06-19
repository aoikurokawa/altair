import Web3 from 'web3';
import { useDispatch, useSelector } from 'react-redux';
import styled from 'styled-components';

import Nav from './components/Nav';
import MediaCard from './components/MediaCard';
import AuctionDetail from './components/AuctionDetail';
import GlobalStyle from './components/GlobalStyle';
import { loadBlockchain } from './actions/loadBlockchainAction';
import { useEffect } from 'react';

function App() {

  const dispatch = useDispatch();
  const { contractInstance, accounts } = useSelector((state) => state.loadBlockchain);

  useEffect(() => {
    dispatch(loadBlockchain());
  }, [accounts]);

  return (
    <div>
      <GlobalStyle />
      <Nav accounts={accounts} />
      <div style={{ display: 'flex', flexWrap: 'nowrap' }}>
        <MediaCard />
        <MediaCard />
        <MediaCard />
      </div>

      <AuctionDetail />
    </div>
  );
}

export default App;
