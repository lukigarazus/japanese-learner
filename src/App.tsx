import { Kanjis } from "./components/Kanjis";
import { Tabs } from "./components/Tabs";
import { Words } from "./components/Words";

const tabs = {
  Words: () => <Words />,
  Kanji: () => <Kanjis />,
};

function App() {
  return (
    <main className="w-[100vw] h-[100vh]">
      <Tabs
        tabs={Object.entries(tabs).map(([name, content]) => ({
          name,
          content,
        }))}
      />
    </main>
  );
}

export default App;
