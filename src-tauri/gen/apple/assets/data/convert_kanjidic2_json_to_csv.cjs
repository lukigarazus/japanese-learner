const fs = require("fs");

const file = fs.readFileSync("./kanjidic2.json", "utf8");

const json = JSON.parse(file);

const characters = json;

const csv = {
  headers: ["literal", "ja_on", "ja_kun", "heisig", "heisig6"],
  rows: [],
};

for (const char of characters) {
  const group = char.readingMeaning.groups[0];
  const ja_on =
    group.readings
      .filter((r) => r.type === "ja_on")
      .map((r) => r.value)
      .join(";") || "";
  const ja_kun =
    group.readings
      .filter((r) => r.type === "ja_kun")
      .map((r) => r.value)
      .join(";") || "";
  const references = char.dictionaryReferences;
  const heisig = references.find((r) => r.type === "heisig")?.value || "";
  const heisig6 = references.find((r) => r.type === "heisig6")?.value || "";

  csv.rows.push([char.literal, ja_on, ja_kun, heisig, heisig6]);
}

const csvContent = [
  csv.headers.join(","),
  ...csv.rows.map((row) => row.join(",")),
].join("\n");

fs.writeFileSync("./kanjidic2.csv", csvContent, "utf8");
