import React, { useEffect } from 'react';
import { useSelector } from 'react-redux';
import { useMoralisQuery } from 'react-moralis';


const MyCollection = () => {
    const { accounts } = useSelector((state) => state.artToken);
    const { data, error, isLoading } = useMoralisQuery("Nft");

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
