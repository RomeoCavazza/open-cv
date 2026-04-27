import React, { createContext, useContext, useState, useEffect } from 'react';

export interface WatchItem {
  kind: 'hashtag' | 'user' | 'niche';
  value: string;
  created_at: string;
}

interface WatchlistContextType {
  items: WatchItem[];
  addItem: (item: WatchItem) => void;
  removeItem: (index: number) => void;
}

const WatchlistContext = createContext<WatchlistContextType | undefined>(undefined);

const STORAGE_KEY = 'insider_watchlist';

export function WatchlistProvider({ children }: { children: React.ReactNode }) {
  const [items, setItems] = useState<WatchItem[]>([]);

  useEffect(() => {
    const stored = localStorage.getItem(STORAGE_KEY);
    if (stored) {
      try {
        setItems(JSON.parse(stored));
      } catch (e) {
        console.error('Failed to load watchlist', e);
      }
    }
  }, []);

  useEffect(() => {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(items));
  }, [items]);

  const addItem = (item: WatchItem) => {
    setItems(prev => [...prev, { ...item, created_at: new Date().toISOString() }]);
  };

  const removeItem = (index: number) => {
    setItems(prev => prev.filter((_, i) => i !== index));
  };

  return (
    <WatchlistContext.Provider value={{ items, addItem, removeItem }}>
      {children}
    </WatchlistContext.Provider>
  );
}

export function useWatchlist() {
  const context = useContext(WatchlistContext);
  if (!context) {
    throw new Error('useWatchlist must be used within WatchlistProvider');
  }
  return context;
}
