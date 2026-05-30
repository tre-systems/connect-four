'use client';

import { memo } from 'react';
import { motion } from 'framer-motion';

interface GamePieceProps {
  player: 'player1' | 'player2';
  isClickable?: boolean;
  isWinning?: boolean;
}

const GamePiece = memo(function GamePiece({
  player,
  isClickable = false,
  isWinning = false,
}: GamePieceProps) {
  const isPlayer1 = player === 'player1';
  const colors = isPlayer1
    ? {
        bg: 'bg-red-500',
        border: 'border-red-400',
        shadow: 'shadow-red-400/40',
        glow: 'rgba(239,68,68,0.8)',
        pulse: 'rgba(239,68,68,0.5)',
        highlight: 'from-red-400/40 to-transparent',
      }
    : {
        bg: 'bg-yellow-400',
        border: 'border-yellow-300',
        shadow: 'shadow-yellow-300/40',
        glow: 'rgba(234,179,8,0.8)',
        pulse: 'rgba(234,179,8,0.5)',
        highlight: 'from-yellow-300/40 to-transparent',
      };

  return (
    <motion.div
      className={`w-full h-full rounded-full border-2 relative overflow-hidden ${
        isClickable ? 'cursor-pointer' : 'cursor-default'
      } ${colors.bg} ${colors.border} ${colors.shadow}`}
      whileHover={isClickable ? { scale: 1.1, boxShadow: `0 0 20px ${colors.glow}` } : {}}
      whileTap={isClickable ? { scale: 0.95 } : {}}
      animate={
        isWinning
          ? {
              scale: [1, 1.15, 1],
              boxShadow: [
                `0 0 32px 12px ${colors.glow}, 0 0 64px 24px ${colors.glow}`,
                `0 0 48px 18px ${colors.glow}, 0 0 96px 36px ${colors.glow}`,
                `0 0 32px 12px ${colors.glow}, 0 0 64px 24px ${colors.glow}`,
              ],
            }
          : {
              scale: 1,
              boxShadow: [
                `0 0 8px ${colors.pulse}`,
                `0 0 16px ${colors.pulse}`,
                `0 0 8px ${colors.pulse}`,
              ],
            }
      }
      transition={{
        type: 'spring',
        stiffness: 400,
        damping: 25,
        scale: {
          duration: isWinning ? 1.2 : 0.2,
          repeat: isWinning ? Infinity : 0,
          repeatType: 'loop',
          ease: 'easeInOut',
        },
        boxShadow: {
          duration: isWinning ? 1.2 : 2,
          repeat: isWinning ? Infinity : 0,
          repeatType: 'loop',
          ease: 'easeInOut',
        },
      }}
      data-testid={`game-piece-${player}-${isClickable ? 'clickable' : 'static'}`}
    >
      <div className={`absolute inset-0 bg-gradient-to-br ${colors.highlight}`} />
      <div className="absolute inset-0 bg-gradient-to-tl from-black/30 to-transparent" />
      <div className="absolute inset-0 bg-gradient-to-tr from-white/20 to-transparent" />

      <motion.div
        className="absolute inset-0 rounded-full"
        animate={{
          boxShadow: [
            'inset 0 0 8px rgba(255, 255, 255, 0.3)',
            'inset 0 0 16px rgba(255, 255, 255, 0.1)',
            'inset 0 0 8px rgba(255, 255, 255, 0.3)',
          ],
        }}
        transition={{
          duration: 3,
          repeat: Infinity,
          ease: 'easeInOut',
        }}
      />

      {isClickable && (
        <motion.div
          className="absolute inset-0 rounded-full border-2 border-white/60 pointer-events-none"
          animate={{
            boxShadow: ['0 0 0 0 rgba(255, 255, 255, 0.8)', '0 0 0 12px rgba(255, 255, 255, 0)'],
            scale: [1, 1.05, 1],
          }}
          transition={{
            duration: 1.5,
            repeat: Infinity,
            ease: 'easeInOut',
          }}
        />
      )}

      <div className="absolute inset-0 flex items-center justify-center">
        <motion.div
          className={`w-1/3 h-1/3 rounded-full ${isPlayer1 ? 'bg-red-300' : 'bg-yellow-300'} shadow-inner`}
          animate={{
            boxShadow: [
              'inset 0 2px 4px rgba(0, 0, 0, 0.3)',
              'inset 0 1px 2px rgba(0, 0, 0, 0.2)',
              'inset 0 2px 4px rgba(0, 0, 0, 0.3)',
            ],
          }}
          transition={{
            duration: 2,
            repeat: Infinity,
            ease: 'easeInOut',
          }}
        />
      </div>

      <motion.div
        className="absolute top-1/4 left-1/4 w-1 h-1 bg-white rounded-full"
        animate={{
          opacity: [0, 1, 0],
          scale: [0, 1, 0],
          x: [0, 2, 0],
          y: [0, -2, 0],
        }}
        transition={{
          duration: 2,
          repeat: Infinity,
          delay: 0.5,
          ease: 'easeInOut',
        }}
      />
    </motion.div>
  );
});

export default GamePiece;
