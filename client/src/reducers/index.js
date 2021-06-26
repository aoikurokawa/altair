import { combineReducers } from 'redux';

import loadBlockchainReducer from './loadBlockchainReducer';
import artTokenReducer from './artTokenReducer';
import modalReducer from './modalReducer';


const rootReducer = combineReducers({
    loadBlockchain: loadBlockchainReducer,
    artToken: artTokenReducer,
    modal: modalReducer,
});

export default rootReducer;
