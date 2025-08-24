import { Kanjis } from "./components/Kanjis";
import { ParsedWords } from "./components/ParsedWords";
import { Tabs } from "./components/Tabs";
import { Words } from "./components/Words";

const tabs = {
  Words: () => <Words />,
  Kanji: () => <Kanjis />,
  "Parsed Words": () => <ParsedWords />,
};

function App() {
  return (
    <main className="w-[100vw] h-[100vh]">
      <Tabs
        defaultActive={2}
        tabs={Object.entries(tabs).map(([name, content]) => ({
          name,
          content,
        }))}
      />
    </main>
  );
}

export default App;
