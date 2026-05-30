class SoundEffects {
  private audioContext: AudioContext | null = null;
  private enabled = true;

  constructor() {
    if (typeof window !== 'undefined' && window.AudioContext) {
      try {
        this.audioContext = new window.AudioContext();
      } catch (error) {
        console.warn('Web Audio API not supported:', error);
      }
    }
  }

  private async ensureAudioContext() {
    if (!this.audioContext) return null;

    if (this.audioContext.state === 'suspended') {
      await this.audioContext.resume();
    }

    return this.audioContext;
  }

  private async playTone(
    frequency: number,
    duration: number,
    type: OscillatorType = 'sine',
    volume = 0.1,
  ) {
    if (!this.enabled) return;

    const ctx = await this.ensureAudioContext();
    if (!ctx) return;

    const oscillator = ctx.createOscillator();
    const gainNode = ctx.createGain();

    oscillator.connect(gainNode);
    gainNode.connect(ctx.destination);

    oscillator.frequency.value = frequency;
    oscillator.type = type;

    gainNode.gain.setValueAtTime(0, ctx.currentTime);
    gainNode.gain.linearRampToValueAtTime(volume, ctx.currentTime + 0.01);
    gainNode.gain.exponentialRampToValueAtTime(0.001, ctx.currentTime + duration);

    oscillator.start(ctx.currentTime);
    oscillator.stop(ctx.currentTime + duration);
  }

  private async playChord(
    frequencies: number[],
    duration: number,
    type: OscillatorType = 'sine',
    volume = 0.05,
  ) {
    frequencies.forEach(freq => this.playTone(freq, duration, type, volume));
  }

  async columnSelect() {
    for (let i = 0; i < 4; i++) {
      setTimeout(() => {
        this.playTone(200 + Math.random() * 100, 0.1, 'square', 0.05);
      }, i * 50);
    }
  }

  async pieceMove() {
    await this.playTone(523.25, 0.2, 'sine', 0.08); // C5
  }

  async specialLanding() {
    const frequencies = [523.25, 659.25, 783.99]; // C5-E5-G5 chord
    await this.playChord(frequencies, 0.5, 'sine', 0.06);
  }

  async gameWin() {
    const melody = [523.25, 659.25, 783.99, 1046.5]; // C5-E5-G5-C6
    melody.forEach((freq, i) => {
      setTimeout(() => this.playTone(freq, 0.3, 'sine', 0.1), i * 200);
    });
  }

  async winAnimation() {
    const sequence = [
      { freq: 523.25, duration: 0.2, type: 'sine' as OscillatorType }, // C5
      { freq: 659.25, duration: 0.2, type: 'sine' as OscillatorType }, // E5
      { freq: 783.99, duration: 0.2, type: 'sine' as OscillatorType }, // G5
      { freq: 1046.5, duration: 0.3, type: 'sine' as OscillatorType }, // C6
      { freq: 1318.5, duration: 0.3, type: 'sine' as OscillatorType }, // E6
      { freq: 1567.98, duration: 0.4, type: 'sine' as OscillatorType }, // G6
    ];

    sequence.forEach((note, i) => {
      setTimeout(() => {
        this.playTone(note.freq, note.duration, note.type, 0.08 + i * 0.02);
      }, i * 150);
    });

    for (let i = 0; i < 8; i++) {
      setTimeout(
        () => {
          this.playTone(800 + Math.random() * 400, 0.1, 'triangle', 0.03);
        },
        900 + i * 100,
      );
    }
  }

  async gameLoss() {
    const melody = [523.25, 493.88, 440, 392]; // C5-B4-A4-G4 descending
    melody.forEach((freq, i) => {
      setTimeout(() => this.playTone(freq, 0.4, 'sine', 0.08), i * 150);
    });
  }

  async aiThinking() {
    for (let i = 0; i < 3; i++) {
      setTimeout(() => {
        this.playTone(400 + i * 50, 0.1, 'sine', 0.03);
      }, i * 300);
    }
  }

  async buttonClick() {
    await this.playTone(800, 0.1, 'square', 0.05);
  }

  toggle() {
    this.enabled = !this.enabled;
    return this.enabled;
  }

  setEnabled(enabled: boolean) {
    this.enabled = enabled;
  }

  get isEnabled() {
    return this.enabled;
  }
}

export const soundEffects = new SoundEffects();
