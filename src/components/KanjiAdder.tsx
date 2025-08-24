import { useState } from "react";
import { commands, HeisigKanjiPayload, KanjiCreatePayload } from "../bindings";
import { Autocomplete } from "./Autocomplete";
import { isJapanese, isKana, isKanji } from "wanakana";
import reactStringReplace from "react-string-replace";
import { CheckBadgeIcon, PlusIcon } from "@heroicons/react/24/outline";
import { Dialog } from "./Dialog";
import { WordAdder } from "./WordAdder";
import { useAddMyWord, useKanjidic2, useMyWordsModel } from "../queries";
import { parseHeisigWord, WordCard } from "./WordCard";

export const KanjiAdder = ({
  addKanji,
}: {
  addKanji: (
    kanji: KanjiCreatePayload
  ) => Promise<{ status: "ok" } | { status: "error"; error: string }>;
}) => {
  const [selectedKanji, setSelectedKanji] = useState<HeisigKanjiPayload | null>(
    null
  );
  return (
    <div className="w-full h-full flex flex-col gap-4 overflow-y-auto p-4">
      <div>
        <Autocomplete<HeisigKanjiPayload>
          validateInput={validateInput}
          fetchData={fetchKanjis}
          onSelect={(item) => {
            setSelectedKanji(item);
          }}
          renderItem={SingleKanjiSelection}
        />
      </div>

      {selectedKanji && (
        <KanjiDisplay
          selectedKanji={selectedKanji}
          saveKanji={async (kanji) => {
            const res = await addKanji(kanji);
            if (res.status === "ok") {
              setSelectedKanji(null);
            }
            return res;
          }}
        />
      )}
    </div>
  );
};

const KanjiDisplay = ({
  selectedKanji,
  saveKanji,
}: {
  selectedKanji: HeisigKanjiPayload;
  saveKanji: (
    kanji: KanjiCreatePayload
  ) => Promise<{ status: "ok" } | { status: "error"; error: string }>;
}) => {
  const [mnemonic, setMnemonic] = useState("");
  const [error, setError] = useState<string | null>(null);
  const { data: kanjidic2Entry } = useKanjidic2(selectedKanji.kanji);
  console.log(kanjidic2Entry);
  return (
    <>
      <div className="font-bold flex flex-row justify-between items-center">
        <span className="text-2xl">{selectedKanji.kanji}</span>
        <button
          className="bg-blue-500 text-white p-1 rounded hover:bg-blue-600 transition cursor-pointer"
          onClick={async () => {
            const res = await saveKanji({
              kanji: selectedKanji.kanji,
              readings: selectedKanji.pronunciation
                .split(",")
                .map((s) => s.trim()),

              tags: [],
              writing_mnemonic: mnemonic.trim() === "" ? null : mnemonic,
              reading_mnemonic: null,
            });
            if (res.status === "error") {
              setError(res.error);
            }
          }}
        >
          Add Kanji
        </button>
      </div>
      {error && <div className="text-red-500 font-semibold">{error}</div>}
      <div>
        <h3 className="font-semibold">Pronunciation:</h3>
        {selectedKanji.pronunciation}
      </div>
      <div>
        <h3 className="font-semibold">Primitives:</h3>
        {selectedKanji.primitives.join(", ")}
      </div>
      <div>
        <h3 className="font-semibold">My Mnemonic</h3>
        <input
          type="text"
          className="border border-gray-300 rounded p-2 w-full"
          placeholder="Enter your mnemonic"
          value={mnemonic}
          onChange={(e) => setMnemonic(e.target.value)}
        />
      </div>
      <CollapsibleMnemonics selectedKanji={selectedKanji} />
      <div>
        <h3 className="font-semibold">Sample words</h3>
        <div className="flex flex-col gap-2">
          {selectedKanji.words.map((word, idx) => {
            const parsed = parseHeisigWord(word);
            if (!parsed) return null;
            return <WordCard word={parsed} key={idx} />;
          })}
        </div>
      </div>
    </>
  );
};

const validateInput = (query: string) => {
  if (parseQuery(query) === null)
    return "Please enter a single kanji character, a reading, or comma-separated Heisig keywords.";
};

const fetchKanjis = async (query: string) => {
  const queryParsed = parseQuery(query);
  if (queryParsed === null) return [];
  const res = await commands.searchHeisigKanji(queryParsed);
  if (res.status === "ok" && res.data)
    return res.data.sort((a, b) => (b.jlpt_level ?? 0) - (a.jlpt_level ?? 0));
  return [];
};

const SingleKanjiSelection = (item: HeisigKanjiPayload, query: string) => {
  const highlightQuery = (text: string) => {
    if (!query) return text;
    return reactStringReplace(text, query, (match, i) => (
      <span key={i} className="bg-yellow-200">
        {match}
      </span>
    ));
  };
  return (
    <div>
      <span className="font-semibold">{highlightQuery(item.kanji)}</span>
      <span className="text-gray-500">
        ({highlightQuery(item.pronunciation)})
      </span>
      <span className="text-gray-600">
        - {highlightQuery(item.primitives.join(", "))}
      </span>
      <span className="text-gray-600">
        {" "}
        [{item.jlpt_level ? `JLPT N${item.jlpt_level}` : "no JLPT"}]
      </span>
    </div>
  );
};

const parseQuery = (query: string) => {
  const trimmed = query.trim();
  if (trimmed.length === 1 && isKanji(trimmed)) {
    return { Kanji: trimmed };
  }
  if ([...trimmed].every(isKana)) {
    return { Reading: trimmed };
  }
  const keywords = trimmed
    .split(",")
    .map((part) => part.trim())
    .filter((part) => part.length > 0 && ![...part].some((c) => isJapanese(c)));
  if (keywords.length > 0) {
    return { Keywords: keywords };
  }
  return null;
};

const CollapsibleMnemonics = ({
  selectedKanji,
}: {
  selectedKanji: HeisigKanjiPayload;
}) => {
  const [open, setOpen] = useState(false);

  const hasAnyMnemonic =
    selectedKanji.heisig_mnemonic ||
    selectedKanji.koohii_mnemonic_1 ||
    selectedKanji.koohii_mnemonic_2;

  if (!hasAnyMnemonic) return null;

  return (
    <div>
      <button
        className="font-semibold underline text-blue-600 mb-2"
        onClick={() => setOpen((v) => !v)}
        type="button"
      >
        {open ? "Hide" : "Show"} Writing Mnemonics
      </button>
      {open && (
        <div>
          {selectedKanji.heisig_mnemonic && (
            <div className="mb-2">
              <h4 className="font-semibold">Heisig:</h4>
              <p
                dangerouslySetInnerHTML={{
                  __html: selectedKanji.heisig_mnemonic,
                }}
              ></p>
            </div>
          )}
          {selectedKanji.koohii_mnemonic_1 && (
            <div className="mb-2">
              <h4 className="font-semibold">Koohii 1:</h4>
              <p>{selectedKanji.koohii_mnemonic_1}</p>
            </div>
          )}
          {selectedKanji.koohii_mnemonic_2 && (
            <div className="mb-2">
              <h4 className="font-semibold">Koohii 2:</h4>
              <p>{selectedKanji.koohii_mnemonic_2}</p>
            </div>
          )}
        </div>
      )}
    </div>
  );
};
