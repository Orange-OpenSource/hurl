#!/usr/bin/env node

const os = require("os");
const path = require("path");
const archive = require("./archive");
const {hurlBinaryVersion} = require("./package.json");

const supportedPlatforms = require("./platform.json");

function error(msg) {
    console.error(msg);
    process.exit(1);
}

function getPlatformMetadata() {
    const type = os.type();
    const architecture = os.arch();

    for (const supportedPlatform of supportedPlatforms) {
        if (type === supportedPlatform.type &&
            architecture === supportedPlatform.architecture
        ) {
            return supportedPlatform;
        }
    }
    const platforms = supportedPlatforms.map((p) => `${p.type} ${p.architecture}`)
        .join("\n");
    error(
        `Platform with type "${type}" and architecture "${architecture}" is not supported.
Your system must be one of the following:
${platforms}`
    );
}


const metadata = getPlatformMetadata();
const url = `https://github.com/Orange-OpenSource/hurl/releases/download/${hurlBinaryVersion}/hurl-${hurlBinaryVersion}-${metadata.rust_target}${metadata.archive_extension}`;
archive.install(url, path.join(__dirname, "dist"), metadata.checksum);