/// <reference types="cypress" />
import { login } from "../utils/authentication";

describe("Log In", () => {
  it("Login to Spog", () => {
    login();
  });
});
