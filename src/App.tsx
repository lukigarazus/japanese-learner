import { Kanjis } from "./components/Kanjis";
import { ParsedWords } from "./components/ParsedWords";
import { Tabs } from "./components/Tabs";
import { Words } from "./components/Words";

const databaseTabs = [
  { name: "Words", content: () => <Words /> },
  { name: "Kanji", content: () => <Kanjis /> },
  {
    name: "Parsed Words",
    content: () => <ParsedWords />,
  },
];

const topTabs = [
  {
    name: "Database",
    content: () => (
      <div className="p-1 size-full">
        <Tabs tabs={databaseTabs} />
      </div>
    ),
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
