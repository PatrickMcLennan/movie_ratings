import React, { FC } from "react";
import { Container } from "@mui/material";
import { gql, useQuery } from "@apollo/client";

type Props = {};

export const Home: FC<Props> = () => {
  const { data, loading, error } = useQuery(gql`
    query getAllUsers {
      getAllUsers {
        email
        firstName
        lastName
        id
        createdAt
        updatedAt
      }
    }
  `);

  console.log(`loading`);
  console.log(loading);
  console.log(`data`);
  console.log(data);
  console.log(`error`);
  console.log(error);

  return (
    <Container component="main" maxWidth="lg">
      <h1>this is the home page</h1>
    </Container>
  );
};
