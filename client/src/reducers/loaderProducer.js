const initialState = {
    isLoading: false,
}

const loaderProducer = (state = initialState, action) => {
    switch(action.type) {
        case "SHOW_LOADER": 
            return {
                isLoading: true,
            };

        case "CLOSE_LOADER":
            return {
                isLoading: false, 
            };

        default: 
            return {
                ...state,
            }
    }
}

export default loaderProducer;
