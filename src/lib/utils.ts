import { type ClassValue, clsx } from 'clsx';
import { twMerge } from 'tailwind-merge';
import type { AIType } from './types';

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

const PLAYER_ID_KEY = 'connect-4-player-id';

export function getPlayerId(): string {
  if (typeof window === 'undefined') {
    return 'unknown';
  }

  let playerId = localStorage.getItem(PLAYER_ID_KEY);

  if (!playerId) {
    playerId = `player_${Date.now()}_${Math.random().toString(36).slice(2, 11)}`;
    localStorage.setItem(PLAYER_ID_KEY, playerId);
  }

  return playerId;
}

export const isProduction = () => {
  if (typeof window === 'undefined') {
    return process.env.NODE_ENV === 'production';
  }

  const hostname = window.location.hostname;
  return hostname === 'connect-4.tre.systems' || hostname === 'www.connect-4.tre.systems';
};

export const isDevelopment = () => {
  if (typeof window === 'undefined') {
    return process.env.NODE_ENV === 'development';
  }

  const hostname = window.location.hostname;
  return (
    hostname === 'localhost' || hostname === '127.0.0.1' || process.env.NODE_ENV === 'development'
  );
};

export function getAITypeLabel(aiType: AIType): string {
  switch (aiType) {
    case 'classic':
      return 'Classic AI';
    case 'ml':
      return 'ML AI';
    default:
      return 'AI';
  }
}
