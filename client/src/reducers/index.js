import { combineReducers } from 'redux';

import loadBlockchainReducer from './loadBlockchainReducer';
import artTokenReducer from './artTokenReducer';
import modalReducer from './modalReducer';
import loaderProducer from './loaderProducer';


const rootReducer = combineReducers({
    loadBlockchain: loadBlockchainReducer,
    artToken: artTokenReducer,
    progress: loaderProducer,
    modal: modalReducer,
});

export default rootReducer;
