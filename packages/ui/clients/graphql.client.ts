import { ApolloClient, InMemoryCache } from "@apollo/client";

export const graphQlClient = new ApolloClient({
  uri:
    process.env.NODE_ENV === `development`
      ? `http://movie_ratings_rust:4000/api/graphql`
      : `/api/graphql`,
  cache: new InMemoryCache(),
});
