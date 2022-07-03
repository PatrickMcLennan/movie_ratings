import React, { FC } from "react";
import { BrowserRouter, Routes, Route } from "react-router-dom";
import * as Pages from "../../pages";
import { MuiTheme } from "..";
import { ApolloProvider } from "@apollo/client";
import { graphQlClient } from "../../clients";

type Props = {};

export const App: FC<Props> = () => (
  <ApolloProvider client={graphQlClient}>
    <MuiTheme>
      <BrowserRouter>
        <Routes>
          <Route path="/" element={<Pages.Home />} />
          <Route path="/login" element={<Pages.Login />} />
        </Routes>
      </BrowserRouter>
    </MuiTheme>
  </ApolloProvider>
);
