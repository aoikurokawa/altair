import React from 'react';
import { useSelector } from 'react-redux';
import { makeStyles, Card, CardHeader, Avatar, CardMedia, CardActionArea } from '@material-ui/core';
import { red } from '@material-ui/core/colors';

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
    carouselItem: {
        backgroundColor: '#3F50B5',
        padding: '88px 140px',
    },
    media: {
        height: 600,
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
}));

const CarouselImage = () => {
    const { nftDetail } = useSelector((state) => state.artToken);
    const classes = useStyles();

    return (
        <div style={{ width: '60%', padding: '5px' }}>
            <div>
                <Card>
                    <CardActionArea>
                        <CardHeader
                            avatar={
                                <Avatar aria-label="recipe" className={classes.avatar}>
                                    {nftDetail.attributes["Account"]}
                                </Avatar>
                            }
                            title={nftDetail.attributes["Name"]}
                        />
                        <CardMedia
                            className={classes.media}
                            image={nftDetail.attributes["IpfsUrl"]}
                            title="Paella dish"
                        />
                    </CardActionArea>
                </Card>
            </div>
        </div>

    );
}

export default CarouselImage;
