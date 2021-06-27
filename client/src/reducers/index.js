import { combineReducers } from 'redux';

import loadBlockchainReducer from './loadBlockchainReducer';
import artTokenReducer from './artTokenReducer';
import modalReducer from './modalReducer';
import progressReducer from './modalReducer';


const rootReducer = combineReducers({
    loadBlockchain: loadBlockchainReducer,
    artToken: artTokenReducer,
    progress: progressReducer,
    modal: modalReducer,
});

export default rootReducer;
