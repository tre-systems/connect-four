'use client';

import React, { useState, useRef } from 'react';
import { GameState, GameMode } from '@/lib/types';
import { motion, AnimatePresence } from 'framer-motion';
import { useGameStore } from '@/lib/game-store';

import VictoryCelebration from './animations/VictoryCelebration';
import ConnectFourWin from './animations/ConnectFourWin';
import GameSquare from './game/GameSquare';
import GameCompletionOverlay from './game/GameCompletionOverlay';
import GameControls from './game/GameControls';
import GameStatus from './game/GameStatus';
import GamePiece from './game/GamePiece';
import { soundEffects } from '@/lib/sound-effects';
import { useGameAnimations } from '@/hooks/useGameAnimations';

interface GameBoardProps {
  gameState: GameState;
  aiThinking?: boolean;
  onResetGame: () => void;
  soundEnabled: boolean;
  onToggleSound: () => void;
  onShowHowToPlay: () => void;
  watchMode?: boolean;
  gameMode?: GameMode;
}

export default function GameBoard({
  gameState,
  aiThinking = false,
  onResetGame,
  soundEnabled,
  onToggleSound,
  onShowHowToPlay,
  watchMode = false,
  gameMode = 'human-vs-ai',
}: GameBoardProps) {
  const [isMounted, setIsMounted] = useState(false);
  const boardRef = useRef<HTMLDivElement>(null);
  const { actions, showWinnerModal } = useGameStore();

  const { celebrations, droppingPieces, showWinAnimation, handleWinAnimationComplete } =
    useGameAnimations(gameState, boardRef);

  React.useEffect(() => {
    setIsMounted(true);
  }, []);

  const handleColumnClick = (column: number) => {
    if (gameState.gameStatus === 'playing' && gameState.currentPlayer === 'player1' && !watchMode) {
      actions.makeMove(column);
      soundEffects.pieceMove();
    }
  };

  const winningSet = new Set(
    Array.isArray(gameState.winningLine?.positions)
      ? gameState.winningLine.positions.map(pos => `${pos.column},${pos.row}`)
      : [],
  );

  return (
    <>
      <AnimatePresence>
        {celebrations.map(celebration => (
          <VictoryCelebration
            key={celebration.id}
            position={celebration.position}
            player={celebration.player}
          />
        ))}
      </AnimatePresence>
      <AnimatePresence>
        {gameState.gameStatus === 'finished' && showWinnerModal && (
          <GameCompletionOverlay
            gameState={gameState}
            onResetGame={onResetGame}
            gameMode={gameMode}
          />
        )}
      </AnimatePresence>
      <motion.div className="w-full max-w-md mx-auto space-y-3" data-testid="game-board">
        <motion.div
          ref={boardRef}
          className="glass mystical-glow rounded-xl p-4 relative"
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ duration: 0.5 }}
        >
          <div className="text-center mb-3">
            <GameStatus gameState={gameState} aiThinking={aiThinking} gameMode={gameMode} />
          </div>
          <div className="grid grid-cols-7 gap-1 bg-black/20 p-2 rounded-lg backdrop-blur relative">
            {gameState.board.map((column, colIndex) => {
              const hasSpace = column.some(cell => cell === null);
              const isClickable =
                gameState.gameStatus === 'playing' &&
                gameState.currentPlayer === 'player1' &&
                !watchMode &&
                hasSpace;

              return (
                <motion.div
                  key={colIndex}
                  className="flex flex-col gap-1 relative"
                  onClick={() => isClickable && handleColumnClick(colIndex)}
                  style={{ cursor: isClickable ? 'pointer' : 'default' }}
                  whileHover={isClickable ? { scale: 1.02 } : {}}
                  transition={{ type: 'spring', stiffness: 400, damping: 25 }}
                  data-testid={`column-${colIndex}`}
                >
                  {column.map((cell, rowIndex) => (
                    <GameSquare
                      key={`${colIndex}-${rowIndex}`}
                      column={colIndex}
                      row={rowIndex}
                      player={cell}
                      isClickable={false}
                      onColumnClick={handleColumnClick}
                      isWinning={winningSet.has(`${colIndex},${rowIndex}`)}
                    />
                  ))}

                  {isClickable && isMounted && (
                    <motion.div
                      className="absolute inset-0 rounded-lg border-2 border-green-400/50 pointer-events-none"
                      animate={{
                        boxShadow: [
                          '0 0 0 0 rgba(34, 197, 94, 0.3)',
                          '0 0 0 8px rgba(34, 197, 94, 0)',
                        ],
                      }}
                      transition={{ duration: 2, repeat: Infinity }}
                    />
                  )}
                </motion.div>
              );
            })}

            {droppingPieces.map(drop => (
              <motion.div
                key={drop.id}
                className="absolute z-20"
                style={{
                  left: `${(drop.column / 7) * 100}%`,
                  top: '0%',
                  width: 'calc(100% / 7 - 8px)',
                  height: 'calc(100% / 6 - 4px)',
                }}
                initial={{ y: 0, opacity: 0.8 }}
                animate={{
                  y: `${(drop.row / 6) * 100}%`,
                  opacity: 1,
                }}
                transition={{
                  y: {
                    type: 'tween',
                    ease: 'easeInOut',
                    duration: 0.8,
                  },
                  opacity: {
                    duration: 0.3,
                  },
                }}
              >
                <div className="w-full h-full flex items-center justify-center">
                  <GamePiece player={drop.player} isClickable={false} />
                </div>
              </motion.div>
            ))}

            <AnimatePresence>
              {showWinAnimation && gameState.winningLine && gameState.winner && (
                <ConnectFourWin onComplete={handleWinAnimationComplete} />
              )}
            </AnimatePresence>
          </div>
          <GameControls
            soundEnabled={soundEnabled}
            onToggleSound={onToggleSound}
            onShowHowToPlay={onShowHowToPlay}
            onResetGame={onResetGame}
          />
        </motion.div>
      </motion.div>
    </>
  );
}
