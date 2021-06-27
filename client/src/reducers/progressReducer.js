const initialState = {
    isLoading: false,
}

const progressProducer = (state = initialState, action) => {
    switch(action.type) {
        case "SHOW_PROGRESS": 
            return {
                isLoading: true,
            };

        case "CLOSE_PROGRESS":
            return {
                isLoading: false, 
            };

        default: 
            return {
                ...state,
            }
    }
}

export default progressProducer;
