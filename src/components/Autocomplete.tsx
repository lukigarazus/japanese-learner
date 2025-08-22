import React, { useState, useEffect, useRef } from "react";

type AutocompleteProps<T> = {
  fetchData: (query: string) => Promise<T[]>;
  renderItem: (item: T) => React.ReactNode;
  onSelect: (item: T) => void;
  placeholder?: string;
  debounceMs?: number;
  value?: string;
};

export function Autocomplete<T>({
  fetchData,
  renderItem,
  onSelect,
  placeholder = "Search...",
  debounceMs = 300,
  value,
}: AutocompleteProps<T>) {
  const [query, setQuery] = useState(value ?? "");
  const [results, setResults] = useState<T[]>([]);
  const [loading, setLoading] = useState(false);
  const [open, setOpen] = useState(false);
  const [highlightedIndex, setHighlightedIndex] = useState<number>(-1);
  const debounceRef = useRef<number | null>(null);

  useEffect(() => {
    if (value !== undefined && value !== query) {
      setQuery(value);
    }
  }, [value]);

  useEffect(() => {
    if (!query.trim()) {
      setResults([]);
      return;
    }

    setLoading(true);
    if (debounceRef.current) clearTimeout(debounceRef.current);

    debounceRef.current = setTimeout(async () => {
      try {
        const data = await fetchData(query);
        setResults(data);
      } catch (err) {
        console.error("Fetch error:", err);
      } finally {
        setLoading(false);
      }
    }, debounceMs);
  }, [query, fetchData, debounceMs]);

  const handleSelect = (item: T) => {
    onSelect(item);
    setOpen(false);
  };

  const handleKeyDown = (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (!open || results.length === 0 || e.nativeEvent.isComposing) return;

    switch (e.key) {
      case "ArrowDown":
        e.preventDefault();
        setHighlightedIndex((prev) => (prev + 1) % results.length);
        break;
      case "ArrowUp":
        e.preventDefault();
        setHighlightedIndex(
          (prev) => (prev - 1 + results.length) % results.length
        );
        break;
      case "Enter":
        e.preventDefault();
        if (highlightedIndex >= 0) {
          handleSelect(results[highlightedIndex]);
        }
        break;
      case "Escape":
        setOpen(false);
        break;
      default:
        break;
    }
  };

  return (
    <div className="relative w-full">
      <input
        type="text"
        value={query}
        placeholder={placeholder}
        className="w-full border p-2 rounded"
        onChange={(e) => {
          setQuery(e.target.value);
          setOpen(true);
        }}
        onFocus={() => setOpen(true)}
        onKeyDown={handleKeyDown}
      />

      {open && (
        <div className="absolute z-10 w-full bg-white border mt-1 rounded shadow max-h-60 overflow-auto">
          {loading && <div className="p-2 text-gray-500">Loading...</div>}
          {!loading && results.length === 0 && query && (
            <div className="p-2 text-gray-500">No results</div>
          )}
          {!loading &&
            results.map((item, idx) => (
              <div
                key={idx}
                onClick={() => handleSelect(item)}
                className={`p-2 cursor-pointer ${
                  idx === highlightedIndex ? "bg-blue-100" : "hover:bg-gray-100"
                }`}
              >
                {renderItem(item)}
              </div>
            ))}
        </div>
      )}
    </div>
  );
}
