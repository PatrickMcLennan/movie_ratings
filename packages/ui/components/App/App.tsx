import React, { FC } from "react";
import { BrowserRouter, Routes, Route } from "react-router-dom";
import * as Pages from "../../pages";
type Props = {};

export const App: FC<Props> = () => {
  return (
    <BrowserRouter>
      <main>
        <Routes>
          <Route path="/" element={<Pages.Home />} />
          <Route path="/login" element={<Pages.Home />} />
        </Routes>
      </main>
    </BrowserRouter>
  );
};
