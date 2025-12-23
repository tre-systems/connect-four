'use client';

import { motion } from 'framer-motion';
import { Player } from '@/lib/types';
import GamePiece from '../game/GamePiece';

interface PieceDropProps {
  column: number;
  row: number;
  player: Player;
  onComplete?: () => void;
}

export default function PieceDrop({ column, row, player, onComplete }: PieceDropProps) {
  return (
    <motion.div
      className="absolute w-4/5 h-4/5 z-20"
      style={{
        left: `${(column / 7) * 100}%`,
        top: `${(row / 6) * 100}%`,
      }}
      initial={{ y: -300, scale: 0.8, opacity: 0.8 }}
      animate={{ y: 0, scale: 1, opacity: 1 }}
      transition={{
        type: 'spring',
        stiffness: 400,
        damping: 25,
        duration: 0.8,
      }}
      onAnimationComplete={onComplete}
    >
      <GamePiece player={player} isClickable={false} />
    </motion.div>
  );
}
