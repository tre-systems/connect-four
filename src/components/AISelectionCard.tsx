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
  onClick,
  icon,
  'data-testid': dataTestId,
}: AISelectionCardProps) {
  return (
    <motion.button
      onClick={onClick}
      className={`
        w-full p-5 rounded-xl border-2 text-left transition-all duration-300
        bg-gray-800/50 hover:bg-gray-700/50
        focus:ring-4 focus:outline-none
        ${borderColorClass}
      `}
      whileHover={{ scale: 1.02 }}
      whileTap={{ scale: 0.98 }}
      data-testid={dataTestId}
    >
      <div className="flex items-start space-x-4">
        <div className={`p-3 rounded-full bg-gray-900/50 ${colorClass} flex-shrink-0`}>
          {icon ? <span className="text-2xl">{icon}</span> : null}
        </div>
        <div className="flex-1 min-w-0">
          <h3 className={`text-xl font-bold ${colorClass} mb-1`}>{title}</h3>
          {subtitle && <div className="text-sm text-gray-400 mb-2">{subtitle}</div>}
          <p className="text-sm text-gray-300 leading-relaxed">{description}</p>
        </div>
      </div>
    </motion.button>
  );
}
