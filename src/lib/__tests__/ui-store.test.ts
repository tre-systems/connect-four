import { describe, it, expect, beforeEach } from 'vitest';
import { useUIStore } from '../ui-store';

describe('UI Store', () => {
  beforeEach(() => {
    useUIStore.getState().actions.reset();
  });

  describe('initial state', () => {
    it('should have correct initial values', () => {
      const state = useUIStore.getState();

      expect(state.showModelOverlay).toBe(true);
      expect(state.soundEnabled).toBe(true);
      expect(state.diagnosticsPanelOpen).toBe(false);
      expect(state.howToPlayOpen).toBe(false);
      expect(state.errorModal.isOpen).toBe(false);
      expect(state.errorModal.error).toBe('');
    });
  });

  describe('setShowModelOverlay', () => {
    it('should update showModelOverlay state', () => {
      const { setShowModelOverlay } = useUIStore.getState().actions;

      setShowModelOverlay(false);
      expect(useUIStore.getState().showModelOverlay).toBe(false);

      setShowModelOverlay(true);
      expect(useUIStore.getState().showModelOverlay).toBe(true);
    });
  });

  describe('setSoundEnabled', () => {
    it('should update soundEnabled state', () => {
      const { setSoundEnabled } = useUIStore.getState().actions;

      setSoundEnabled(false);
      expect(useUIStore.getState().soundEnabled).toBe(false);

      setSoundEnabled(true);
      expect(useUIStore.getState().soundEnabled).toBe(true);
    });
  });

  describe('setDiagnosticsPanelOpen', () => {
    it('should update diagnosticsPanelOpen state', () => {
      const { setDiagnosticsPanelOpen } = useUIStore.getState().actions;

      setDiagnosticsPanelOpen(true);
      expect(useUIStore.getState().diagnosticsPanelOpen).toBe(true);

      setDiagnosticsPanelOpen(false);
      expect(useUIStore.getState().diagnosticsPanelOpen).toBe(false);
    });
  });

  describe('setHowToPlayOpen', () => {
    it('should update howToPlayOpen state', () => {
      const { setHowToPlayOpen } = useUIStore.getState().actions;

      setHowToPlayOpen(true);
      expect(useUIStore.getState().howToPlayOpen).toBe(true);

      setHowToPlayOpen(false);
      expect(useUIStore.getState().howToPlayOpen).toBe(false);
    });
  });

  describe('error modal', () => {
    it('should show error modal', () => {
      const { showError } = useUIStore.getState().actions;
      const testError = 'Test error message';

      showError(testError);
      const state = useUIStore.getState();
      expect(state.errorModal.isOpen).toBe(true);
      expect(state.errorModal.error).toBe(testError);
    });

    it('should hide error modal', () => {
      const { showError, hideError } = useUIStore.getState().actions;
      const testError = 'Test error message';

      showError(testError);
      expect(useUIStore.getState().errorModal.isOpen).toBe(true);

      hideError();
      const state = useUIStore.getState();
      expect(state.errorModal.isOpen).toBe(false);
      expect(state.errorModal.error).toBe('');
    });

    it('should update error message when showing new error', () => {
      const { showError } = useUIStore.getState().actions;
      const firstError = 'First error message';
      const secondError = 'Second error message';

      showError(firstError);
      expect(useUIStore.getState().errorModal.error).toBe(firstError);

      showError(secondError);
      expect(useUIStore.getState().errorModal.error).toBe(secondError);
      expect(useUIStore.getState().errorModal.isOpen).toBe(true);
    });
  });

  describe('reset', () => {
    it('should reset all state to initial values', () => {
      const { actions } = useUIStore.getState();

      actions.setShowModelOverlay(false);
      actions.setSoundEnabled(false);
      actions.setDiagnosticsPanelOpen(true);
      actions.setHowToPlayOpen(true);
      actions.showError('Test error');

      const stateBeforeReset = useUIStore.getState();
      expect(stateBeforeReset.showModelOverlay).toBe(false);
      expect(stateBeforeReset.soundEnabled).toBe(false);
      expect(stateBeforeReset.diagnosticsPanelOpen).toBe(true);
      expect(stateBeforeReset.howToPlayOpen).toBe(true);
      expect(stateBeforeReset.errorModal.isOpen).toBe(true);
      expect(stateBeforeReset.errorModal.error).toBe('Test error');

      actions.reset();

      const stateAfterReset = useUIStore.getState();
      expect(stateAfterReset.showModelOverlay).toBe(true);
      expect(stateAfterReset.soundEnabled).toBe(true);
      expect(stateAfterReset.diagnosticsPanelOpen).toBe(false);
      expect(stateAfterReset.howToPlayOpen).toBe(false);
      expect(stateAfterReset.errorModal.isOpen).toBe(false);
      expect(stateAfterReset.errorModal.error).toBe('');
    });
  });

  describe('useUIState hook', () => {
    it('should return the non-action state values', () => {
      const state = useUIStore.getState();
      const selectedState = {
        showModelOverlay: state.showModelOverlay,
        soundEnabled: state.soundEnabled,
        diagnosticsPanelOpen: state.diagnosticsPanelOpen,
        howToPlayOpen: state.howToPlayOpen,
      };

      expect(selectedState.showModelOverlay).toBe(true);
      expect(selectedState.soundEnabled).toBe(true);
      expect(selectedState.diagnosticsPanelOpen).toBe(false);
      expect(selectedState.howToPlayOpen).toBe(false);
      expect(selectedState).not.toHaveProperty('actions');
    });
  });

  describe('state persistence', () => {
    it('should maintain state across multiple actions', () => {
      const { actions } = useUIStore.getState();

      actions.setSoundEnabled(false);
      actions.setDiagnosticsPanelOpen(true);

      const state = useUIStore.getState();
      expect(state.soundEnabled).toBe(false);
      expect(state.diagnosticsPanelOpen).toBe(true);
      expect(state.showModelOverlay).toBe(true);
      expect(state.howToPlayOpen).toBe(false);
    });
  });
});
