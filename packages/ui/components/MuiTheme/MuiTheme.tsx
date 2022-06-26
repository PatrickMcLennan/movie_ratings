import React, { FC, ReactNode } from "react";

import {
  createTheme,
  CssBaseline,
  responsiveFontSizes,
  ThemeProvider,
} from "@mui/material";

type Props = {
  children: ReactNode | ReactNode[];
};

const darkTheme = responsiveFontSizes(
  createTheme({
    palette: {
      mode: "dark",
    },
  })
);

export const MuiTheme: FC<Props> = ({ children }) => (
  <ThemeProvider theme={darkTheme}>
    <CssBaseline />
    {children}
  </ThemeProvider>
);
