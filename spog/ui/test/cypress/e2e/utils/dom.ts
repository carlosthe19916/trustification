import { SEC } from "../types/constants";
import * as commonView from "../views/common.view";

export function inputText(fieldSelector: string, text: any): void {
  cy.get(fieldSelector).click().focused().clear();
  cy.wait(200);
  cy.get(fieldSelector).clear().type(text);
}

export function clickByText(
  fieldId: string,
  buttonText: string,
  isForced = true
): void {
  // https://github.com/cypress-io/cypress/issues/2000#issuecomment-561468114
  cy.contains(fieldId, buttonText, { timeout: 120 * SEC }).click({
    force: isForced,
  });
  cy.wait(SEC);
}

export function click(fieldId: string, isForced = true): void {
  cy.get(fieldId, { timeout: 120 * SEC }).click({ force: isForced });
}

export function applySearchFilterText(filterText: string): void {
  inputText(
    ".pf-v5-c-form .pf-v5-c-form-control input",
    `${filterText}{enter}`
  );
  cy.wait(4000);
}

export function applyCheckboxFilter(checkboxName: string): void {
  cy.get(".pf-v5-c-check")
    .contains(checkboxName)
    .parent("div.pf-v5-c-check")
    .find("input[type='checkbox']")
    .check();
  cy.wait(4000);
}

export function applyRadioButtonFilter(radioButtonName: string): void {
  cy.get(".pf-v5-c-radio")
    .contains(radioButtonName)
    .parent("div.pf-v5-c-radio")
    .find("input[type='radio']")
    .check();
  cy.wait(4000);
}

export function existsRow(value: string): void {
  // Wait for DOM to render table and sibling elements
  cy.get(commonView.mainPageTable, { timeout: 5 * SEC }).then(($div) => {
    if (!$div.hasClass("pf-v5-c-empty-state")) {
      cy.get("td", { timeout: 120 * SEC }).should("contain", value);
    }
  });
}
