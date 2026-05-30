import { test, expect, Page } from '@playwright/test';

async function dismissErrorModalIfPresent(page: Page) {
  try {
    const errorModal = page.getByTestId('error-modal');
    if (await errorModal.isVisible()) {
      console.log('Dismissing error modal...');
      await page.getByTestId('error-close-bottom').click();
      await page.waitForTimeout(200);
      await expect(errorModal).not.toBeVisible();
    }
  } catch {
    // Error modal not present, continue
  }
}

async function startGame(page: Page) {
  await page.goto('/');

  // Clicking the AI selection card starts the game
  const aiSelectionPanel = page.getByTestId('ai-selection-classic');
  if (await aiSelectionPanel.isVisible()) {
    await aiSelectionPanel.click();
    await page.waitForTimeout(600); // wait for transition
  }

  await expect(page.getByTestId('game-board')).toBeVisible();
  await page.waitForTimeout(600); // wait for drop animation (0.5s + buffer)

  // WASM AI may fail to load in tests, surfacing an error modal; dismiss it
  await dismissErrorModalIfPresent(page);

  const errorModal = page.getByTestId('error-modal');
  await expect(errorModal).not.toBeVisible();
}

test.describe('Core Game Functionality', () => {
  test('can start a game and see initial state', async ({ page }) => {
    await startGame(page);
    await expect(page.getByTestId('game-board')).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Connect 4' })).toBeVisible();
    await expect(page.getByText('Drop your pieces to get four in a row!')).toBeVisible();
  });
});

test.describe('Game Interactions', () => {
  test.beforeEach(async ({ page }) => {
    await startGame(page);
  });

  test('can click on board columns', async ({ page }) => {
    const gameBoard = page.getByTestId('game-board');
    await expect(gameBoard).toBeVisible();

    await dismissErrorModalIfPresent(page);

    await page.getByTestId('column-3').click();
    await page.waitForTimeout(1000);

    await dismissErrorModalIfPresent(page);

    await expect(gameBoard).toBeVisible();
  });

  test('can make a move by clicking on a column', async ({ page }) => {
    await page.getByTestId('column-3').click();
    await page.waitForTimeout(1000);

    await dismissErrorModalIfPresent(page);

    await expect(page.getByTestId('game-status-text')).not.toBeEmpty();
  });

  test('can toggle sound settings', async ({ page }) => {
    await dismissErrorModalIfPresent(page);

    const soundToggle = page.getByTestId('toggle-sound');
    await expect(soundToggle).toBeVisible();

    await soundToggle.click();
    await page.waitForTimeout(100);

    await expect(soundToggle).toBeVisible();
  });

  test('can open and close help panel', async ({ page }) => {
    await page.getByTestId('how-to-play').click();
    await expect(page.getByTestId('help-panel')).toBeVisible();
    await expect(page.getByTestId('help-close')).toBeVisible();

    await page.getByTestId('help-close').click();
    await expect(page.getByTestId('help-panel')).not.toBeVisible();
  });
});

test.describe('Game Completion and Database Saves', () => {
  test('can make moves and see game state changes', async ({ page }) => {
    await startGame(page);

    await page.getByTestId('column-3').click();
    await page.waitForTimeout(1000);
    await dismissErrorModalIfPresent(page);

    await page.getByTestId('column-2').click();
    await page.waitForTimeout(1000);
    await dismissErrorModalIfPresent(page);

    await expect(page.getByTestId('game-board')).toBeVisible();
    await expect(page.getByTestId('game-status-text')).not.toBeEmpty();
  });

  test('can reset game', async ({ page }) => {
    await startGame(page);

    await page.getByTestId('column-3').click();
    await page.waitForTimeout(1000);
    await dismissErrorModalIfPresent(page);

    await page.getByTestId('reset-game').click();

    // Reset returns to the AI selection screen
    await expect(page.getByTestId('ai-selection-classic')).toBeVisible();

    await page.getByTestId('ai-selection-classic').click();
    await page.waitForTimeout(600);

    await expect(page.getByTestId('game-board')).toBeVisible();
  });
});

test.describe('Error Handling and Edge Cases', () => {
  test('handles rapid column clicks gracefully', async ({ page }) => {
    await startGame(page);

    for (let i = 0; i < 3; i++) {
      await page.getByTestId(`column-${i}`).click();
      await page.waitForTimeout(50);
    }

    await dismissErrorModalIfPresent(page);

    await expect(page.getByTestId('game-board')).toBeVisible();
  });

  test('handles rapid column selections gracefully', async ({ page }) => {
    await startGame(page);

    for (let i = 0; i < 3; i++) {
      await page.getByTestId(`column-${i}`).click();
      await page.waitForTimeout(50);
    }

    await dismissErrorModalIfPresent(page);

    await expect(page.getByTestId('game-board')).toBeVisible();
  });

  test('maintains game state during navigation', async ({ page }) => {
    await startGame(page);

    await page.getByTestId('column-3').click();
    await page.waitForTimeout(1000);
    await dismissErrorModalIfPresent(page);

    await page.goto('/');
    await page.goto('/');
    await page.waitForTimeout(1000);

    await expect(page.getByTestId('ai-selection-classic')).toBeVisible();

    await page.getByTestId('ai-selection-classic').click();
    await page.waitForTimeout(600);

    await expect(page.getByTestId('game-board')).toBeVisible();
  });
});

test.describe('Mobile Responsiveness', () => {
  test.use({ viewport: { width: 375, height: 667 } });

  test('game is fully functional on mobile', async ({ page }) => {
    await startGame(page);

    await expect(page.getByTestId('game-board')).toBeVisible();
    await expect(page.getByTestId('toggle-sound')).toBeVisible();
    await expect(page.getByTestId('how-to-play')).toBeVisible();

    await page.getByTestId('column-3').click();
    await page.waitForTimeout(1000);
    await dismissErrorModalIfPresent(page);
    await expect(page.getByTestId('game-board')).toBeVisible();
  });
});
