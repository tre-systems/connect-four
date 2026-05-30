import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { cn, getPlayerId, isProduction, isDevelopment } from '../utils';

describe('Utils', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    vi.stubGlobal('window', undefined);
    delete process.env.GITHUB_SHA;
  });

  afterEach(() => {
    vi.restoreAllMocks();
  });

  describe('cn', () => {
    it('should merge class names and handle conflicts', () => {
      expect(cn('class1', 'class2')).toBe('class1 class2');
      expect(cn('px-2 py-1', 'px-4')).toBe('py-1 px-4');
      expect(cn('bg-red-500', 'bg-blue-500')).toBe('bg-blue-500');
    });
  });

  describe('getPlayerId', () => {
    it('should return existing player ID from localStorage', () => {
      const existingId = 'player_1234567890_abc123';
      const localStorageMock = {
        getItem: vi.fn().mockReturnValue(existingId),
        setItem: vi.fn(),
      };
      vi.stubGlobal('localStorage', localStorageMock);
      vi.stubGlobal('window', { localStorage: localStorageMock });
      expect(getPlayerId()).toBe(existingId);
    });

    it('should generate a new player ID when none exists', () => {
      const localStorageMock = {
        getItem: vi.fn().mockReturnValue(null),
        setItem: vi.fn(),
      };
      vi.stubGlobal('localStorage', localStorageMock);
      vi.stubGlobal('window', { localStorage: localStorageMock });
      const result = getPlayerId();
      expect(result).toMatch(/^player_\d+_[a-z0-9]{9}$/);
      expect(localStorageMock.setItem).toHaveBeenCalledWith('connect-4-player-id', result);
    });

    it('should return "unknown" when window is undefined', () => {
      vi.stubGlobal('window', undefined);
      expect(getPlayerId()).toBe('unknown');
    });
  });

  describe('environment detection', () => {
    it('should detect production environment', () => {
      vi.stubGlobal('window', undefined);
      vi.stubEnv('NODE_ENV', 'production');
      expect(isProduction()).toBe(true);
      expect(isDevelopment()).toBe(false);
    });

    it('should detect development environment', () => {
      vi.stubGlobal('window', { location: { hostname: 'localhost' } });
      vi.stubEnv('NODE_ENV', 'development');
      expect(isProduction()).toBe(false);
      expect(isDevelopment()).toBe(true);
    });
  });
});
