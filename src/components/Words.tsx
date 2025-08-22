import { FixedSizeList as List, ListChildComponentProps } from "react-window";
import AutoSizer from "react-virtualized-auto-sizer";
import { isKanji } from "wanakana";
import { useMemo, useState } from "react";
import Fuse from "fuse.js";
import reactStringReplace from "react-string-replace";
import {
  BookOpenIcon,
  XCircleIcon,
  PlusIcon,
} from "@heroicons/react/24/outline";
import { WordAdder } from "./WordAdder";
import { Word } from "../bindings";
import { useAddWord, useWords } from "../queries";

type WordsState =
  | { kind: "idle" }
  | { kind: "word-details"; word: Word }
  | { kind: "add-word" };

export const Words = () => {
  const [state, setState] = useState<WordsState>({ kind: "idle" });
  const [searchQuery, setSearchQuery] = useState<null | string>(null);
  const { data, isLoading } = useWords();
  const words = useMemo(() => {
    return data?.status === "ok" ? data.data : [];
  }, [data]);
  const fuse = useMemo(() => {
    return new Fuse(words, {
      keys: ["word", "meaning"],
      threshold: 0.3,
    });
  }, [words]);
  const filteredWordsResults = useMemo(() => {
    if (!searchQuery || searchQuery.trim() === "") return words;
    return fuse.search(searchQuery);
  }, [words, searchQuery, fuse]);
  const filteredWords = useMemo(() => {
    return filteredWordsResults.map((res) => ("item" in res ? res.item : res));
  }, [filteredWordsResults]);
  const WordRowInternal = useMemo(
    () =>
      WordRow(
        searchQuery ?? "",
        (index) =>
          setState({ kind: "word-details", word: filteredWords[index] }),
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
            placeholder="Search words..."
            className="flex-4 border border-gray-300 rounded p-2"
            value={searchQuery ?? ""}
            onChange={(e) => setSearchQuery(e.target.value)}
          />
          <button
            title="Add new word"
            className="flex-1 bg-blue-500 text-white rounded hover:bg-blue-600 transition cursor-pointer"
            onClick={() => setState({ kind: "add-word" })}
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
                  itemCount={filteredWords.length}
                  itemData={filteredWords}
                >
                  {WordRowInternal}
                </List>
              )}
            </AutoSizer>
          )}
        </div>
      </div>
      <div className="bg-white flex-3 rounded-lg p-4 flex justify-center items-center">
        {state.kind === "idle" && <div>Nothing to see here, yet</div>}
        {state.kind === "word-details" && (
          <div>Word details for {state.word.word}</div>
        )}
        {state.kind === "add-word" && <WordAdd />}
      </div>
    </div>
  );
};

const WordRow =
  (
    query: string,
    onOpen: (index: number) => void,
    onRemove: (index: number) => void
  ) =>
  ({ index, style, data }: ListChildComponentProps) => {
    const word: Word = data[index];
    const pronunciation = word.word.split("").reduce(
      (acc, el) => {
        if (isKanji(el)) {
          const kanjiPron = word.kanji_readings[acc.kanji];
          acc.kanji += 1;
          acc.pron += kanjiPron ? kanjiPron.reading : el;
        } else acc.pron += el;
        return acc;
      },
      { kanji: 0, pron: "" }
    ).pron;
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
            <div className="font-medium">{replaceQuery(word.word)}</div>
            <div className="text-sm text-gray-400">({pronunciation})</div>
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
          {replaceQuery(word.meaning)}
        </div>
      </div>
    );
  };

const WordAdd = ({}: {}) => {
  const { mutateAsync: addWord } = useAddWord();
  return <WordAdder addWord={addWord} />;
};
