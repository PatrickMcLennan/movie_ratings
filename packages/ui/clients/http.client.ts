import axios from "axios";

export const httpClient = axios.create({
  baseURL:
    process.env.NODE_ENV === `development`
      ? `http://movie_ratings_rust:4000/`
      : `/`,
});
