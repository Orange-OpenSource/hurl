/*
 * Hurl (https://hurl.dev)
 * Copyright (C) 2024 Orange
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
const { Readable } = require('stream');
const tar = require("tar");
const extract = require("extract-zip");


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
        fs.rmSync(dir, {recursive: true});
    }
    fs.mkdirSync(dir, {recursive: true});

    fetch(url)
        .then(res => {
            if (!res.ok) {
                console.error(`Error fetching release ${url}: ${res.statusText}`);
                process.exit(1);
            }

            // Check archive extension.
            const isWindows = url.endsWith(".zip");
            const isUnixLike = url.endsWith(".tar.gz");
            if (!isWindows && !isUnixLike) {
                console.error("Error: unsupported archive type");
                process.exit(1);
            }

            const archive = isWindows ? "archive.zip" : "archive.tar.gz";
            const archivePath = path.join(dir, archive);
            const fileStream = fs.createWriteStream(archivePath);
            const readable = Readable.fromWeb(res.body);

            return new Promise((resolve, reject) => {
                readable.pipe(fileStream)
                    .on("finish", () => {
                        try {
                            verifyCheckSumSync(archivePath, checksum);
                        } catch (e) {
                            return reject(e);
                        }

                        const extractor = isWindows
                            ? extract(archivePath, { dir })
                            : tar.x({ strip: 1, C: dir, file: archivePath });

                        extractor.then(resolve).catch(reject);
                    })
                    .on("error", reject);
            });
        })
        .then(() => {
            console.log(`Archive has been installed to ${dir}!`);
        })
        .catch(e => {
            console.error(`Installation failed: ${e.message}`);
            process.exit(1);
        });
}

/**
 * Exits process with error if the SHA256 checksum of file is not equal to the expected checksum.
 * @param file input file
 * @param expectedChecksum expected checksum
 */
function verifyCheckSumSync(file, expectedChecksum) {
    const checksum = sha256(file);
    if (expectedChecksum !== checksum) {
        console.error(`Downloaded archive checksum didn't match the expected checksum (actual: ${checksum}, expected ${expectedChecksum}`);
        process.exit(1);
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
