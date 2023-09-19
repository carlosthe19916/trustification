import { SEC } from "../types/constants";
import { click, inputText } from "./dom";
import * as loginView from "../views/login.view";

let userName = Cypress.env("username");
let userPassword = Cypress.env("password");
const spogUiUrl = Cypress.env("spogUiUrl");

export function login(username?: string, password?: string): void {
  cy.visit(spogUiUrl, { timeout: 120 * SEC });
  cy.wait(5000);
  cy.get("h1", { timeout: 120 * SEC }).then(($b) => {
    if ($b.text().toString().trim() == "Sign in to your account") {
      if (username && password) {
        userName = username;
        userPassword = password;
      }
      inputText(loginView.userNameInput, userName);
      inputText(loginView.userPasswordInput, userPassword);
      click(loginView.loginButton);
    }
  });

  cy.contains("Trusted Content");
  cy.contains("A service for software supply chain security");
}
