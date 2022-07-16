import { ApolloClient, InMemoryCache, HttpLink, from } from "@apollo/client";
import { onError } from "@apollo/client/link/error";

const authLink = onError(({ networkError }) => {
  /**
   * Why is it so difficult to see the http status code of the response with apollo?!
   * Hack put in place because there's no other place to get it other than this
   * object, which for some reason apollo doesn't think is on this object, but it is
   */
  //@ts-ignore
  const { statusCode } = networkError;
  if (statusCode === 401) {
    window.location.assign(`/login`);
  }
});

const httpLink = new HttpLink({
  uri:
    process.env.NODE_ENV === `development`
      ? `http://localhost:4000/api/graphql`
      : `/api/graphql`,
  credentials: `include`,
});

export const graphQlClient = new ApolloClient({
  cache: new InMemoryCache(),
  credentials: `include`,
  link: from([authLink, httpLink]),
});
