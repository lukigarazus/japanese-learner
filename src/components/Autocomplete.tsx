import React, { useState, useEffect, useRef } from "react";

type AutocompleteProps<T> = {
  fetchData: (query: string) => Promise<T[]>;
  renderItem: (item: T, query: string) => React.ReactNode;
  onSelect: (item: T) => void;
  placeholder?: string;
  debounceMs?: number;
  value?: string;
  validateInput?: (query: string) => void | string; // <-- validation prop
};

export function Autocomplete<T>({
  fetchData,
  renderItem,
  onSelect,
  placeholder = "Search...",
  debounceMs = 300,
  value,
  validateInput,
}: AutocompleteProps<T>) {
  const [query, setQuery] = useState(value ?? "");
  const [results, setResults] = useState<T[]>([]);
  const [loading, setLoading] = useState(false);
  const [open, setOpen] = useState(false);
  const [highlightedIndex, setHighlightedIndex] = useState<number>(-1);
  const [error, setError] = useState<string | null>(null);
  const debounceRef = useRef<number | null>(null);

  useEffect(() => {
    if (value !== undefined && value !== query) {
      setQuery(value);
    }
  }, [value]);

  useEffect(() => {
    // Validate input if validator is provided
    if (validateInput) {
      const validationResult = validateInput(query);
      if (typeof validationResult === "string") {
        setError(validationResult);
        setResults([]);
        setLoading(false);
        return;
      } else {
        setError(null);
      }
    }

    if (!query.trim()) {
      setResults([]);
      setError(null);
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
  }, [query, fetchData, debounceMs, validateInput]);

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

  useEffect(() => {
    if (highlightedIndex >= 0) {
      const el = document.getElementById("suggestion-item-" + highlightedIndex);
      const container = document.getElementById("suggestion-container");
      if (el && container) {
        const elTop = el.offsetTop;
        const elBottom = elTop + el.offsetHeight;
        const containerTop = container.scrollTop;
        const containerBottom = containerTop + container.offsetHeight;

        if (elTop < containerTop) {
          container.scrollTop = elTop;
        } else if (elBottom > containerBottom) {
          container.scrollTop = elBottom - container.offsetHeight;
        }
      }
    }
  }, [highlightedIndex]);
  return (
    <div className="relative w-full">
      <input
        type="text"
        value={query}
        placeholder={placeholder}
        className={`w-full border p-2 rounded ${error ? "border-red-400" : ""}`}
        onChange={(e) => {
          setQuery(e.target.value);
          setOpen(true);
        }}
        onFocus={() => setOpen(true)}
        onKeyDown={handleKeyDown}
        aria-invalid={!!error}
      />

      {error && <div className="text-red-500 text-sm mt-1">{error}</div>}

      {open && !error && (
        <div
          id="suggestion-container"
          className="absolute z-10 w-full bg-white border mt-1 rounded shadow max-h-60 overflow-auto"
        >
          {loading && <div className="p-2 text-gray-500">Loading...</div>}
          {!loading && results.length === 0 && query && (
            <div className="p-2 text-gray-500">No results</div>
          )}
          {!loading &&
            results.map((item, idx) => (
              <div
                id={"suggestion-item-" + idx}
                key={idx}
                onClick={() => handleSelect(item)}
                className={`p-2 cursor-pointer ${
                  idx === highlightedIndex ? "bg-blue-100" : "hover:bg-gray-100"
                }`}
              >
                {renderItem(item, query)}
              </div>
            ))}
        </div>
      )}
    </div>
  );
}
