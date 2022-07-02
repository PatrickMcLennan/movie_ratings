import React, { FC } from "react";
import { Box, Button } from "@mui/material";
import { useForm } from "react-hook-form";
import { TextInput } from "..";
import { yupResolver } from "@hookform/resolvers/yup";
import * as yup from "yup";

type Props = {};

export const errorMessages = {
  noEmail: `An email is required`,
  invalidEmail: `This is an invalid email`,
  noPassword: `A password is required`,
  invalidPassword: `A valid password must be at least 8 characters - 1 lowercase + 1 uppercase letter, 1 special character and 1 number`,
};

const sx = {
  form: {
    display: `grid`,
    width: "100%",
  },
  submit: {
    fontSize: `1.6rem`,
    marginTop: `1.5rem`,
    padding: `1.6rem`,
    // backgroundColor: (theme: Theme) => theme.palette.secondary.main,
  },
} as const;

const resolver = yup.object().shape({
  email: yup
    .string()
    .trim()
    .matches(
      /^(([^<>()\[\]\\.,;:\s@"]+(\.[^<>()\[\]\\.,;:\s@"]+)*)|(".+"))@((\[[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}])|(([a-zA-Z\-0-9]+\.)+[a-zA-Z]{2,}))$/,
      errorMessages.invalidEmail
    )
    .required(errorMessages.noEmail),
  password: yup
    .string()
    .trim()
    .matches(
      /^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)(?=.*[@$!%*?&])[A-Za-z\d@$!%*?&]{8,}$/
    )
    .min(8, errorMessages.invalidPassword)
    .required(errorMessages.noPassword),
});

export const LoginForm: FC<Props> = () => {
  const {
    formState: { errors },
    handleSubmit,
    control,
  } = useForm<{
    email: string;
    password: string;
  }>({
    resolver: yupResolver(resolver),
  });

  return (
    <Box
      component="form"
      noValidate
      onSubmit={handleSubmit(console.log)}
      sx={sx.form}
    >
      <TextInput
        control={control}
        errors={errors}
        label="Email"
        name="email"
        type="email"
      />
      <TextInput
        control={control}
        errors={errors}
        label="Password"
        name="password"
        type="password"
      />
      <Button sx={sx.submit} type="submit" variant="contained">
        Log In
      </Button>
    </Box>
  );
};
