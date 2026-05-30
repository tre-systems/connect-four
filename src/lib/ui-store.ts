import { create } from 'zustand';

type UIStore = {
  showModelOverlay: boolean;
  soundEnabled: boolean;
  diagnosticsPanelOpen: boolean;
  howToPlayOpen: boolean;
  errorModal: {
    isOpen: boolean;
    error: string;
  };
  actions: {
    setShowModelOverlay: (show: boolean) => void;
    setSoundEnabled: (enabled: boolean) => void;
    setDiagnosticsPanelOpen: (open: boolean) => void;
    setHowToPlayOpen: (open: boolean) => void;
    showError: (error: string) => void;
    hideError: () => void;
    reset: () => void;
  };
};

export const useUIStore = create<UIStore>(set => ({
  showModelOverlay: true,
  soundEnabled: true,
  diagnosticsPanelOpen: false,
  howToPlayOpen: false,
  errorModal: {
    isOpen: false,
    error: '',
  },
  actions: {
    setShowModelOverlay: show => set({ showModelOverlay: show }),
    setSoundEnabled: enabled => set({ soundEnabled: enabled }),
    setDiagnosticsPanelOpen: open => set({ diagnosticsPanelOpen: open }),
    setHowToPlayOpen: open => set({ howToPlayOpen: open }),
    showError: error => set({ errorModal: { isOpen: true, error } }),
    hideError: () => set({ errorModal: { isOpen: false, error: '' } }),
    reset: () =>
      set({
        showModelOverlay: true,
        soundEnabled: true,
        diagnosticsPanelOpen: false,
        howToPlayOpen: false,
        errorModal: { isOpen: false, error: '' },
      }),
  },
}));

export const useUIState = () =>
  useUIStore(state => ({
    showModelOverlay: state.showModelOverlay,
    soundEnabled: state.soundEnabled,
    diagnosticsPanelOpen: state.diagnosticsPanelOpen,
    howToPlayOpen: state.howToPlayOpen,
    errorModal: state.errorModal,
  }));
