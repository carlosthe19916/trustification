# Cypess E2E tests

## Developing guide

The tests require a running instance of Spog UI.

### How the data is injected?

Reasoning: Currently the UI has no way of injecting data through the UI, therefore data must be initialized somehow before all tests are executed.

Tests need an stable, repeatable, and realiable way of verifying data. A set of Advisories and SBOMs are injected before all the tests run using [./importer.js](importer.js)

[./importer.js](importer.js) contains hardcoded Keycloak Clients, and a set of HTTP POST request instructions to set up data.

### Setup and run of Cypress

- Configure Spog UI at `cypress.env.json`
- Install dependencies:

```shell
npm install
```

- Open Cypress Test Suite (select E2E tests):

```shell
npm run cypress:open
```

## Tips and tricks

### Execute only one test through UI

If a single file contains multiple tests then all of them will be executed in the UI, if you want to execute only one of them, select the appropiate file through Cypress UI and then replace the test definition from:

```javascript
it("TestName", function () {}
```

to:

```javascript
it.only("TestName", function () {}
```
