import { useState } from "react";
import { useAddMyWord, useMyWordsModel } from "../queries";
import { Dialog } from "./Dialog";
import CheckBadgeIcon from "@heroicons/react/24/outline/CheckBadgeIcon";
import PlusIcon from "@heroicons/react/24/outline/PlusIcon";
import { WordAdder } from "./WordAdder";

export const WordCard = ({ word }: { word: SimpleWordRepresentation }) => {
  const wordModel = useMyWordsModel();
  const { mutateAsync: addWord } = useAddMyWord();
  const [dialogOpen, setDialogOpen] = useState(false);
  let parsed = word;
  return (
    <div className="p-3 rounded border border-gray-200 shadow-sm flex flex-row justify-between items-center gap-1">
      <div className="flex flex-col gap-1">
        <div className="flex flex-row gap-2 justify-start items-center">
          <div className="text-xl font-bold">{parsed.word}</div>
          <div className="text-sm text-gray-600 italic">({parsed.reading})</div>
        </div>
        <div className="text-base text-gray-700">{parsed.meaning}</div>
      </div>
      <div className="flex justify-center items-center">
        <Dialog
          open={dialogOpen}
          onOpenChange={setDialogOpen}
          trigger={
            wordModel?.has(parsed.word) ? (
              <CheckBadgeIcon className="h-8 w-8 text-green-500" />
            ) : (
              <button className="bg-blue-500 p-1 rounded cursor-pointer hover:bg-blue-600">
                <PlusIcon className="h-5 w-5 text-white" />
              </button>
            )
          }
          content={
            <WordAdder
              addWord={addWord}
              initialQuery={parsed.word}
              onSave={() => setDialogOpen(false)}
            />
          }
        />
      </div>
    </div>
  );
};

export type SimpleWordRepresentation = {
  word: string;
  reading: string;
  meaning: string;
};

export const parseHeisigWord = (
  word: string
): SimpleWordRepresentation | null => {
  const regexp = /(.+)\((.+)\):(.+)$/;
  const match = word.match(regexp);
  if (match) {
    return {
      word: match[1].trim(),
      reading: match[2].trim(),
      meaning: match[3].trim(),
    };
  }
  return null;
};
