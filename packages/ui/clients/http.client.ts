import axios from "axios";

export const httpClient = axios.create({
  baseURL:
    process.env.NODE_ENV === `development`
      ? `http://localhost:4000/api`
      : `/api`,
  withCredentials: true,
  headers: {
    "Access-Control-Allow-Credentials": true,
  },
});
