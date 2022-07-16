import React, { FC } from "react";
import { Container, Typography } from "@mui/material";
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

  return (
    <Container component="main" maxWidth="lg">
      <Typography component="h1">this is the home page</Typography>
    </Container>
  );
};
