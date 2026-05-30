export interface Point {
  x: number;
  y: number;
}

export interface BaseEntity {
  opacity: number;
  color: string;
  life: number;
  fadeOut: boolean;
  targetOpacity: number;
}

export interface Shape extends BaseEntity {
  type: 'line' | 'circle' | 'triangle' | 'square' | 'star';
  x: number;
  y: number;
  size: number;
  rotation: number;
  speed: number;
  direction: Point;
  pulse: number;
  pulseSpeed: number;
  targetX: number;
  targetY: number;
  targetSize: number;
}

export interface Line extends BaseEntity {
  x1: number;
  y1: number;
  x2: number;
  y2: number;
  width: number;
}

export interface Particle extends BaseEntity {
  x: number;
  y: number;
  size: number;
  direction: Point;
}

export const COLORS = [
  'rgba(99, 102, 241, 0.5)', // Indigo
  'rgba(236, 72, 153, 0.5)', // Pink
  'rgba(251, 191, 36, 0.5)', // Amber
  'rgba(34, 197, 94, 0.5)', // Green
  'rgba(147, 51, 234, 0.5)', // Purple
  'rgba(59, 130, 246, 0.5)', // Blue
];

export class BackgroundEffects {
  shapes: Shape[] = [];
  lines: Line[] = [];
  particles: Particle[] = [];
  width: number = 0;
  height: number = 0;

  constructor(width: number, height: number) {
    this.resize(width, height);
    this.init();
  }

  resize(width: number, height: number) {
    this.width = width;
    this.height = height;
  }

  init() {
    for (let i = 0; i < 30; i++) {
      this.shapes.push(this.createShape());
    }
    for (let i = 0; i < 20; i++) {
      this.lines.push(this.createLine());
    }
    for (let i = 0; i < 40; i++) {
      this.particles.push(this.createParticle());
    }
  }

  createShape(): Shape {
    const types: Array<'line' | 'circle' | 'triangle' | 'square' | 'star'> = [
      'line',
      'circle',
      'triangle',
      'square',
      'star',
    ];
    const type = types[Math.floor(Math.random() * types.length)];

    return {
      type,
      x: Math.random() * this.width,
      y: Math.random() * this.height,
      size: Math.random() * 50 + 25,
      rotation: Math.random() * Math.PI * 2,
      speed: Math.random() * 0.03 + 0.008,
      opacity: Math.random() * 0.7 + 0.3,
      color: COLORS[Math.floor(Math.random() * COLORS.length)],
      life: 1.0,
      direction: {
        x: (Math.random() - 0.5) * 0.8,
        y: (Math.random() - 0.5) * 0.8,
      },
      pulse: Math.random() * Math.PI * 2,
      pulseSpeed: Math.random() * 0.05 + 0.02,
      fadeOut: false,
      targetX: 0,
      targetY: 0,
      targetSize: 0,
      targetOpacity: 0,
    };
  }

  createLine(): Line {
    return {
      x1: Math.random() * this.width,
      y1: Math.random() * this.height,
      x2: Math.random() * this.width,
      y2: Math.random() * this.height,
      opacity: Math.random() * 0.3 + 0.1,
      color: COLORS[Math.floor(Math.random() * COLORS.length)],
      life: 1.0,
      width: Math.random() * 2 + 1,
      fadeOut: false,
      targetOpacity: 0,
    };
  }

  createParticle(): Particle {
    return {
      x: Math.random() * this.width,
      y: Math.random() * this.height,
      size: Math.random() * 4 + 1,
      opacity: Math.random() * 0.6 + 0.2,
      color: COLORS[Math.floor(Math.random() * COLORS.length)],
      life: 1.0,
      direction: {
        x: (Math.random() - 0.5) * 0.3,
        y: (Math.random() - 0.5) * 0.3,
      },
      fadeOut: false,
      targetOpacity: 0,
    };
  }

  update() {
    this.updateShapes();
    this.updateLines();
    this.updateParticles();
  }

  updateShapes() {
    for (let i = this.shapes.length - 1; i >= 0; i--) {
      const shape = this.shapes[i];

      shape.rotation += shape.speed;
      shape.x += shape.direction.x;
      shape.y += shape.direction.y;
      shape.life -= 0.001;
      shape.pulse += shape.pulseSpeed;

      if (shape.x < -shape.size) shape.x = this.width + shape.size;
      if (shape.x > this.width + shape.size) shape.x = -shape.size;
      if (shape.y < -shape.size) shape.y = this.height + shape.size;
      if (shape.y > this.height + shape.size) shape.y = -shape.size;

      if (shape.life <= 0.1 && !shape.fadeOut) {
        shape.fadeOut = true;
        shape.targetOpacity = 0;
        shape.targetSize = shape.size * 0.5;
      }

      if (shape.fadeOut) {
        shape.opacity = Math.max(0, shape.opacity - 0.02);
        shape.size = Math.max(shape.size * 0.98, shape.targetSize);
        if (shape.opacity <= 0) {
          this.shapes.splice(i, 1);
          const newShape = this.createShape();
          newShape.x = shape.x;
          newShape.y = shape.y;
          newShape.opacity = 0;
          newShape.size = shape.size * 0.5;
          newShape.targetOpacity = newShape.opacity;
          newShape.targetSize = newShape.size;
          this.shapes.push(newShape);
          continue;
        }
      } else {
        shape.opacity = Math.min(shape.opacity + 0.01, shape.targetOpacity || shape.opacity);
        shape.size = Math.min(shape.size * 1.01, shape.targetSize || shape.size);
      }
    }
  }

