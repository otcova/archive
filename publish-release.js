const fs = require("fs")
const path = require("path")

const exitWithError = msg => {
	console.error("\x1b[31m%s\x1b[0m", msg)
	process.exit(1)
}

////////////  Loading Version  //////////////////////////////////////////////////

let raw_config_data
try { raw_config_data = fs.readFileSync("./src-tauri/tauri.conf.json") }
catch (error) { exitWithError("Unable to read tauri.conf.json") }

let config
try { config = JSON.parse(raw_config_data) }
catch (error) { exitWithError("Unable to parse tauri.conf.json") }

if (!config.package?.version)
	exitWithError("missing field 'package.version' on tauri.conf.json")
let version = config.package?.version

//////////  Move artifacts  /////////////////////////////////////////////////

let msi_dir
try { msi_dir = fs.readdirSync(`./releases/msi`) }
catch (error) { exitWithError("Could not read directory because: " + error) }

try { fs.mkdirSync("./releases/past-msi", { recursive: true }) }
catch (error) { exitWithError("Could not create path because: " + error) }

for (const file of msi_dir) {
	let name = "archive" + (file.match(/(\.msi|\.msi\.zip|\.msi\.zip\.sig)$/g)[0] || "");
	try { fs.renameSync("./releases/msi/" + file, "./releases/past-msi/" + name) }
	catch (error) { exitWithError("Could not move file because: " + error) }
}

try { fs.cpSync("./src-tauri/target/release/bundle", "./releases", { recursive: true, force: true }) }
catch (error) { exitWithError("Could not copy releases because: " + error) }

////////////  Create version_report.json  ///////////////////////////////////////////

let artifact_path = `/releases/msi/archive_${version}_x64_en-US.msi.zip`

let signature
try { signature = fs.readFileSync("." + artifact_path + ".sig").toString() }
catch (error) { exitWithError("Could not load signature because: " + error) }

const version_report = {
	version,
	pub_date: new Date().toISOString(),
	platforms: {
		"windows-x86_64": {
			signature,
			url: `https://raw.githubusercontent.com/otcova/archive/main${artifact_path}`
		}
	}
}
try { fs.writeFileSync("./releases/version_report.json", JSON.stringify(version_report, null, "\t")) }
catch (error) { exitWithError("Could not write version_report.json because: " + error) }

////////////  Increase Version  /////////////////////////////////////////////////

let [major, median, minor] = config.package?.version.split(".").map(n => Number(n))
if (Number.isNaN(major) || Number.isNaN(median) || Number.isNaN(minor))
	exitWithError("field 'package.version: " + config.package?.version + "' on tauri.conf.json is invalid")

let new_version = major + "." + median + "." + (minor + 1)
config.package.version = new_version

try { fs.writeFileSync("./src-tauri/tauri.conf.json", JSON.stringify(config, null, "\t")) }
catch (error) { exitWithError("Could not write to tauri.conf.json because: " + error) }

////////////  Clean artifacts from target/bundle  /////////////////////////////////////

try { fs.rmSync("./src-tauri/target/release/bundle", { recursive: true, force: true }) }
catch (error) { exitWithError("Could not clean releases because: " + error) }


///////////  Publish new version on Git ///////////////////////////////////////////////

const { execSync } = require("child_process");

try { execSync("git add .") }
catch(error) { exitWithError("Could not 'git add .' because: " + error) }

try { execSync(`git commit -m "release v${version}"`) }
catch(error) { exitWithError("Could not 'git add .' because: " + error) }

try { execSync("git push") }
catch(error) { exitWithError("Could not 'git add .' because: " + error) }