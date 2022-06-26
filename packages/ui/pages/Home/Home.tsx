import React, { FC } from "react";
import { Container } from "@mui/material";

type Props = {};

export const Home: FC<Props> = () => {
  return (
    <Container component="main" maxWidth="lg">
      <h1>this is the home page</h1>
    </Container>
  );
};
