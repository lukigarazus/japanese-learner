import { Kanjis } from "./components/Kanjis";
import { ParsedWords } from "./components/ParsedWords";
import { Tabs } from "./components/Tabs";
import { VerbConjugator } from "./components/VerbConjugator";
import { Words } from "./components/Words";

const databaseTabs = [
  { name: "Words", content: () => <Words /> },
  { name: "Kanji", content: () => <Kanjis /> },
];

const topTabs = [
  {
    name: "Database",
    content: () => <Tabs tabs={databaseTabs} />,
  },
  {
    name: "Parsed Words",
    content: () => <ParsedWords />,
  },
];

function App() {
  return (
    <main className="w-[100vw] h-[100vh]">
      <Tabs tabs={topTabs} defaultActive={1} />
    </main>
  );
}

export default App;
