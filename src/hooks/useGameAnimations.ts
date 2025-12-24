import { useState, useEffect } from 'react';
import { Player, GameState } from '@/lib/types';
import { useGameStore } from '@/lib/game-store';
import { soundEffects } from '@/lib/sound-effects';

interface Celebration {
  id: string;
  position: { x: number; y: number };
  player: Player;
}

interface DroppingPiece {
  id: string;
  column: number;
  row: number;
  player: Player;
}

export function useGameAnimations(gameState: GameState, boardRef: React.RefObject<HTMLDivElement>) {
  const [celebrations, setCelebrations] = useState<Celebration[]>([]);
  const [droppingPieces, setDroppingPieces] = useState<DroppingPiece[]>([]);
  const [showWinAnimation, setShowWinAnimation] = useState(false);

  const { actions, pendingMove } = useGameStore();

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

  const handleWinAnimationComplete = () => {
    setShowWinAnimation(false);
    // Show the winner modal after the win animation completes
    setTimeout(() => {
      actions.showWinnerModal();
    }, 500); // Small delay for smooth transition
  };

  return {
    celebrations,
    droppingPieces,
    showWinAnimation,
    handleWinAnimationComplete,
  };
}
