import { useMutation } from "@tanstack/react-query";
import { useEffect, useMemo, useState } from "react";
import { isKana, isJapanese } from "wanakana";
import { commands, MyEntryDisplay, WordCreatePayload } from "../bindings";
import { Autocomplete } from "./Autocomplete";

const useChars = (currentWord: string | null) => {
  const chars = useMemo(() => {
    return (currentWord?.split("") ?? []).reduce(
      (acc, char) => {
        const val = {
          char,
          isKanji: isJapanese(char) && !isKana(char),
          index: -1,
        };
        acc.arr.push(val);
        if (val.isKanji) {
          val.index = acc.kanji;
          acc.kanji++;
        }
        return acc;
      },
      {
        kanji: 0,
        arr: [] as {
          char: string;
          isKanji: boolean;
          index: number;
        }[],
      }
    );
  }, [currentWord]);

  return chars;
};

const useSearchKanjis = () => {
  return useMutation({
    mutationFn: (chars: string[]) => commands.searchHeisigKanjis(chars),
  });
};

const useKanjis = (
  chars: { char: string; isKanji: boolean; index: number }[]
) => {
  const { mutate: search, data: kanjis } = useSearchKanjis();
  useEffect(() => {
    if (chars.some((char) => char.isKanji)) {
      search(chars.filter((char) => char.isKanji).map((char) => char.char));
    }
  }, [chars]);
  return kanjis;
};

// const useDictionary = (text: string | null) => {
//   const { mutate, data } = useMutation({
//     mutationFn: (text: string) => commands.getWordDictEntry(text),
//   });

//   useEffect(() => {
//     if (text) {
//       mutate(text);
//     }
//   }, [text, mutate]);

//   if (!text) return null;

//   return data;
// };

