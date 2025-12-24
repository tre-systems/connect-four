'use client';

import { useAnimatedBackground } from '../hooks/useAnimatedBackground';

export default function AnimatedBackground() {
  const canvasRef = useAnimatedBackground();

  return (
    <canvas
      ref={canvasRef}
      className="fixed inset-0 pointer-events-none z-0"
      style={{ background: 'transparent' }}
    />
  );
}
