'use client';

import { motion } from 'framer-motion';

export default function ConnectFourWin({ onComplete }: { onComplete?: () => void }) {
  return (
    <motion.div
      initial={{ opacity: 0 }}
      animate={{ opacity: 0 }}
      transition={{ delay: 1.5, duration: 0.5 }}
      onAnimationComplete={onComplete}
    />
  );
}
