'use client';

import { motion } from 'framer-motion';
import { Trophy, Zap, Brain, Cpu } from 'lucide-react';
import { cn } from '@/lib/utils';
import { GameState } from '@/lib/types';
import { useGameStore } from '@/lib/game-store';

interface GameCompletionOverlayProps {
  gameState: GameState;
  onResetGame: () => void;
  gameMode?: 'human-vs-human' | 'human-vs-ai' | 'ai-vs-ai';
}

export default function GameCompletionOverlay({
  gameState,
  onResetGame,
  gameMode = 'human-vs-ai',
}: GameCompletionOverlayProps) {
  const { player1AI, player2AI } = useGameStore();
  const isPlayer1Winner = gameState.winner === 'player1';
  const isDraw = gameState.gameStatus === 'finished' && !gameState.winner;

  const getAITypeLabel = (aiType: string) => {
    switch (aiType) {
      case 'classic':
        return 'Classic AI';
      case 'ml':
        return 'ML AI';
      default:
        return 'AI';
    }
  };

  const getTitle = () => {
    if (isDraw) return 'Draw!';
    if (gameMode === 'ai-vs-ai') {
      const aiLabel = getAITypeLabel(gameState.winner === 'player1' ? player1AI : player2AI);
      return isPlayer1Winner ? `${aiLabel} (Red) Wins!` : `${aiLabel} (Yellow) Wins!`;
    }
    return isPlayer1Winner ? 'You Win!' : 'AI Wins!';
  };

  const getMessage = () => {
    if (isDraw) return "The board is full. It's a tie!";
    if (gameMode === 'ai-vs-ai') {
      return '🎉 AI Battle Complete! 🎉';
    }
    return isPlayer1Winner ? '🎉 Congratulations! 🎉' : '💫 The AI won this round! 💫';
  };

  const getIcon = () => {
    if (isDraw) return null;
    if (gameMode === 'ai-vs-ai') {
      const aiType = gameState.winner === 'player1' ? player1AI : player2AI;
      return aiType === 'ml' ? Brain : Cpu;
    }
    return isPlayer1Winner ? Trophy : Zap;
  };

  return (
    <motion.div
      className="absolute inset-0 bg-black/50 backdrop-blur-sm flex items-center justify-center z-50"
      initial={{ opacity: 0 }}
      animate={{ opacity: 1 }}
      exit={{ opacity: 0 }}
      data-testid="game-completion-overlay"
    >
      <motion.div
        className="glass rounded-lg p-8 text-center shadow-2xl max-w-sm mx-4 relative overflow-hidden"
        initial={{ scale: 0.5, opacity: 0 }}
        animate={{ scale: 1, opacity: 1 }}
        exit={{ scale: 0.5, opacity: 0 }}
        transition={{
          type: 'spring',
          stiffness: 260,
          damping: 20,
        }}
      >
        <motion.div
          className="text-center relative z-10"
          initial={{ scale: 0 }}
          animate={{ scale: 1 }}
          transition={{
            type: 'spring',
            stiffness: 200,
            damping: 15,
            delay: 0.2,
          }}
        >
          {!isDraw && getIcon() && (
            <motion.div
              animate={{
                rotate: [0, 10, -10, 0],
                scale: [1, 1.1, 1],
              }}
              transition={{
                repeat: Infinity,
                duration: 2,
                ease: 'easeInOut',
              }}
            >
              <motion.div
                initial={{ scale: 0, rotate: -180 }}
                animate={{ scale: 1, rotate: 0 }}
                transition={{
                  type: 'spring',
                  stiffness: 200,
                  damping: 15,
                  delay: 0.5,
                }}
              >
                {(() => {
                  const Icon = getIcon();
                  if (!Icon) return null;
                  return (
                    <Icon
                      className={cn(
                        'w-20 h-20 mx-auto mb-4 drop-shadow-lg',
                        gameMode === 'ai-vs-ai'
                          ? isPlayer1Winner
                            ? 'text-red-400'
                            : 'text-yellow-400'
                          : isPlayer1Winner
                            ? 'text-green-400'
                            : 'text-pink-400'
                      )}
                    />
                  );
                })()}
              </motion.div>
            </motion.div>
          )}

          <motion.h2
            className={cn(
              'text-4xl font-bold neon-text mb-6',
              isDraw
                ? 'text-gray-400'
                : gameMode === 'ai-vs-ai'
                  ? isPlayer1Winner
                    ? 'text-red-400'
                    : 'text-yellow-400'
                  : isPlayer1Winner
                    ? 'text-green-400'
                    : 'text-pink-400'
            )}
            initial={{ y: 50, opacity: 0 }}
            animate={{ y: 0, opacity: 1 }}
            transition={{
              type: 'spring',
              stiffness: 200,
              damping: 15,
              delay: 0.8,
            }}
            data-testid="game-completion-title"
          >
            {getTitle()}
          </motion.h2>

          <motion.div
            className="text-white/80 mb-6"
            initial={{ y: 30, opacity: 0 }}
            animate={{ y: 0, opacity: 1 }}
            transition={{
              type: 'spring',
              stiffness: 200,
              damping: 15,
              delay: 1.0,
            }}
          >
            <p className="text-lg mb-3" data-testid="game-completion-message">
              {getMessage()}
            </p>
            {gameMode === 'ai-vs-ai' && !isDraw && (
              <p className="text-sm text-gray-300">
                {getAITypeLabel(player1AI)} vs {getAITypeLabel(player2AI)}
              </p>
            )}
          </motion.div>

          <motion.button
            onClick={onResetGame}
            className="px-8 py-4 bg-gradient-to-r from-blue-600 to-purple-600 text-white rounded-lg font-bold hover:from-blue-700 hover:to-purple-700 transition-all duration-200 shadow-lg relative overflow-hidden group"
            whileHover={{
              scale: 1.05,
              boxShadow: '0 0 30px rgba(59, 130, 246, 0.4)',
            }}
            whileTap={{ scale: 0.95 }}
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ delay: 1.2 }}
            data-testid="reset-game-button"
          >
            <motion.div
              className="absolute inset-0 bg-gradient-to-r from-transparent via-white/20 to-transparent"
              initial={{ x: '-100%' }}
              animate={{ x: '100%' }}
              transition={{
                duration: 1.5,
                repeat: Infinity,
                delay: 1.5,
              }}
            />
            <span className="relative z-10">Play Again</span>
          </motion.button>
        </motion.div>
      </motion.div>
    </motion.div>
  );
}
