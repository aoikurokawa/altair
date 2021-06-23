import React, {useEffect} from 'react';
import { Switch, Route, useLocation } from "react-router-dom";
import { useDispatch, useSelector } from 'react-redux';

import Nav from './components/Nav';
import Home from './pages/Home';
import Auction from './pages/Auction';
import SpeedDials from './components/SpeedDials';
import AuctionDetail from './components/AuctionDetail';
import Carousel from './components/Carouselmage';
import DisplayPrice from './components/DisplayPrice';
import GlobalStyle from './components/GlobalStyle';
import BidModal from './components/BidModal';
import { loadBlockchain } from './actions/loadBlockchainAction';

function App() {

  const location = useLocation();

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
      <Switch location={location} key={location.pathname}>
        <Route path="/" exact>
          <Home />
        </Route>
        <Route path="/auction" exact>
          <Auction />
        </Route>
        <Route path="/upload" exact>
          <Auction />
        </Route>
      </Switch>
      <SpeedDials />
      <BidModal isModalVisible={isModalVisible} />
    </div>
  );
}

export default App;
