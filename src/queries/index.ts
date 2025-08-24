import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { commands, Kanji, Word } from "../bindings";
import { useEffect, useMemo } from "react";

export const useMyWords = () => {
  return useQuery({
    queryKey: ["words"],
    queryFn: () => commands.getWords(),
    staleTime: Infinity,
  });
};

export class MyWordsModel {
  private wordMap: Map<string, Word>;
  private wordSet: Set<string>;

  constructor(words: Word[]) {
    this.wordMap = new Map(words.map((word) => [word.id, word]));
    this.wordSet = new Set(words.map((word) => word.word));
  }

  has(word: string): boolean {
    return this.wordSet.has(word);
  }
}
export const useMyWordsModel = (): MyWordsModel | null => {
  const { data } = useMyWords();

  return useMemo(() => {
    return data?.status === "ok" ? new MyWordsModel(data.data) : null;
  }, [data]);
};

export const useAddMyWord = () => {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: commands.addWord,
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["words"] });
    },
  });
};

export const useMyKanjis = () => {
  return useQuery({
    queryKey: ["kanjis"],
    queryFn: () => commands.getKanjis(),
  });
};

export class MyKanjisModel {
  private kanjiMap: Map<string, Kanji>;
  private kanjiSet: Set<string>;

  constructor(private kanjis: Kanji[]) {
    this.kanjiMap = new Map(kanjis.map((kanji) => [kanji.id, kanji]));
    this.kanjiSet = new Set(kanjis.map((kanji) => kanji.kanji));
  }

  has(kanji: string): boolean {
    return this.kanjiSet.has(kanji);
  }
}
export const useMyKanjisModel = (): MyKanjisModel | null => {
  const { data } = useMyKanjis();

  return useMemo(() => {
    return data?.status === "ok" ? new MyKanjisModel(data.data) : null;
  }, [data]);
};

export const useAddMyKanji = () => {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: commands.addKanji,
    onSuccess: async () => {
      await queryClient.invalidateQueries({ queryKey: ["kanjis"] });
    },
  });
};

export const useFindInKanjidic2 = () => {
  return useMutation({
    mutationFn: (kanji: string) => commands.getKanjidic2ByKanji(kanji),
  });
};

export const useKanjidic2 = (kanji: string) => {
  const { mutate, data } = useFindInKanjidic2();
  useEffect(() => {
    if (kanji) {
      mutate(kanji);
    }
  }, [kanji, mutate]);
  return { data };
};
