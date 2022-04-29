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

const path = require("path");
const child_process = require("child_process");
const os = require("os");
const fs = require("fs");

/**
 * Execute a binary by name uniformly on Windows and Unix*
 * @param name name of the binary (without extension)
 */
function run(name) {
    const execPath = path.join(__dirname, "bin", os.platform() === "win32" ? name + ".exe" : name);

    try {
        const result = child_process.spawnSync(
            execPath,
            process.argv.slice(2),
            { stdio: "inherit" },
        );

        if (result.status !== 0) {
            throwIfNoExePath(execPath);
        }

        process.exitCode = result.status;
    } catch (err) {
        throwIfNoExePath(execPath);
        throw err;
    }
}

function throwIfNoExePath(execPath) {
    if (!fs.existsSync(execPath)) {
        throw new Error("Could not find exe at path '" + exePath + "'. Please ensure the hurl 'postinstall' script runs on install.");
    }
}

exports.run = run;