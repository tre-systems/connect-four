import { useEffect, useRef } from 'react';
import { BackgroundEffects } from '../lib/visuals/background-effects';

export function useAnimatedBackground() {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const effectsRef = useRef<BackgroundEffects | null>(null);

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    const resizeCanvas = () => {
      canvas.width = window.innerWidth;
      canvas.height = window.innerHeight;

      if (!effectsRef.current) {
        effectsRef.current = new BackgroundEffects(canvas.width, canvas.height);
      } else {
        effectsRef.current.resize(canvas.width, canvas.height);
      }
    };

    resizeCanvas();
    window.addEventListener('resize', resizeCanvas);

    let animationFrameId: number;

    const animate = () => {
      if (effectsRef.current) {
        effectsRef.current.update();
        effectsRef.current.draw(ctx);
      }
      animationFrameId = requestAnimationFrame(animate);
    };

    animate();

    return () => {
      window.removeEventListener('resize', resizeCanvas);
      cancelAnimationFrame(animationFrameId);
    };
  }, []);

  return canvasRef;
}
