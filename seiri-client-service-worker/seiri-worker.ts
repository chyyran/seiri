import ApolloClient from 'apollo-boost'
import gql from 'graphql-tag'

const client = new ApolloClient({
  uri: 'http://localhost:9234/graphql'
});


const seedCache = async () => {
    let result = await client.query({
        query: gql`
            query TracksQuery {
                tracks(query:"") {
                    title,
                    trackNumber,
                    artist,
                    albumArtists,
                    album
                }
            }
        `
    })

    console.log(result)
}

console.log("Hello World")
seedCache()