import { useEffect, useMemo, useState } from "react";
import { useMyKanjis, useMyWords } from "../queries";
import { isKanji } from "wanakana";
import AnnotatedText from "./AnnotatedText";
import { commands } from "../bindings";

export const ParsedWords = () => {
  const [word, setWord] = useState("時間");
  const [parsedWord, setParsedWord] = useState("");

  const { data: words } = useMyWords();
  const { data: kanjis } = useMyKanjis();

  const dataNotLoaded =
    !words || !kanjis || words.status !== "ok" || kanjis.status !== "ok";

  useEffect(() => {
    commands.parseWord(word).then(() => {
      setParsedWord(word);
    });
  }, [word]);

  return (
    <div>
      <div>
        {!dataNotLoaded &&
          words.data.map((w) => {
            const kanji = w.word
              .split("")
              .filter((char) => isKanji(char))
              .map((k, i) => [k, i]);
            const unknownKanji = kanji.filter(
              (k) => !kanjis.data.find((kk) => kk.kanji === k[0])
            );
            const annotations = unknownKanji.reduce((acc, [k, i]) => {
              acc[k] = w.kanji_readings[i as number].reading;
              return acc;
            }, {} as Record<string, string>);
            return (
              <span className="border p-2">
                <AnnotatedText text={w.word} annotations={annotations} />
              </span>
            );
          })}
      </div>
      <div>
        <input
          type="text"
          placeholder="Type a word..."
          className="m-2 p-2 border rounded"
          value={word}
          onChange={(e) => setWord(e.target.value)}
        />
      </div>
      <div>
        <button
          onClick={() => {
            commands.validateDictionary();
          }}
        >
          Validate all dictionary words
        </button>
      </div>
    </div>
  );
};
