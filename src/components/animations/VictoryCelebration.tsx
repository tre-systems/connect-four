'use client';

import { motion } from 'framer-motion';
import { Player } from '@/lib/types';

interface VictoryCelebrationProps {
  position: { x: number; y: number };
  player: Player;
}

export default function VictoryCelebration({ position, player }: VictoryCelebrationProps) {
  const isPlayer1 = player === 'player1';
  const colors = isPlayer1 ? 'text-red-400' : 'text-yellow-400';

  return (
    <motion.div
      className="fixed pointer-events-none z-50"
      style={{ left: position.x, top: position.y }}
      initial={{ opacity: 1 }}
      animate={{ opacity: 0 }}
      exit={{ opacity: 0 }}
      transition={{ duration: 1.5 }}
    >
      <motion.div
        className={`absolute -translate-x-1/2 -translate-y-8 font-bold text-lg ${colors}`}
        initial={{ scale: 0, y: 0, opacity: 0 }}
        animate={{
          scale: [0, 1.1, 1, 0.8],
          y: [0, -15, -25, -35],
          opacity: [0, 1, 1, 0],
        }}
        transition={{ duration: 1.5, ease: 'easeOut' }}
      >
        {isPlayer1 ? 'WIN!' : 'AI WINS!'}
      </motion.div>
    </motion.div>
  );
}
