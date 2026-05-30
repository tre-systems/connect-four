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

export function useGameAnimations(
  gameState: GameState,
  boardRef: React.RefObject<HTMLDivElement | null>,
) {
  const [celebrations, setCelebrations] = useState<Celebration[]>([]);
  const [droppingPieces, setDroppingPieces] = useState<DroppingPiece[]>([]);
  const [showWinAnimation, setShowWinAnimation] = useState(false);

  const { actions, pendingMove } = useGameStore();

  useEffect(() => {
    if (gameState.gameStatus === 'finished' && gameState.winner) {
      const winner = gameState.winner;
      const boardRect = boardRef.current?.getBoundingClientRect();
      if (boardRect) {
        // eslint-disable-next-line react-hooks/set-state-in-effect
        setCelebrations(prevCelebrations => [
          ...prevCelebrations,
          {
            id: `celebration-${Date.now()}-${winner}`,
            position: {
              x: boardRect.left + boardRect.width / 2,
              y: boardRect.top + boardRect.height / 2,
            },
            player: winner,
          },
        ]);
      }

      if (gameState.winningLine) {
        setShowWinAnimation(true);
        soundEffects.winAnimation();
      }
    }
  }, [gameState.gameStatus, gameState.winner, gameState.winningLine, boardRef]);

  useEffect(() => {
    celebrations.forEach(celebration => {
      setTimeout(() => {
        setCelebrations(prev => prev.filter(c => c.id !== celebration.id));
      }, 3000);
    });
  }, [celebrations]);

  useEffect(() => {
    if (pendingMove) {
      const { column, player } = pendingMove;

      const col = gameState.board[column];
      const row = col.lastIndexOf(null);
      if (row === -1) return;

      const dropId = `drop-${Date.now()}-${column}-${row}`;

      // eslint-disable-next-line react-hooks/set-state-in-effect
      setDroppingPieces(prev => [
        ...prev,
        {
          id: dropId,
          column,
          row,
          player,
        },
      ]);

      setTimeout(() => {
        setDroppingPieces(prev => prev.filter(p => p.id !== dropId));
        actions.completeMove();
      }, 800);
    }
  }, [pendingMove, gameState, actions]);

  const handleWinAnimationComplete = () => {
    setShowWinAnimation(false);
    setTimeout(() => {
      actions.showWinnerModal();
    }, 500);
  };

  return {
    celebrations,
    droppingPieces,
    showWinAnimation,
    handleWinAnimationComplete,
  };
}
