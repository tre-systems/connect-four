'use client';


import { motion } from 'framer-motion';
import type { AIType } from '../lib/types';

interface AISelectionCardProps {
  aiType: AIType;
  title: string;
  description: string;
  subtitle?: string;
  colorClass: string;
  borderColorClass: string;
  isSelected: boolean;
  onClick: () => void;
  icon?: string;
  'data-testid'?: string;
}

export default function AISelectionCard({
  title,
  description,
  subtitle,
  colorClass,
  borderColorClass,
  isSelected,
  onClick,
  icon,
  'data-testid': dataTestId,
}: AISelectionCardProps) {
  return (
    <motion.button
      onClick={onClick}
      className={`
        w-full p-5 rounded-xl border-2 text-left transition-all duration-300
        ${isSelected ? 'bg-gray-700/50' : 'bg-gray-800/50 hover:bg-gray-700/50'}
        focus:ring-4 focus:outline-none
        ${isSelected ? 'ring-2 ring-blue-500' : borderColorClass}
      `}
      whileHover={{ scale: 1.02 }}
      whileTap={{ scale: 0.98 }}
      data-testid={dataTestId}
    >
      <div className="flex items-start space-x-4">
        <div
          className={`p-3 rounded-full ${isSelected ? 'bg-blue-600' : 'bg-gray-900/50'} ${colorClass} flex-shrink-0`}
        >
          {icon ? (
            <span className="text-2xl">{icon}</span>
          ) : isSelected ? (
            <svg className="w-6 h-6 text-white" fill="currentColor" viewBox="0 0 20 20">
              <path
                fillRule="evenodd"
                d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z"
                clipRule="evenodd"
              />
            </svg>
          ) : null}
        </div>
        <div className="flex-1 min-w-0">
          <h3 className={`text-xl font-bold ${colorClass} mb-1`}>{title}</h3>
          {subtitle && <div className="text-sm text-gray-400 mb-2">{subtitle}</div>}
          <p className="text-sm text-gray-300 leading-relaxed">{description}</p>
        </div>
        {isSelected && (
          <div className="text-blue-500 flex-shrink-0">
            <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 20 20">
              <path
                fillRule="evenodd"
                d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z"
                clipRule="evenodd"
              />
            </svg>
          </div>
        )}
      </div>
    </motion.button>
  );
}
