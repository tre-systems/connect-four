'use client';

import { motion } from 'framer-motion';

interface ModeSelectionCardProps {
  icon: React.ElementType;
  title: string;
  description: string;
  subtitle?: string;
  onClick: () => void;
  colorClass: string;
  borderColorClass: string;
  'data-testid'?: string;
}

export default function ModeSelectionCard({
  icon: Icon,
  title,
  description,
  subtitle,
  onClick,
  colorClass,
  borderColorClass,
  'data-testid': dataTestId,
}: ModeSelectionCardProps) {
  return (
    <motion.button
      onClick={onClick}
      className={`
        w-full p-6 rounded-xl border-2 text-left transition-all duration-300
        bg-gray-800/50 hover:bg-gray-700/50 focus:ring-4 focus:outline-none
        ${borderColorClass}
      `}
      whileHover={{ scale: 1.03 }}
      whileTap={{ scale: 0.98 }}
      data-testid={dataTestId}
    >
      <div className="flex items-center space-x-4">
        <div className={`p-3 rounded-full bg-gray-900/50 ${colorClass}`}>
          <Icon className="w-6 h-6" />
        </div>
        <div>
          <h3 className={`text-lg font-bold ${colorClass}`}>{title}</h3>
          {subtitle && <div className="text-xs text-gray-400 mt-0.5">{subtitle}</div>}
          <p className="text-sm text-gray-300 mt-1">{description}</p>
        </div>
      </div>
    </motion.button>
  );
}
