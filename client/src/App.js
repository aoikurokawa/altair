import React, { useEffect } from 'react';
import { Switch, Route, useLocation } from "react-router-dom";
import { useDispatch, useSelector } from 'react-redux';
import { useMoralis } from 'react-moralis';

import Nav from './components/Nav';
import Home from './pages/Home';
import Auction from './pages/Auction';
import UploadPage from './pages/UploadPage';
import SignUp from './components/SignUp';
import SignIn from './components/SignIn';
import MyCollection from './pages/MyCollection';
import MyPage from './pages/MyPage';
import SpeedDials from './components/SpeedDials';
import Loader from './components/Loader';
import GlobalStyle from './components/GlobalStyle';
import BidModal from './components/BidModal';
import { loadBlockchain } from './actions/loadBlockchainAction';
import { getArtTokenContract } from './actions/artTokenAction';

function App() {
  const location = useLocation();
  const dispatch = useDispatch();
  const { isModalVisible } = useSelector((state) => state.modal);
  const { isLoading } = useSelector((state) => state.progress);
  const { isAuthenticated } = useMoralis();

  useEffect(() => {
    dispatch(loadBlockchain());
    dispatch(getArtTokenContract());
  }, []);

  return (
    <div>
      <GlobalStyle />
      <Nav />
      <Switch location={location} key={location.pathname}>
        <Route path="/" exact>
          <Home />
        </Route>
        <Route path="/signup" exact>
          <SignUp />
        </Route>
        <Route path="/signin" exact>
          <SignIn />
        </Route>
        <Route path="/auction" exact>
          <Auction />
        </Route>
        <Route path="/upload" exact>
          <UploadPage />
        </Route>
        <Route path="/mypage" exact>
          <MyPage />
        </Route>
        <Route path="/mycollection" exact>
          <MyCollection />
        </Route>
      </Switch>
      <SpeedDials />
      <Loader isLoading={isLoading} />
      <BidModal isModalVisible={isModalVisible} />
    </div>
  );
}

export default App;
