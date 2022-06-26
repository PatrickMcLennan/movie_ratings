import React, { FC } from "react";
import {
  Box,
  Button,
  FormControl,
  FormHelperText,
  InputLabel,
  Input,
  Theme,
} from "@mui/material";
import { Controller, useForm } from "react-hook-form";
import { yupResolver } from "@hookform/resolvers/yup";
import * as yup from "yup";

type Props = {};

export const errorMessages = {
  noEmail: `An email is required`,
  invalidEmail: `This is an invalid email`,
  noPassword: `A password is required`,
  invalidPassword: `A valid password must be at least 6 characters`,
};

const sx = {
  form: {
    display: `grid`,
    width: "100%",
  },
  formHelperText: {
    "&, & span": {
      height: `2rem`,
    },
  },
  submit: {
    fontSize: `1.6rem`,
    marginTop: `1.5rem`,
    padding: `1.6rem`,
    // backgroundColor: (theme: Theme) => theme.palette.secondary.main,
  },
} as const;

export const LoginForm: FC<Props> = () => {
  const {
    formState: { errors },
    handleSubmit,
    control,
  } = useForm<{
    email: string;
    password: string;
  }>({
    resolver: yupResolver(
      yup.object().shape({
        email: yup
          .string()
          .email(errorMessages.invalidEmail)
          .required(errorMessages.noEmail),
        password: yup
          .string()
          .min(8, errorMessages.invalidPassword)
          .required(errorMessages.noPassword),
      })
    ),
  });

  return (
    <Box component="form" onSubmit={handleSubmit(console.log)} sx={sx.form}>
      <Controller
        control={control}
        name="email"
        render={({ field }) => {
          const emailError = errors?.email;
          return (
            <FormControl error={!!emailError}>
              <InputLabel htmlFor="email">Email</InputLabel>
              <Input {...field} id="email" type="email" />
              <FormHelperText sx={sx.formHelperText} aria-hidden={!emailError}>
                {emailError ? emailError.message : ` `}
              </FormHelperText>
            </FormControl>
          );
        }}
      />
      <Controller
        control={control}
        name="password"
        render={({ field }) => {
          const passwordError = errors.password;
          return (
            <FormControl error={!!passwordError}>
              <InputLabel htmlFor="password">Password</InputLabel>
              <Input {...field} id="password" type="password" />
              <FormHelperText
                sx={sx.formHelperText}
                aria-hidden={!passwordError}
              >
                {passwordError ? passwordError.message : ` `}
              </FormHelperText>
            </FormControl>
          );
        }}
      />
      <Button sx={sx.submit} type="submit" variant="contained">
        Log In
      </Button>
    </Box>
  );
};
