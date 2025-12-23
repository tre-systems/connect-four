'use client';

import { motion, AnimatePresence } from 'framer-motion';
import { cn, isDevelopment } from '@/lib/utils';
import { Player } from '@/lib/types';
import GamePiece from './GamePiece';

interface GameSquareProps {
  column: number;
  row: number;
  player: Player | null;
  isClickable: boolean;
  onColumnClick: (column: number) => void;
}

export default function GameSquare({
  column,
  row,
  player,
  isClickable,
  onColumnClick,
  isWinning = false,
}: GameSquareProps & { isWinning?: boolean }) {
  const handleSquareClick = () => {
    if (isClickable) {
      onColumnClick(column);
    }
  };

  return (
    <motion.div
      className={cn(
        'aspect-square relative flex items-center justify-center overflow-hidden',
        'board-square rounded-lg',
        isClickable && 'clickable-square cursor-pointer'
      )}
      whileHover={{
        scale: isClickable ? 1.02 : 1,
      }}
      transition={{ type: 'spring', stiffness: 400, damping: 25 }}
      data-column={column}
      data-row={row}
      data-testid={`square-${column}-${row}`}
      onClick={handleSquareClick}
    >
      {isDevelopment() && (
        <span className="absolute top-1 left-1 text-xs text-white/60 font-mono select-none pointer-events-none z-10">
          {column},{row}
        </span>
      )}

      <AnimatePresence mode="wait">
        {player && (
          <motion.div
            key={`${player}-${column}-${row}`}
            className="w-4/5 h-4/5 p-0.5"
            layoutId={`piece-${player}-${column}-${row}`}
            data-testid={`piece-${column}-${row}`}
          >
            <GamePiece player={player} isClickable={false} isWinning={isWinning} />
          </motion.div>
        )}
      </AnimatePresence>

      {isClickable && (
        <motion.div
          className="absolute inset-0 rounded-lg border-2 border-green-400 pointer-events-none"
          animate={{
            boxShadow: ['0 0 0 0 rgba(34, 197, 94, 0.7)', '0 0 0 10px rgba(34, 197, 94, 0)'],
          }}
          transition={{ duration: 1.5, repeat: Infinity }}
        />
      )}

      {/* Drop indicator for empty squares in clickable columns */}
      {!player && isClickable && (
        <motion.div
          className="absolute inset-0 rounded-lg border-2 border-blue-400/30 pointer-events-none"
          animate={{
            opacity: [0.3, 0.7, 0.3],
          }}
          transition={{ duration: 2, repeat: Infinity }}
        />
      )}
    </motion.div>
  );
}
