import { combineReducers } from 'redux';

import loadBlockchainReducer from './loadBlockchainReducer';

const rootReducer = combineReducers({
    loadBlockchain: loadBlockchainReducer,
});

export default rootReducer;
