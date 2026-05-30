import { create } from 'zustand';

type UIStore = {
  errorModal: {
    isOpen: boolean;
    error: string;
  };
  actions: {
    showError: (error: string) => void;
    hideError: () => void;
  };
};

export const useUIStore = create<UIStore>(set => ({
  errorModal: {
    isOpen: false,
    error: '',
  },
  actions: {
    showError: error => set({ errorModal: { isOpen: true, error } }),
    hideError: () => set({ errorModal: { isOpen: false, error: '' } }),
  },
}));
