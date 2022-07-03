import { ApolloClient, InMemoryCache } from "@apollo/client";

export const graphQlClient = new ApolloClient({
  uri:
    process.env.NODE_ENV === `development`
      ? `http://localhost:4000/api/graphql`
      : `/api/graphql`,
  credentials: `include`,
  cache: new InMemoryCache(),
});
