import Web3 from 'web3';
import { useDispatch, useSelector } from 'react-redux';
import styled from 'styled-components';

import Nav from './components/Nav';
import { loadBlockchain } from './actions/loadBlockchainAction';
import { useEffect } from 'react';

function App() {

  const dispatch = useDispatch();
  const { contractInstance, accounts } = useSelector((state) => state.loadBlockchain);

  useEffect(() => {
    dispatch(loadBlockchain());
  }, []);

  return (
    <div>

      <Nav />
    </div>
  );
}

export default App;
