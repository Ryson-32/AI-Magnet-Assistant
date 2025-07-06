import { Store } from "tauri-plugin-store-api";

// Create a singleton store instance
let storeInstance: Store | null = null;

export function useStore() {
  if (!storeInstance) {
    storeInstance = new Store("settings.json");
  }

  const isStoreAvailable = () => {
    return typeof window !== 'undefined' && (window as any).__TAURI__;
  };

  const saveToStore = async (key: string, value: any): Promise<void> => {
    if (isStoreAvailable() && storeInstance) {
      try {
        await storeInstance.set(key, value);
        await storeInstance.save();
        return;
      } catch (error) {
        console.warn("Tauri store failed, falling back to localStorage:", error);
      }
    }
    
    // Fallback to localStorage
    localStorage.setItem(key, JSON.stringify(value));
  };

  const loadFromStore = async (key: string): Promise<any> => {
    if (isStoreAvailable() && storeInstance) {
      try {
        const value = await storeInstance.get(key);
        if (value !== null) {
          return value;
        }
      } catch (error) {
        console.warn("Tauri store failed, falling back to localStorage:", error);
      }
    }
    
    // Fallback to localStorage
    const savedValue = localStorage.getItem(key);
    return savedValue ? JSON.parse(savedValue) : null;
  };

  return {
    saveToStore,
    loadFromStore,
    isStoreAvailable
  };
}
