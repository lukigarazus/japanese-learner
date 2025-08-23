import { FixedSizeList as List, ListChildComponentProps } from "react-window";
import AutoSizer from "react-virtualized-auto-sizer";
import { useMemo, useState } from "react";
import Fuse from "fuse.js";
import reactStringReplace from "react-string-replace";
import {
  BookOpenIcon,
  XCircleIcon,
  PlusIcon,
} from "@heroicons/react/24/outline";
import { Kanji } from "../bindings";
import { KanjiAdder } from "./KanjiAdder";
import { useAddMyKanji, useMyKanjis } from "../queries";

type KanjisState =
  | { kind: "idle" }
  | { kind: "kanji-details"; kanji: Kanji }
  | { kind: "add-kanji" };

export const Kanjis = () => {
  const [state, setState] = useState<KanjisState>({ kind: "idle" });
  const [searchQuery, setSearchQuery] = useState<null | string>(null);
  const { data, isLoading } = useMyKanjis();
  const kanjis = useMemo(() => {
    return data?.status === "ok" ? data.data : [];
  }, [data]);
  const fuse = useMemo(() => {
    return new Fuse(kanjis, {
      keys: ["kanji", "readings"],
      threshold: 0.3,
    });
  }, [kanjis]);
  const filteredKanjisResults = useMemo(() => {
    if (!searchQuery || searchQuery.trim() === "") return kanjis;
    return fuse.search(searchQuery);
  }, [kanjis, searchQuery, fuse]);
  const filteredKanjis = useMemo(() => {
    return filteredKanjisResults.map((res) => ("item" in res ? res.item : res));
  }, [filteredKanjisResults]);
  const KanjiRowInternal = useMemo(
    () =>
      KanjiRow(
        searchQuery ?? "",
        (index) =>
          setState({ kind: "kanji-details", kanji: filteredKanjis[index] }),
        () => {}
      ),
    [searchQuery]
  );
  return (
    <div className="h-full w-full flex flex-row gap-2 p-2">
      <div className="bg-white flex-1 max-w-xs rounded-lg p-4 flex flex-col">
        <div className="flex flex-row gap-1 mb-4 min-h-[40px]">
          <input
            type="text"
            placeholder="Search your kanjis..."
            className="flex-4 border border-gray-300 rounded p-2"
            value={searchQuery ?? ""}
            onChange={(e) => setSearchQuery(e.target.value)}
          />
          <button
            title="Add new word"
            className="flex-1 bg-blue-500 text-white rounded hover:bg-blue-600 transition cursor-pointer"
            onClick={() => setState({ kind: "add-kanji" })}
          >
            <PlusIcon className="h-5 w-5 m-auto" />
          </button>
        </div>
        <div className="overflow-y-auto w-full flex-1">
          {isLoading ? (
            "Loading..."
          ) : (
            <AutoSizer>
              {({ height, width }) => (
                <List
                  height={height}
                  width={width}
                  itemSize={100}
                  itemCount={filteredKanjis.length}
                  itemData={filteredKanjis}
                >
                  {KanjiRowInternal}
                </List>
              )}
            </AutoSizer>
          )}
        </div>
      </div>
      <div className="bg-white flex-3 rounded-lg p-4 flex justify-center items-center h-full">
        {state.kind === "idle" && <div>Nothing to see here, yet</div>}
        {state.kind === "kanji-details" && (
          <div>Kanji details for {state.kanji.kanji}</div>
        )}
        {state.kind === "add-kanji" && <KanjiAdd />}
      </div>
    </div>
  );
};

const KanjiRow =
  (
    query: string,
    onOpen: (index: number) => void,
    onRemove: (index: number) => void
  ) =>
  ({ index, style, data }: ListChildComponentProps) => {
    const kanji: Kanji = data[index];
    const replaceQuery = (text: string) => {
      return reactStringReplace(text, query, (item) => (
        <span className="bg-yellow-500">{item}</span>
      ));
    };
    return (
      <div
        style={style}
        className="p-2 border-b border-gray-300 last:border-0 flex flex-col justify-start items-start overflow-y-auto"
      >
        <div className="flex flex-row justify-between w-full items-center">
          <div className="flex flex-row">
            <div className="font-bold text-lg">{replaceQuery(kanji.kanji)}</div>
          </div>
          <div>
            <button
              title="Open details"
              onClick={() => onOpen(index)}
              className="p-1 rounded hover:bg-gray-200 cursor-pointer"
            >
              <BookOpenIcon className="h-5 w-5 text-gray-600" />
            </button>
            <button
              title="Remove word"
              className="p-1 rounded hover:bg-gray-200 cursor-pointer"
            >
              <XCircleIcon className="h-5 w-5 text-red-600" />
            </button>
          </div>
        </div>
        <div className="text-sm text-gray-500">
          {replaceQuery(kanji.readings.join(", "))}
        </div>
      </div>
    );
  };

const KanjiAdd = ({}: {}) => {
  const { mutateAsync: addKanji } = useAddMyKanji();
  return <KanjiAdder addKanji={addKanji} />;
};
