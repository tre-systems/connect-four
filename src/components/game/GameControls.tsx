'use client';

import { motion } from 'framer-motion';
import { Volume2, VolumeX, HelpCircle, RotateCcw } from 'lucide-react';
import { isDevelopment } from '@/lib/utils';

interface GameControlsProps {
  soundEnabled: boolean;
  onToggleSound: () => void;
  onShowHowToPlay: () => void;
  onResetGame: () => void;
}

export default function GameControls({
  soundEnabled,
  onToggleSound,
  onShowHowToPlay,
  onResetGame,
}: GameControlsProps) {
  return (
    <>
      <hr className="my-4 border-white/10" />
      <div className="flex items-center justify-between w-full">
        {/* Dev-only controls on the left */}
        {isDevelopment() && (
          <div className="flex items-center space-x-2">
            <motion.button
              onClick={onResetGame}
              className="p-3 glass-dark rounded-lg text-white/70 hover:text-white transition-colors button-glow"
              whileHover={{ scale: 1.05 }}
              whileTap={{ scale: 0.95 }}
              title="Reset Game"
              data-testid="reset-game"
            >
              <RotateCcw className="w-5 h-5" />
            </motion.button>
          </div>
        )}

        {/* Main controls on the right */}
        <div className="flex items-center space-x-3">
          <motion.button
            onClick={onToggleSound}
            className="p-3 glass-dark rounded-lg text-white/70 hover:text-white transition-colors button-glow"
            whileHover={{ scale: 1.05 }}
            whileTap={{ scale: 0.95 }}
            title={soundEnabled ? 'Mute Sound' : 'Unmute Sound'}
            data-testid="toggle-sound"
          >
            {soundEnabled ? <Volume2 className="w-5 h-5" /> : <VolumeX className="w-5 h-5" />}
          </motion.button>

          <motion.button
            onClick={onShowHowToPlay}
            className="p-3 glass-dark rounded-lg text-white/70 hover:text-white transition-colors button-glow"
            whileHover={{ scale: 1.05 }}
            whileTap={{ scale: 0.95 }}
            title="How to Play"
            data-testid="how-to-play"
          >
            <HelpCircle className="w-5 h-5" />
          </motion.button>
        </div>
      </div>
    </>
  );
}