  updateLines() {
    for (let i = this.lines.length - 1; i >= 0; i--) {
      const line = this.lines[i];

      line.life -= 0.002;

      if (line.life <= 0.1 && !line.fadeOut) {
        line.fadeOut = true;
        line.targetOpacity = 0;
      }

      if (line.fadeOut) {
        line.opacity = Math.max(0, line.opacity - 0.02);
        if (line.opacity <= 0) {
          this.lines.splice(i, 1);
          const newLine = this.createLine();
          newLine.opacity = 0;
          newLine.targetOpacity = newLine.opacity;
          this.lines.push(newLine);
          continue;
        }
      } else {
        line.opacity = Math.min(line.opacity + 0.01, line.targetOpacity || line.opacity);
      }
    }
  }

  updateParticles() {
    for (let i = this.particles.length - 1; i >= 0; i--) {
      const particle = this.particles[i];

      particle.x += particle.direction.x;
      particle.y += particle.direction.y;
      particle.life -= 0.001;

      if (particle.x < -particle.size) particle.x = this.width + particle.size;
      if (particle.x > this.width + particle.size) particle.x = -particle.size;
      if (particle.y < -particle.size) particle.y = this.height + particle.size;
      if (particle.y > this.height + particle.size) particle.y = -particle.size;

      if (particle.life <= 0.1 && !particle.fadeOut) {
        particle.fadeOut = true;
        particle.targetOpacity = 0;
      }

      if (particle.fadeOut) {
        particle.opacity = Math.max(0, particle.opacity - 0.02);
        if (particle.opacity <= 0) {
          this.particles.splice(i, 1);
          const newParticle = this.createParticle();
          newParticle.x = particle.x;
          newParticle.y = particle.y;
          newParticle.opacity = 0;
          newParticle.targetOpacity = newParticle.opacity;
          this.particles.push(newParticle);
          continue;
        }
      } else {
        particle.opacity = Math.min(
          particle.opacity + 0.01,
          particle.targetOpacity || particle.opacity,
        );
      }
    }
  }

  draw(ctx: CanvasRenderingContext2D) {
    ctx.fillStyle = 'rgba(0, 0, 0, 0.95)';
    ctx.fillRect(0, 0, this.width, this.height);

    this.drawParticles(ctx);
    this.drawLines(ctx);
    this.drawShapes(ctx);

    ctx.save();
    ctx.globalCompositeOperation = 'screen';
    ctx.fillStyle = 'rgba(99, 102, 241, 0.02)';
    ctx.fillRect(0, 0, this.width, this.height);
    ctx.restore();
  }

  drawParticles(ctx: CanvasRenderingContext2D) {
    for (const particle of this.particles) {
      ctx.save();
      ctx.globalAlpha = particle.opacity * particle.life;
      ctx.fillStyle = particle.color;
      ctx.beginPath();
      ctx.arc(particle.x, particle.y, particle.size, 0, Math.PI * 2);
      ctx.fill();
      ctx.restore();
    }
  }

  drawLines(ctx: CanvasRenderingContext2D) {
    for (const line of this.lines) {
      ctx.save();
      ctx.globalAlpha = line.opacity * line.life;
      ctx.strokeStyle = line.color;
      ctx.lineWidth = line.width;
      ctx.lineCap = 'round';
      ctx.beginPath();
      ctx.moveTo(line.x1, line.y1);
      ctx.lineTo(line.x2, line.y2);
      ctx.stroke();
      ctx.restore();
    }
  }

  drawShapes(ctx: CanvasRenderingContext2D) {
    for (const shape of this.shapes) {
      const pulseScale = 1 + Math.sin(shape.pulse) * 0.2;
      const currentSize = shape.size * pulseScale;

      ctx.save();
      ctx.globalAlpha = shape.opacity * shape.life;
      ctx.fillStyle = shape.color;
      ctx.strokeStyle = shape.color;
      ctx.lineWidth = 2;

      ctx.translate(shape.x, shape.y);
      ctx.rotate(shape.rotation);
      ctx.scale(pulseScale, pulseScale);

      switch (shape.type) {
        case 'circle':
          const gradient = ctx.createRadialGradient(0, 0, 0, 0, 0, currentSize / 2);
          gradient.addColorStop(0, shape.color);
          gradient.addColorStop(1, 'rgba(255, 255, 255, 0)');
          ctx.fillStyle = gradient;
          ctx.beginPath();
          ctx.arc(0, 0, currentSize / 2, 0, Math.PI * 2);
          ctx.fill();
          ctx.strokeStyle = shape.color;
          ctx.lineWidth = 1;
          ctx.stroke();
          break;
        case 'square':
          ctx.strokeRect(-currentSize / 2, -currentSize / 2, currentSize, currentSize);
          break;
        case 'triangle':
          ctx.beginPath();
          ctx.moveTo(0, -currentSize / 2);
          ctx.lineTo(-currentSize / 2, currentSize / 2);
          ctx.lineTo(currentSize / 2, currentSize / 2);
          ctx.closePath();
          ctx.stroke();
          break;
        case 'line':
          ctx.beginPath();
          ctx.moveTo(-currentSize / 2, 0);
          ctx.lineTo(currentSize / 2, 0);
          ctx.stroke();
          break;
        case 'star':
          ctx.beginPath();
          for (let i = 0; i < 5; i++) {
            const angle = (i * 4 * Math.PI) / 5 - Math.PI / 2;
            const x = (Math.cos(angle) * currentSize) / 2;
            const y = (Math.sin(angle) * currentSize) / 2;
            if (i === 0) ctx.moveTo(x, y);
            else ctx.lineTo(x, y);
          }
          ctx.closePath();
          ctx.stroke();
          break;
      }

      ctx.restore();
    }
  }
}
