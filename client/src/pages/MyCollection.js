import React, { useEffect } from 'react';
import { useMoralisQuery } from 'react-moralis';


const MyCollection = () => {
    const { data, } = useMoralisQuery("Nft");

    useEffect(() => {
        data.map((d) => {
            console.log(d.attributes["TokenId"]);
        });
    }, [data])
    return (
        <div>

        </div>
    );

}

export default MyCollection;
