import { describe, it, expect, beforeEach } from 'vitest';
import { useUIStore } from '../ui-store';

describe('UI Store', () => {
  beforeEach(() => {
    useUIStore.getState().actions.hideError();
  });

  it('starts with the error modal closed', () => {
    const { errorModal } = useUIStore.getState();
    expect(errorModal.isOpen).toBe(false);
    expect(errorModal.error).toBe('');
  });

  it('shows an error modal', () => {
    useUIStore.getState().actions.showError('Test error message');
    const { errorModal } = useUIStore.getState();
    expect(errorModal.isOpen).toBe(true);
    expect(errorModal.error).toBe('Test error message');
  });

  it('hides the error modal', () => {
    const { showError, hideError } = useUIStore.getState().actions;
    showError('Test error message');
    expect(useUIStore.getState().errorModal.isOpen).toBe(true);

    hideError();
    const { errorModal } = useUIStore.getState();
    expect(errorModal.isOpen).toBe(false);
    expect(errorModal.error).toBe('');
  });

  it('replaces the message when a new error is shown', () => {
    const { showError } = useUIStore.getState().actions;
    showError('First error message');
    expect(useUIStore.getState().errorModal.error).toBe('First error message');

    showError('Second error message');
    expect(useUIStore.getState().errorModal.error).toBe('Second error message');
    expect(useUIStore.getState().errorModal.isOpen).toBe(true);
  });
});
