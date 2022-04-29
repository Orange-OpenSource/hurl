#!/usr/bin/env node

const os = require("os");
const path = require("path");
const cTable = require("console.table");
const archive = require("./archive");
//const {version} = require("./package.json");
// FIXME: temporary fix to test npm installation, we use an "hard coded" version of Hurl
const version = "1.6.1";

const supportedPlatforms = require("./platform.json")

function error(msg) {
    console.error(msg);
    process.exit(1);
}

function getPlatformMetadata() {
    const type = os.type();
    const architecture = os.arch();

    for (let supportedPlatform of supportedPlatforms) {
        if (type === supportedPlatform.type &&
            architecture === supportedPlatform.architecture
        ) {
            return supportedPlatform;
        }
    }
    error(
        `Platform with type "${type}" and architecture "${architecture}" is not supported.
        Your system must be one of the following:
        ${cTable.getTable(supportedPlatforms)}`
    );
}


const metadata = getPlatformMetadata();
const url = `https://github.com/Orange-OpenSource/hurl/releases/download/${version}/hurl-${version}-${metadata.rust_target}${metadata.archive_extension}`;
archive.install(url, path.join(__dirname, "bin"), metadata.checksum);