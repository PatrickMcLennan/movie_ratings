import React, { FC } from "react";
import { BrowserRouter, Routes, Route } from "react-router-dom";
import * as Pages from "../../pages";
import { MuiTheme } from "..";

type Props = {};

export const App: FC<Props> = () => (
  <MuiTheme>
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<Pages.Home />} />
        <Route path="/login" element={<Pages.Login />} />
      </Routes>
    </BrowserRouter>
  </MuiTheme>
);
