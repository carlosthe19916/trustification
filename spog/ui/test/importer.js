const axios = require("axios");
const path = require("path");
const fs = require("fs");

const SSO_URL = "http://localhost:8090";
const SSO_REALM = "chicken";

const tokenUrl = `${SSO_URL}/realms/${SSO_REALM}/protocol/openid-connect/token`;
const tokenQueryParams = new URLSearchParams({
  client_id: "walker",
  client_secret: "ZVzq9AMOVUdMY1lSohpx1jI3aW56QDPS",
  grant_type: "client_credentials",
});

const fixturesDirectory = path.join(__dirname, "cypress", "fixtures");

const sbomDirectory = path.join(fixturesDirectory, "sboms");
const sbomUrl = "http://127.0.0.1:8082/api/v1/sbom";

const advisoryDirectory = path.join(fixturesDirectory, "advisories");
const advisoryUrl = "http://127.0.0.1:8081/api/v1/vex";

const getToken = () => {
  return axios.post(tokenUrl, tokenQueryParams);
};

const getFiles = (directory) => {
  return fs.readdirSync(directory).map((file) => {
    const filePath = path.join(directory, file);
    return { filename: file, buffer: fs.readFileSync(filePath) };
  });
};

const importAdvisories = () => {
  const files = getFiles(advisoryDirectory);
  return getToken()
    .then((response) => {
      return axios.all(
        files.map((file) => {
          const body = JSON.parse(file.buffer.toString());
          return axios.post(advisoryUrl, body, {
            headers: {
              "Content-type": "application/json",
              Authorization: "Bearer " + response.data.access_token,
            },
          });
        })
      );
    })
    .then(() => {
      console.log("Advisories imported");
    })
    .catch((error) => {
      console.log(error);
    });
};

const importSboms = () => {
  const files = getFiles(sbomDirectory);
  return getToken()
    .then((response) => {
      return axios.all(
        files.map((file) => {
          const body = JSON.parse(file.buffer.toString());
          return axios.post(`${sbomUrl}?id=${file.filename}`, body, {
            headers: {
              "Content-type": "application/json",
              Authorization: "Bearer " + response.data.access_token,
            },
          });
        })
      );
    })
    .then(() => {
      console.log("SBOMs imported");
    })
    .catch((error) => {
      console.log(error);
    });
};

module.exports = {
  importAdvisories,
  importSboms,
};
