import { Kanjis } from "./components/Kanjis";
import { Tabs } from "./components/Tabs";
import { Words } from "./components/Words";

const tabs = {
  Words: () => <Words />,
  Kanji: () => <Kanjis />,
  "Parsed Words": () => {
    return (
      <div className="p-4">
        <h1 className="text-2xl font-bold mb-4">Parsed Words</h1>
        <p>This is a placeholder for the Parsed Words tab.</p>
      </div>
    );
  },
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
