import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { commands } from "../bindings";

export const useWords = () => {
  return useQuery({
    queryKey: ["words"],
    queryFn: () => commands.getWords(),
  });
};

export const useAddWord = () => {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: commands.addWord,
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["words"] });
    },
  });
};

export const useKanjis = () => {
  return useQuery({
    queryKey: ["kanjis"],
    queryFn: () => commands.getKanjis(),
  });
};

export const useAddKanji = () => {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: commands.addKanji,
    onSuccess: async () => {
      await queryClient.invalidateQueries({ queryKey: ["kanjis"] });
    },
  });
};
