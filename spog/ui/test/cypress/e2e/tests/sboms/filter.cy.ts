/// <reference types="cypress" />

import { SIDEBAR, button, clearAllFilters } from "../../types/constants";
import { login } from "../../utils/authentication";
import {
  applyCheckboxFilter,
  applyRadioButtonFilter,
  applySearchFilterText,
  clickByText,
  existsRow,
} from "../../utils/dom";
import { navMenu } from "../../views/sidebar.view";

let SBOMList = {
  ubi9: { name: "ubi9-container" },
  seedwingJavaExample: { name: "seedwing-java-example" },
};

let ProductsFilter = {
  container: "Container",
  createdOn: {
    last7Days: "Last 7 days",
    last30Days: "Last 30 days",
    thisYear: "This year",
    anyTime: "Any time",
  },
};

describe("SBOMs filter validations", () => {
  before("Login", () => {
    login();
  });

  afterEach("Clean", () => {
    cy.reload();
  });

  it("SearchInput validations", function () {
    // Navigate to page
    clickByText(navMenu, SIDEBAR.SBOMs);

    // Enter an existing display name substring and assert
    let searchInput = SBOMList.ubi9.name.substring(0, 3);
    applySearchFilterText(searchInput);
    existsRow(SBOMList.ubi9.name);

    searchInput = SBOMList.seedwingJavaExample.name.substring(0, 8);
    applySearchFilterText(searchInput);
    existsRow(SBOMList.seedwingJavaExample.name);

    // Enter a non-existing display name substring and apply it as search filter
    searchInput = "non existent sbom";
    applySearchFilterText(searchInput);

    // Assert that no search results are found
    cy.get("h1").contains("No results");
  });

  it("Filter checkboxes validations", function () {
    // Navigate to page
    clickByText(navMenu, SIDEBAR.SBOMs);

    // Enter an existing display name substring and assert
    applyCheckboxFilter(ProductsFilter.container);
    existsRow(SBOMList.ubi9.name);
    clickByText(button, clearAllFilters);

    applyRadioButtonFilter(ProductsFilter.createdOn.thisYear);
    existsRow(SBOMList.seedwingJavaExample.name);
    clickByText(button, clearAllFilters);

    // Enter a filter that will not generate results
    applyRadioButtonFilter(ProductsFilter.createdOn.last30Days);

    // Assert that no search results are found
    cy.get("h1").contains("No results");
  });
});
