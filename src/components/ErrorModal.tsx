'use client';

import { motion, AnimatePresence } from 'framer-motion';
import { X, AlertTriangle } from 'lucide-react';

interface ErrorModalProps {
  isOpen: boolean;
  onClose: () => void;
  error: string;
}

export default function ErrorModal({ isOpen, onClose, error }: ErrorModalProps) {
  return (
    <AnimatePresence>
      {isOpen && (
        <motion.div
          className="fixed inset-0 bg-black/60 backdrop-blur-sm flex items-center justify-center z-50 p-4"
          initial={{ opacity: 0 }}
          animate={{ opacity: 1 }}
          exit={{ opacity: 0 }}
          onClick={onClose}
          data-testid="error-modal"
        >
          <motion.div
            className="glass mystical-glow rounded-xl p-6 max-w-md w-full"
            initial={{ scale: 0.9, opacity: 0, y: 20 }}
            animate={{ scale: 1, opacity: 1, y: 0 }}
            exit={{ scale: 0.9, opacity: 0, y: 20 }}
            onClick={e => e.stopPropagation()}
          >
            <div className="flex items-center justify-between mb-4">
              <div className="flex items-center space-x-2">
                <AlertTriangle className="w-5 h-5 text-red-400" />
                <h2 className="text-xl font-bold text-white neon-text">AI Error</h2>
              </div>
              <motion.button
                onClick={onClose}
                className="p-1.5 glass-dark rounded-lg text-white/70 hover:text-white transition-colors"
                whileHover={{ scale: 1.05 }}
                whileTap={{ scale: 0.95 }}
                data-testid="error-close"
              >
                <X className="w-4 h-4" />
              </motion.button>
            </div>

            <div className="mb-6">
              <p className="text-white/80 text-sm leading-relaxed" data-testid="error-message">
                {error}
              </p>
            </div>

            <div className="flex justify-center">
              <button
                onClick={onClose}
                className="px-6 py-2 bg-gradient-to-r from-red-600 to-yellow-600 text-white rounded-lg font-bold hover:from-red-700 hover:to-yellow-700 transition-all duration-200 shadow-lg focus:outline-none focus:ring-2 focus:ring-red-400"
                data-testid="error-close-bottom"
              >
                OK
              </button>
            </div>
          </motion.div>
        </motion.div>
      )}
    </AnimatePresence>
  );
}
