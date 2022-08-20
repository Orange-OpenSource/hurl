/*
 * Hurl (https://hurl.dev)
 * Copyright (C) 2022 Orange
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *          http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
 *
 */
const fs = require("fs");
const path = require("path");
const crypto = require("crypto");

const rimraf = require("rimraf");
const axios = require("axios");
const tar = require("tar");
const extract = require("extract-zip")


/**
 * Install executables in a folder.
 * @param url url of a tar archive (or zip archive on Windows) containing executable
 * @param dir installation folder
 * @param checksum SHA256 checksum of the archive
 */
function install(url, dir, checksum) {
    console.log(`Downloading release from ${url} to ${dir}`);

    // Install a fresh bin directory.
    if (fs.existsSync(dir)) {
        rimraf.sync(dir);
    }
    if (!fs.existsSync(dir)) {
        fs.mkdirSync(dir, { recursive: true });
    }

    axios({url: url, responseType: "stream" })
        .then(res => {
            return new Promise((resolve, reject) => {
                // Linux, macOS archives are tar.gz files.
                if (url.endsWith(".tar.gz")) {
                    const archive = path.join(dir, "archive.tar.gz");
                    const sink = res.data.pipe(
                        fs.createWriteStream(archive)
                    )
                    sink.on("finish", () => {
                        verifyCheckSum(archive, checksum);
                        tar.x({ strip: 1, C: dir, file: archive });
                        resolve();
                    });
                    sink.on("error", err => reject(err));
                }
                // Windows archive is a zip archive.
                else if (url.endsWith(".zip")) {
                    const archive = path.join(dir, "archive.zip");
                    const sink = res.data.pipe(
                        fs.createWriteStream(archive)
                    )
                    sink.on("finish", () => {
                        verifyCheckSum(archive, checksum);
                        extract(archive, {dir: dir})
                            .then( () => resolve())
                            .catch( err => reject(err));
                    });
                    sink.on("error", err => reject(err));
                } else {
                    console.error("Error unsupported archive");
                    process.exit(1);
                }
            });
        })
        .then(() => {
            console.log(`Archive has been installed to ${dir}!`);
        })
        .catch(e => {
            console.error(`Error fetching release: ${e.message}`);
            process.exit(1);
        });
}

/**
 * Exits process with error if the SHA256 checksum of file is not equal to the expected checksum.
 * @param file input file
 * @param expectedChecksum expected checksum
 */
function verifyCheckSum(file, expectedChecksum) {
    const checksum = sha256(file);
    if (expectedChecksum !== checksum) {
        console.error(`Downloaded archive checksum didn't match the expected checksum (actual: ${checksum}, expected ${expectedChecksum}`);
        process.exit(1)
    }
}

/**
 * Returns the SHA256 checksum of a file.
 * @param file input file
 * @returns checksum as a string of hex digits
 */
function sha256(file) {
    const data = fs.readFileSync(file);
    return crypto.createHash("sha256").update(data).digest("hex").toLowerCase();
}

exports.install = install;