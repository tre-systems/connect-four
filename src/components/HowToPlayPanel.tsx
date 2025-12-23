'use client';

import { motion, AnimatePresence } from 'framer-motion';
import { X, Crown, Star, Zap, Trophy, ArrowRight, Circle } from 'lucide-react';

interface HowToPlayPanelProps {
  isOpen: boolean;
  onClose: () => void;
}

export default function HowToPlayPanel({ isOpen, onClose }: HowToPlayPanelProps) {
  return (
    <AnimatePresence>
      {isOpen && (
        <motion.div
          className="fixed inset-0 bg-black/60 backdrop-blur-sm flex items-center justify-center z-50 p-4"
          initial={{ opacity: 0 }}
          animate={{ opacity: 1 }}
          exit={{ opacity: 0 }}
          onClick={onClose}
          data-testid="help-panel"
        >
          <motion.div
            className="glass mystical-glow rounded-xl p-6 max-w-md w-full max-h-[90vh] overflow-y-auto"
            initial={{ scale: 0.9, opacity: 0, y: 20 }}
            animate={{ scale: 1, opacity: 1, y: 0 }}
            exit={{ scale: 0.9, opacity: 0, y: 20 }}
            onClick={e => e.stopPropagation()}
          >
            <div className="flex items-center justify-between mb-6">
              <h2 className="text-xl font-bold text-white neon-text">How to Play Connect 4</h2>
              <motion.button
                onClick={onClose}
                className="p-1.5 glass-dark rounded-lg text-white/70 hover:text-white transition-colors"
                whileHover={{ scale: 1.05 }}
                whileTap={{ scale: 0.95 }}
                data-testid="help-close"
              >
                <X className="w-4 h-4" />
              </motion.button>
            </div>

            <div className="space-y-6 text-white/90">
              <div>
                <h3 className="text-lg font-semibold text-white mb-3 flex items-center">
                  <Crown className="w-5 h-5 mr-2 text-amber-400" />
                  Objective
                </h3>
                <p className="text-sm leading-relaxed">
                  Be the first player to connect 4 of your colored checkers in a row, either
                  horizontally, vertically, or diagonally!
                </p>
              </div>

              <div>
                <h3 className="text-lg font-semibold text-white mb-3 flex items-center">
                  <Circle className="w-5 h-5 mr-2 text-red-400" />
                  Game Pieces
                </h3>
                <div className="text-sm leading-relaxed mb-2">
                  <p className="mb-2">Each player has their own colored pieces:</p>
                  <div className="grid grid-cols-2 gap-2 text-xs">
                    <div className="glass-dark p-2 rounded flex items-center">
                      <div className="w-4 h-4 bg-red-500 rounded-full mr-2"></div>
                      Red pieces
                    </div>
                    <div className="glass-dark p-2 rounded flex items-center">
                      <div className="w-4 h-4 bg-yellow-500 rounded-full mr-2"></div>
                      Yellow pieces
                    </div>
                  </div>
                </div>
              </div>

              <div>
                <h3 className="text-lg font-semibold text-white mb-3 flex items-center">
                  <ArrowRight className="w-5 h-5 mr-2 text-green-400" />
                  Taking Turns
                </h3>
                <ul className="text-sm space-y-2">
                  <li className="flex items-start">
                    <span className="w-2 h-2 bg-green-400 rounded-full mt-2 mr-2 flex-shrink-0"></span>
                    Players take turns dropping their pieces into the board
                  </li>
                  <li className="flex items-start">
                    <span className="w-2 h-2 bg-green-400 rounded-full mt-2 mr-2 flex-shrink-0"></span>
                    Click on any column to drop your piece in that column
                  </li>
                  <li className="flex items-start">
                    <span className="w-2 h-2 bg-green-400 rounded-full mt-2 mr-2 flex-shrink-0"></span>
                    Pieces fall to the lowest available position in the column
                  </li>
                </ul>
              </div>

              <div>
                <h3 className="text-lg font-semibold text-white mb-3 flex items-center">
                  <Zap className="w-5 h-5 mr-2 text-red-400" />
                  Winning Conditions
                </h3>
                <p className="text-sm leading-relaxed mb-2">
                  You win by connecting 4 of your pieces in a row:
                </p>
                <ul className="text-sm space-y-1">
                  <li className="flex items-start">
                    <span className="w-2 h-2 bg-red-400 rounded-full mt-2 mr-2 flex-shrink-0"></span>
                    <strong>Horizontally:</strong> 4 pieces in a row across
                  </li>
                  <li className="flex items-start">
                    <span className="w-2 h-2 bg-red-400 rounded-full mt-2 mr-2 flex-shrink-0"></span>
                    <strong>Vertically:</strong> 4 pieces stacked in a column
                  </li>
                  <li className="flex items-start">
                    <span className="w-2 h-2 bg-red-400 rounded-full mt-2 mr-2 flex-shrink-0"></span>
                    <strong>Diagonally:</strong> 4 pieces in a diagonal line
                  </li>
                </ul>
              </div>

              <div>
                <h3 className="text-lg font-semibold text-white mb-3 flex items-center">
                  <Star className="w-5 h-5 mr-2 text-amber-400" />
                  Strategy Tips
                </h3>
                <ul className="text-sm space-y-2">
                  <li className="flex items-start">
                    <span className="w-2 h-2 bg-amber-400 rounded-full mt-2 mr-2 flex-shrink-0"></span>
                    Control the center columns for more winning opportunities
                  </li>
                  <li className="flex items-start">
                    <span className="w-2 h-2 bg-amber-400 rounded-full mt-2 mr-2 flex-shrink-0"></span>
                    Block your opponent&apos;s potential winning moves
                  </li>
                  <li className="flex items-start">
                    <span className="w-2 h-2 bg-amber-400 rounded-full mt-2 mr-2 flex-shrink-0"></span>
                    Look for multiple winning threats simultaneously
                  </li>
                </ul>
              </div>

              <div>
                <h3 className="text-lg font-semibold text-white mb-3 flex items-center">
                  <Trophy className="w-5 h-5 mr-2 text-yellow-400" />
                  Game End
                </h3>
                <p className="text-sm leading-relaxed">
                          The game ends when a player connects 4 pieces in a row, or when the board is
        completely full (a draw). The first player to get 4 in a row wins!
                </p>
              </div>

              <div className="pt-4 border-t border-white/10">
                <p className="text-xs text-white/60 text-center">
                  Connect 4 was invented by Howard Wexler and Ned Strongin in 1974. It&apos;s a
                  classic strategy game that&apos;s easy to learn but challenging to master!
                </p>
              </div>

              <div className="flex justify-center mt-6">
                <button
                  onClick={onClose}
                  className="px-6 py-2 bg-gradient-to-r from-red-600 to-yellow-600 text-white rounded-lg font-bold hover:from-red-700 hover:to-yellow-700 transition-all duration-200 shadow-lg focus:outline-none focus:ring-2 focus:ring-red-400"
                  data-testid="help-close-bottom"
                >
                  Close
                </button>
              </div>
            </div>
          </motion.div>
        </motion.div>
      )}
    </AnimatePresence>
  );
}
