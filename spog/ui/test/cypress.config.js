const { defineConfig } = require("cypress");
const { importSboms, importAdvisories } = require("./importer");

module.exports = defineConfig({
  e2e: {
    setupNodeEvents(on, config) {
      on("before:run", (details) => {
        // If running with UI don't wait, but if running with terminal most probably we are on a CI environment so let's wait more
        const waitingTime = !details.config.isInteractive ? 10_000 : 500;
        return Promise.all([importAdvisories(), importSboms()]).then(() => {
          console.log(
            `We will wait ${waitingTime}ms the data imported to be indexed`
          );
          return new Promise((resolve) => setTimeout(() => resolve(), waitingTime));
        });
      });
    },
    retries: 2,
    experimentalInteractiveRunEvents: true,
    viewportWidth: 1366,
    viewportHeight: 768,
    testIsolation: false,
  },
});
