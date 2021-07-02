import React, { useEffect } from 'react';
import { Typography, Card, CardHeader, Avatar, CardMedia, CardContent, Link, IconButton ,makeStyles } from '@material-ui/core';
import { red } from '@material-ui/core/colors';
import GavelIcon from '@material-ui/icons/Gavel';
import { useMoralisQuery } from 'react-moralis';

import AuctionDetail from '../components/AuctionDetail';
import Carousel from '../components/Carouselmage';
import DisplayPrice from '../components/DisplayPrice';

const useStyles = makeStyles((theme) => ({
    root: {
        height: '60.4rem',
        paddingTop: '1rem',
    },
    loader: {
        textAlign: 'center',
        marginTop: '50%',
    },
    container: {
        marginTop: "5rem",
    },
    cardContainer: {
        display: 'flex',
        padding: '2rem 6rem',
        overflow: 'hidden',
    },
    cardRoot: {
        width: '75%',
    },
    carouselItem: {
        backgroundColor: '#3F50B5',
        padding: '88px 140px',
    },
    media: {
        height: 0,
        paddingTop: '56.25%', // 16:9
    },
    expand: {
        transform: 'rotate(0deg)',
        marginLeft: 'auto',
        transition: theme.transitions.create('transform', {
            duration: theme.transitions.duration.shortest,
        }),
    },
    expandOpen: {
        transform: 'rotate(180deg)',
    },
    avatar: {
        backgroundColor: red[500],
    },
}))

const Auction = () => {
    const classes = useStyles();

    const { data, error, isFetching } = useMoralisQuery(
        "Nft",
        query =>
            query.equalTo("IsSelled", true),
        [],

    );

    console.log(data);
    return (
        <div style={{ height: '60.4rem' }}>
            <Typography component="h1" variant="h3" gutterBottom style={{ padding: '5px' }}>
                Welcome to Auction
            </Typography>
            {data.map((d) => {
                return (
                    <div key={d.attributes["TokenId"]}>
                        <Card className={classes.cardRoot} >
                            <CardHeader
                                avatar={
                                    <Avatar aria-label="recipe" className={classes.avatar}>
                                        {d.attributes["Account"]}
                                    </Avatar>
                                }
                                action={
                                    <IconButton aria-label="settings">
                                        <GavelIcon />
                                    </IconButton>
                                }
                                title={d.attributes["Name"]}
                            />
                            <CardMedia
                                className={classes.media}
                                image={d.attributes["IpfsUrl"]}
                                title="Paella dish"
                            />
                            <CardContent>
                                <Typography variant="body2" color="textSecondary" component="p">
                                    IPFS Hash: <br />{d.attributes["IpfsHash"]}
                                </Typography>
                                <Typography variant="body2" color="textSecondary" component="p">
                                    IPFS URL: <br />
                                    <Link href={d.attributes["IpfsUrl"]} target="_blank">
                                        {d.attributes["IpfsUrl"]}
                                    </Link>
                                </Typography>
                                <Typography variant="body2" color="textSecondary" component="p">
                                    Token ID: {d.attributes["TokenId"]}
                                </Typography>
                                {
                                    d.attributes["IsSelled"] ?
                                        <Typography variant="body2" color="textSecondary" component="p">
                                            Sale
                                        </Typography> :
                                        <Typography variant="body2" color="textSecondary" component="p">
                                            Not Sale
                                        </Typography>
                                }
                            </CardContent>
                        </Card>
                    </div>
                )
            })}
            <div style={{ display: 'flex' }}>
                <Carousel />
                <DisplayPrice />
            </div>
            <AuctionDetail />
        </div>
    );
}

export default Auction;
