'use client';

import React, { useState, useEffect, useRef } from 'react';
import { GameState, Player } from '@/lib/types';
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

interface GameBoardProps {
  gameState: GameState;
  aiThinking?: boolean;
  onResetGame: () => void;
  soundEnabled: boolean;
  onToggleSound: () => void;
  onShowHowToPlay: () => void;
  watchMode?: boolean;
  gameMode?: 'human-vs-human' | 'human-vs-ai' | 'ai-vs-ai';
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
  const [celebrations, setCelebrations] = useState<
    Array<{ id: string; position: { x: number; y: number }; player: Player }>
  >([]);
  const [droppingPieces, setDroppingPieces] = useState<
    Array<{ id: string; column: number; row: number; player: Player }>
  >([]);
  const [showWinAnimation, setShowWinAnimation] = useState(false);
  const [isMounted, setIsMounted] = useState(false);
  const boardRef = useRef<HTMLDivElement>(null);
  const { actions, pendingMove, showWinnerModal } = useGameStore();

  React.useEffect(() => {
    setIsMounted(true);
  }, []);

  React.useEffect(() => {
    if (gameState.gameStatus === 'finished' && gameState.winner) {
      // Handle game completion
    }
  }, [gameState.gameStatus, gameState.winner, actions]);

  useEffect(() => {
    if (gameState.gameStatus === 'finished' && gameState.winner) {
      const boardRect = boardRef.current?.getBoundingClientRect();
      if (boardRect) {
        // eslint-disable-next-line
        setCelebrations(prevCelebrations => [
          ...prevCelebrations,
          {
            id: `celebration-${Date.now()}-${gameState.winner}`,
            position: {
              x: boardRect.left + boardRect.width / 2,
              y: boardRect.top + boardRect.height / 2,
            },
            player: gameState.winner as Player,
          },
        ]);
      }

      // Show Connect Four win animation
      if (gameState.winningLine) {
        setShowWinAnimation(true);
        // Play win animation sound
        soundEffects.winAnimation();
      }
    }
  }, [gameState.gameStatus, gameState.winner, gameState.winningLine]);

  useEffect(() => {
    celebrations.forEach(celebration => {
      setTimeout(() => {
        setCelebrations(prev => prev.filter(c => c.id !== celebration.id));
      }, 3000);
    });
  }, [celebrations]);

  // Handle dropping piece animations
  useEffect(() => {
    if (pendingMove) {
      const { column, player } = pendingMove;

      // Calculate the row where the piece will land
      const col = gameState.board[column];
      const row = col.lastIndexOf(null);
      if (row === -1) return; // Column full

      const dropId = `drop-${Date.now()}-${column}-${row}`;
      // eslint-disable-next-line
      setDroppingPieces(prev => [
        ...prev,
        {
          id: dropId,
          column,
          row,
          player,
        },
      ]);

      // After animation completes, actually make the move
      setTimeout(() => {
        setDroppingPieces(prev => prev.filter(p => p.id !== dropId));
        actions.completeMove();
      }, 800);
    }
  }, [pendingMove, gameState, actions]);

  const handleColumnClick = (column: number) => {
    if (gameState.gameStatus === 'playing' && gameState.currentPlayer === 'player1' && !watchMode) {
      actions.makeMove(column);
      soundEffects.pieceMove();
    }
  };

  const handleWinAnimationComplete = () => {
    setShowWinAnimation(false);
    // Show the winner modal after the win animation completes
    setTimeout(() => {
      actions.showWinnerModal();
    }, 500); // Small delay for smooth transition
  };

  const winningSet = new Set(
    Array.isArray(gameState.winningLine?.positions)
      ? gameState.winningLine.positions.map(pos => `${pos.column},${pos.row}`)
      : []
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
              // Check if column has space (any cell is null)
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
                      isClickable={false} // Individual squares are not clickable
                      onColumnClick={handleColumnClick}
                      isWinning={winningSet.has(`${colIndex},${rowIndex}`)}
                    />
                  ))}

                  {/* Column click indicator */}
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

            {/* Dropping piece animation - positioned relative to the entire board */}
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

            {/* Connect Four Win Animation */}
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