export const WordAdder = ({
  addWord,
}: {
  addWord: (word: WordCreatePayload) => Promise<
    | {
        status: "ok";
      }
    | { status: "error"; error: string }
  >;
}) => {
  const [dictionaryWord, setDictionaryWord] = useState<MyEntryDisplay | null>(
    null
  );
  const [kanjiPronunciations, setKanjiPronunciations] = useState<
    { id: string; pronunciation: string; error: string | null }[]
  >([]);
  const [currentChar, setCurrentChar] = useState<null | number>(null);
  const [error, setError] = useState<string | null>(null);
  const chars = useChars(dictionaryWord?.word ?? null);
  const kanjis = useKanjis(chars.arr);

  const meaning = dictionaryWord?.translations ?? null;
  const setMeaning = (meaning: string) => {
    if (dictionaryWord) {
      setDictionaryWord({ ...dictionaryWord, translations: meaning });
    }
  };

  const onSave = () => {
    if (!dictionaryWord) return;
    if (
      chars.arr.some(
        (char) =>
          (char.isKanji && !kanjiPronunciations[char.index]) ||
          kanjiPronunciations[char.index]?.error
      )
    ) {
      alert("Please fill in all kanji pronunciations with kana");
      return;
    }
    addWord({
      word: dictionaryWord.word,
      meaning: meaning ?? "",
      kanji_readings: kanjiPronunciations.map((kp) => ({
        reading: kp.pronunciation,
      })),
    }).then((res) => {
      if (res.status === "ok") {
        setDictionaryWord(null);
        setKanjiPronunciations([]);
        setCurrentChar(null);
        setError(null);
      } else if (res.status === "error") {
        setError(res.error);
      }
    });
  };

  useEffect(() => {
    setKanjiPronunciations([]);
    setCurrentChar(null);
    setError(null);
  }, [dictionaryWord]);

  return (
    <div className="min-w-lg bg-white rounded-xl p-8">
      <h2 className="text-2xl font-bold mb-6 text-center text-gray-800">
        Add a Word
      </h2>
      <div className="mb-4">
        {/* <label
          className="block text-gray-700 font-medium mb-1"
          htmlFor="word-input"
        >
          Word
        </label>
        <input
          id="word-input"
          className="w-full px-4 py-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-400 transition"
          placeholder="Word"
          type="text"
          value={currentWord ?? ""}
          onChange={(e) => setWord(e.target.value)}
        /> */}
        <Autocomplete<MyEntryDisplay>
          fetchData={async (v) => {
            const res = await commands.getWordCandidates(v);
            if (res.status === "ok" && res.data) {
              return res.data;
            } else if (res.status === "error") {
              console.error("Error fetching word candidates:", res.error);
              return [];
            }
            return [];
          }}
          onSelect={setDictionaryWord}
          renderItem={(item) => (
            <div className="p-2 hover:bg-gray-100 transition">
              <span className="font-semibold">{item.word}</span>
              <span className="text-gray-500"> ({item.reading})</span>
              <span className="text-gray-600"> - {item.translations}</span>
            </div>
          )}
          placeholder="Search for a word..."
          value={dictionaryWord?.word}
        />
      </div>

      <div className="mb-4">
        {dictionaryWord ? (
          <span className="text-blue-600 font-medium">
            Dictionary pronunciation: {dictionaryWord.reading}
          </span>
        ) : null}
      </div>

      <div className="flex gap-2 mb-4 text-xl">
        {chars.arr.map((char) => (
          <span
            key={char.index !== -1 ? `${char.char}-${char.index}` : char.char}
            className={
              char.isKanji
                ? currentChar === char.index
                  ? "text-blue-600 font-bold border-b-2 border-blue-400"
                  : kanjiPronunciations[char.index] &&
                    !kanjiPronunciations[char.index].error
                  ? "text-green-600 font-bold"
                  : "text-red-600 font-bold"
                : "text-gray-800"
            }
          >
            {char.char}
          </span>
        ))}
      </div>

      <div className="mb-4 flex flex-col gap-2">
        {kanjis ? (
          kanjis.status === "ok" ? (
            chars.arr.map((char, index) => {
              if (!char.isKanji) return null;
              return (
                <input
                  key={char.index}
                  className={`w-full px-3 py-2 border rounded-lg focus:outline-none focus:ring-2 ${
                    currentChar === char.index
                      ? "focus:ring-blue-400"
                      : "focus:ring-gray-200"
                  } transition`}
                  placeholder={`Pronunciation for ${char.char}`}
                  type="text"
                  value={kanjiPronunciations[index]?.pronunciation ?? ""}
                  onFocus={() => setCurrentChar(char.index)}
                  onBlur={() => setCurrentChar(null)}
                  onChange={(e) => {
                    const newPronunciations = [...kanjiPronunciations];
                    newPronunciations[index] = {
                      id: kanjis.data[char.index].id,
                      pronunciation: e.target.value,
                      error: e.target.value
                        .trim()
                        .split("")
                        .some((c) => !isKana(c) && !isJapanese(c))
                        ? "Invalid pronunciation"
                        : null,
                    };
                    setKanjiPronunciations(newPronunciations);
                  }}
                />
              );
            })
          ) : (
            <span className="text-gray-400">Loading kanji data...</span>
          )
        ) : null}
      </div>

      <div className="mb-4">
        {currentChar !== null && kanjis && kanjis.status === "ok" && (
          <pre className="bg-gray-100 rounded p-2 text-xs text-gray-600 overflow-x-auto">
            {JSON.stringify(kanjis.data[currentChar], null, 2)}
          </pre>
        )}
      </div>

      <div className="mb-4">
        <label
          className="block text-gray-700 font-medium mb-1"
          htmlFor="meaning-input"
        >
          Meaning
        </label>
        <input
          id="meaning-input"
          className="w-full px-4 py-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-400 transition"
          placeholder="Meaning"
          type="text"
          value={meaning ?? ""}
          onChange={(e) => setMeaning(e.target.value)}
        />
        {/* <div className="mt-2 text-gray-600 text-sm">
          Dictionary translation:{" "}
          {dictionary?.status === "ok"
            ? dictionary.data
              ? dictionary.data.translations
              : "No translation found"
            : null}
        </div> */}
      </div>

      <div className="flex justify-end">
        <button
          disabled={
            !chars.arr.length ||
            chars.arr.some(
              (char) => char.isKanji && !kanjiPronunciations[char.index]
            )
          }
          onClick={onSave}
          className="px-6 py-2 rounded-lg bg-blue-600 text-white font-semibold shadow hover:bg-blue-700 transition disabled:opacity-50 disabled:cursor-not-allowed"
        >
          Save
        </button>
      </div>

      {error && (
        <div className="mt-4 p-3 bg-red-100 text-red-700 border border-red-400 rounded">
          {error}
        </div>
      )}
    </div>
  );
};
