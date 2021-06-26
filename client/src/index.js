import React from 'react';
import ReactDOM from 'react-dom';
import App from './App';
import reportWebVitals from './reportWebVitals';
import { createStore, applyMiddleware, compose } from 'redux';
import { Provider } from 'react-redux';
import { BrowserRouter } from "react-router-dom";
import thunk from 'redux-thunk';
import {MoralisProvider} from 'react-moralis';

import rootReducer from './reducers';

const composeEnhancers =
  (typeof window !== "undefined" &&
    window.__REDUX_DEVTOOLS_EXTENSION_COMPOSE__) ||
  compose;

const store = createStore(
  rootReducer,
  composeEnhancers(applyMiddleware(thunk))
);

const appId = "SPDlAa3F1I1dg2PaZYK97hjV6KHpErz6nN0t1jbr";
const serverUrl = "https://okrter5l6x6l.moralis.io:2053/server";

ReactDOM.render(
  <MoralisProvider appId={appId} serverUrl={serverUrl}>
    <Provider store={store}>
      <BrowserRouter>
        <App />
      </BrowserRouter>
    </Provider>
  </MoralisProvider>,
  document.getElementById('root')
);

// If you want to start measuring performance in your app, pass a function
// to log results (for example: reportWebVitals(console.log))
// or send to an analytics endpoint. Learn more: https://bit.ly/CRA-vitals
reportWebVitals();
