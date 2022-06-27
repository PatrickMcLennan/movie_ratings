import {
  FormControl,
  FormHelperText,
  Input,
  InputLabel,
  SxProps,
} from "@mui/material";
import React, { FC } from "react";
import {
  Control,
  Controller,
  FieldError,
  FieldErrors,
  Path,
} from "react-hook-form";

type Props<T> = {
  control: Control<T>;
  errors: FieldErrors<T>;
  name: Path<T>;
  type: string;
};

const sx = {
  formHelperText: {
    "&, & span": {
      height: `2rem`,
    },
  },
} as const;

export function TextInput<T>({ control, errors, name, type }: Props<T>) {
  return (
    <Controller
      control={control}
      name={name}
      render={({ field }) => {
        const error = errors?.[name as string];
        return (
          <FormControl error={!!error}>
            <InputLabel htmlFor={name}>Email</InputLabel>
            <Input {...field} id={name} type={type} />
            <FormHelperText sx={sx.formHelperText} aria-hidden={!error}>
              {error ? error.message : ` `}
            </FormHelperText>
          </FormControl>
        );
      }}
    />
  );
}
