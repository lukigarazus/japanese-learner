import { useMutation } from "@tanstack/react-query";

import { commands } from "../bindings";
import { useState } from "react";

const useKanjiSearch = () => {
  return useMutation({
    mutationFn: (char: string) => commands.searchKanji(char),
  });
};

export const KanjiSelector = () => {
  const [currentQuery, setQuery] = useState<null | string>(null);
  const { mutate: search, data } = useKanjiSearch();

  return (
    <div>
      <input
        type="text"
        value={currentQuery ?? ""}
        onFocus={() => setQuery(null)}
        onChange={(e) => setQuery(e.target.value)}
        onBlur={(e) => search(e.target.value)}
      />
      <div>{data?.status === "ok" && JSON.stringify(data.data)}</div>
    </div>
  );
};
