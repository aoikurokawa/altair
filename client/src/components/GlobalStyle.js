import { createGlobalStyle } from 'styled-components';

const GlobalStyle = createGlobalStyle`
    *{
        margin: 0;
        padding: 0;
        box-sizing: border-box;
    }
    html{
        height: 100%;
        @media (max-width: 1700px) {
            font-size: 75%;
        }
    }
`;

export default GlobalStyle;
