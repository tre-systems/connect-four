'use client';

import { useState, useEffect } from 'react';
import { motion } from 'framer-motion';
import { ExternalLink, Github } from 'lucide-react';
import Image from 'next/image';
import { useGameStore, useGameState, useGameActions } from '@/lib/game-store';
import { soundEffects } from '@/lib/sound-effects';
import { useUIStore } from '@/lib/ui-store';
import GameBoard from './GameBoard';
import AnimatedBackground from './AnimatedBackground';
import HowToPlayPanel from './HowToPlayPanel';
import ErrorModal from './ErrorModal';
import AISelectionPanel from './AISelectionPanel';

function isStandalonePWA() {
  if (typeof window === 'undefined') return false;
  const nav = window.navigator as Navigator & { standalone?: boolean };
  return window.matchMedia('(display-mode: standalone)').matches || nav.standalone === true;
}

export default function ConnectFour() {
  const gameState = useGameState();
  const { makeAIMove, reset, startGame } = useGameActions();
  const aiThinking = useGameStore(state => state.aiThinking);
  const gameMode = useGameStore(state => state.gameMode);
  const { errorModal } = useUIStore();
  const { hideError } = useUIStore(state => state.actions);

  const [soundEnabled, setSoundEnabled] = useState(true);
  const [isStandalone, setIsStandalone] = useState(false);
  const [showHowToPlay, setShowHowToPlay] = useState(false);
  const [isMounted, setIsMounted] = useState(false);
  const [showAISelection, setShowAISelection] = useState(true);

  useEffect(() => {
    setIsStandalone(isStandalonePWA());
    setIsMounted(true);
  }, []);

  useEffect(() => {
    soundEffects.setEnabled(soundEnabled);
  }, [soundEnabled]);

  // Log initial game state
  useEffect(() => {
    // Game initialized
  }, []);

  // Handle AI moves
  useEffect(() => {
    const shouldMakeAIMove =
      gameState.gameStatus === 'playing' &&
      !aiThinking &&
      (gameMode === 'ai-vs-ai' ||
        (gameMode === 'human-vs-ai' && gameState.currentPlayer === 'player2'));

    if (shouldMakeAIMove) {
      const timer = setTimeout(() => {
        try {
          makeAIMove();
        } catch (error) {
          console.error('AI move failed:', error);
        }
      }, 300);
      return () => clearTimeout(timer);
    }
    return undefined;
  }, [gameState.gameStatus, gameState.currentPlayer, aiThinking, makeAIMove, gameMode]);

  // Handle game completion sounds
  useEffect(() => {
    if (gameState.gameStatus === 'finished' && gameState.winner) {
      setTimeout(() => {
        if (gameState.winner === 'player1') {
          soundEffects.gameWin();
        } else {
          soundEffects.gameLoss();
        }
      }, 500);
    }
  }, [gameState.gameStatus, gameState.winner]);

  const handleReset = () => {
    reset();
    setShowAISelection(true);
  };

  const toggleSound = () => {
    const newState = soundEffects.toggle();
    setSoundEnabled(newState);
  };

  const handleShowHowToPlay = () => {
    setShowHowToPlay(true);
  };

  const handleCloseHowToPlay = () => {
    setShowHowToPlay(false);
  };

  const handleStartGame = () => {
    startGame();
    setShowAISelection(false);
  };

  return (
    <>
      <div className="fixed bottom-5 left-1/2 transform -translate-x-1/2 z-50">
        <a
          href="https://ko-fi.com/N4N31DPNUS"
          target="_blank"
          rel="noopener noreferrer"
          aria-label="Buy me a coffee"
          className="opacity-60 hover:opacity-100 transition-opacity"
          data-testid="ko-fi-link"
        >
          <Image
            height={36}
            width={120}
            style={{ border: '0px', height: '36px' }}
            src="https://storage.ko-fi.com/cdn/kofi2.png?v=6"
            alt="Buy Me a Coffee at ko-fi.com"
            priority
          />
        </a>
      </div>
      <a
        href="https://github.com/rgilks/connect-four"
        target="_blank"
        rel="noopener noreferrer"
        aria-label="GitHub Repository"
        className="fixed bottom-5 right-4 z-50 opacity-60 hover:opacity-100 transition-opacity"
        data-testid="github-link"
      >
        <Github className="w-6 h-6" />
      </a>
      <AnimatedBackground />
      <div className="relative min-h-screen w-full flex items-center justify-center p-4 pb-24">
        {!isStandalone && (
          <div className="hidden md:block absolute top-4 right-4 z-50">
            <button
              onClick={() => {
                window.open(
                  '/',
                  'GamePopout',
                  'width=420,height=800,menubar=no,toolbar=no,location=no,status=no,resizable=yes,scrollbars=no'
                );
              }}
              className="glass-dark rounded-lg px-4 py-2 flex items-center space-x-2 text-white/80 hover:text-white font-semibold shadow-lg backdrop-blur-md border border-white/10 transition-colors"
              title="Pop Out Game"
            >
              <ExternalLink className="w-4 h-4 mr-1" />
              <span>Pop Out Game</span>
            </button>
          </div>
        )}

        {showAISelection ? (
          <div className="w-full">
            <motion.div
              className="text-center mb-4"
              initial={{ opacity: 0, y: -20 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{ duration: 0.5 }}
            >
              <h1 className="text-4xl font-bold text-white mb-2 title-glow">Connect 4</h1>
              <p className="text-gray-300 text-sm">Choose your AI opponent and start playing!</p>
            </motion.div>

            <AISelectionPanel onStartGame={handleStartGame} />
          </div>
        ) : (
          <div className="w-full max-w-md">
            <motion.div
              className="text-center mb-6"
              initial={{ opacity: 0, y: -20 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{ duration: 0.5 }}
            >
              <h1 className="text-4xl font-bold text-white mb-2 title-glow">Connect 4</h1>
              <p className="text-gray-300 text-sm">Drop your pieces to get four in a row!</p>
              {process.env.NODE_ENV === 'development' && isMounted && (
                <div className="text-xs text-gray-500 mt-2">
                  Status: {gameState.gameStatus} | Player: {gameState.currentPlayer} | AI Thinking:{' '}
                  {aiThinking ? 'Yes' : 'No'}
                </div>
              )}
            </motion.div>

            <GameBoard
              gameState={gameState}
              aiThinking={aiThinking}
              onResetGame={handleReset}
              soundEnabled={soundEnabled}
              onToggleSound={toggleSound}
              onShowHowToPlay={handleShowHowToPlay}
              watchMode={gameMode === 'ai-vs-ai'}
              gameMode={gameMode}
              data-testid="game-board-component"
            />
          </div>
        )}
      </div>

      <HowToPlayPanel isOpen={showHowToPlay} onClose={handleCloseHowToPlay} />
      <ErrorModal isOpen={errorModal.isOpen} onClose={hideError} error={errorModal.error} />
    </>
  );
}
