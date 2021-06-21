import { combineReducers } from 'redux';

import loadBlockchainReducer from './loadBlockchainReducer';
import modalReducer from './modalReducer';

const rootReducer = combineReducers({
    loadBlockchain: loadBlockchainReducer,
    modal: modalReducer,
});

export default rootReducer;
