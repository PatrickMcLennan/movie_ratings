import React, { FC } from "react";
import { Container } from "@mui/material";
import { LoginForm } from "../../components";

type Props = {};

const sx = {
  container: {
    display: "grid",
    width: "100%",
    placeItems: "center",
  },
} as const;

export const Login: FC<Props> = () => {
  return (
    <Container component="main" maxWidth="lg" sx={sx.container}>
      <LoginForm />
    </Container>
  );
};
