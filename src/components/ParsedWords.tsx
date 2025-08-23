import { useMemo } from "react";
import { useMyKanjis, useMyWords } from "../queries";
import { isKanji } from "wanakana";
import AnnotatedText from "./AnnotatedText";

export const ParsedWords = () => {
  const { data: words } = useMyWords();
  const { data: kanjis } = useMyKanjis();

  const dataNotLoaded =
    !words || !kanjis || words.status !== "ok" || kanjis.status !== "ok";

  const parsedWords = useMemo(() => {
    if (dataNotLoaded) return [];
    const ws = words.data;
    const ks = kanjis.data;
    return ws.map((w) => {
      return {
        front: w.word.split("").reduce(
          (acc, el) => {
            return acc;
          },
          { kanji: 0, res: "" }
        ).res,
        back: w.meaning,
      };
    });
  }, [words, kanjis]);
  return (
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
  );
};
