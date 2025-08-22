const fs = require("fs");
const path = require("path");
const toml = require("@iarna/toml");
const { execSync } = require("child_process");

const rootDir = path.resolve(__dirname, "..");
const cargoTomlPath = path.join(rootDir, "Cargo.toml");
const packageJsonPath = path.join(rootDir, "package.json");

try {
  const localCargoTomlContent = fs.readFileSync(cargoTomlPath, "utf8");
  const localCargoConfig = toml.parse(localCargoTomlContent);
  const packageName = localCargoConfig.package.name;

  if (!packageName) {
    throw new Error("Package name not found in Cargo.toml");
  }

  const metadataOutput = execSync(
    "cargo metadata --no-deps --format-version=1"
  ).toString();
  const metadata = JSON.parse(metadataOutput);

  const currentPackage = metadata.packages.find((p) => p.name === packageName);

  if (!currentPackage) {
    throw new Error(
      `Package "${packageName}" not found in "cargo metadata" output.`
    );
  }

  const newVersion = currentPackage.version;

  const packageJsonContent = fs.readFileSync(packageJsonPath, "utf8");
  const packageConfig = JSON.parse(packageJsonContent);

  if (packageConfig.version === newVersion) {
    console.log(`Version is already up to date: ${newVersion}`);
  } else {
    const oldVersion = packageConfig.version;
    packageConfig.version = newVersion;
    fs.writeFileSync(
      packageJsonPath,
      JSON.stringify(packageConfig, null, 2) + "\n"
    );
    console.log(
      `✅ Version in package.json updated from ${oldVersion} to ${newVersion} (resolved from workspace)`
    );
  }
} catch (error) {
  console.error("Error syncing version using cargo metadata:", error);
  process.exit(1);
}
