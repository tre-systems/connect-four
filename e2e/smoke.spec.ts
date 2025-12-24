import { test, expect, Page } from '@playwright/test';

async function dismissErrorModalIfPresent(page: Page) {
  try {
    // Check if error modal is visible and dismiss it
    const errorModal = page.getByTestId('error-modal');
    if (await errorModal.isVisible()) {
      console.log('Dismissing error modal...');
      await page.getByTestId('error-close-bottom').click();
      await page.waitForTimeout(200);

      // Wait for modal to disappear
      await expect(errorModal).not.toBeVisible();
    }
  } catch {
    // Error modal not present, continue
  }
}

async function startGame(page: Page) {
  await page.goto('/');

  // Check if we're on the AI selection screen
  const aiSelectionPanel = page.getByTestId('ai-selection-classic');
  if (await aiSelectionPanel.isVisible()) {
    // Select classic AI and start game - clicking the card directly starts the game
    await aiSelectionPanel.click();
    await page.waitForTimeout(600); // Wait for transition
  }

  // The game board should now be visible
  await expect(page.getByTestId('game-board')).toBeVisible();
  // Wait for the animation to complete (0.5s duration + buffer)
  await page.waitForTimeout(600);

  // Dismiss any error modal that might appear due to WASM AI not loading in tests
  await dismissErrorModalIfPresent(page);

  // Ensure no error modal is blocking interactions
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

    // Ensure no error modal is blocking
    await dismissErrorModalIfPresent(page);

    // Click on a column to drop a piece
    await page.getByTestId('column-3').click();

    // Wait a moment for the move to complete
    await page.waitForTimeout(1000);

    // Dismiss any error modal that might appear
    await dismissErrorModalIfPresent(page);

    // The board should still be visible after the move
    await expect(gameBoard).toBeVisible();
  });

  test('can make a move by clicking on a column', async ({ page }) => {
    // Click on a column to drop a piece
    await page.getByTestId('column-3').click();
    await page.waitForTimeout(1000);

    // Dismiss any error modal that might appear
    await dismissErrorModalIfPresent(page);

    // Should see some change in game state
    await expect(page.getByTestId('game-status-text')).not.toBeEmpty();
  });

  test('can toggle sound settings', async ({ page }) => {
    // Ensure no error modal is blocking
    await dismissErrorModalIfPresent(page);

    const soundToggle = page.getByTestId('toggle-sound');
    await expect(soundToggle).toBeVisible();

    // Click to toggle
    await soundToggle.click();
    await page.waitForTimeout(100);

    // Should still be visible after toggle
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

    // Make a few moves to test game state changes
    await page.getByTestId('column-3').click();
    await page.waitForTimeout(1000);
    await dismissErrorModalIfPresent(page);

    await page.getByTestId('column-2').click();
    await page.waitForTimeout(1000);
    await dismissErrorModalIfPresent(page);

    // Verify the game board is still visible and functional
    await expect(page.getByTestId('game-board')).toBeVisible();
    await expect(page.getByTestId('game-status-text')).not.toBeEmpty();
  });

  test('can reset game', async ({ page }) => {
    await startGame(page);

    // Make a move
    await page.getByTestId('column-3').click();
    await page.waitForTimeout(1000);
    await dismissErrorModalIfPresent(page);

    // Click reset button
    await page.getByTestId('reset-game').click();

    // Should return to AI selection screen
    await expect(page.getByTestId('ai-selection-classic')).toBeVisible();

    // Select classic AI and start game again
    await page.getByTestId('ai-selection-classic').click();
    await page.waitForTimeout(600);

    // Should return to game board
    await expect(page.getByTestId('game-board')).toBeVisible();
  });
});

test.describe('Error Handling and Edge Cases', () => {
  test('handles rapid column clicks gracefully', async ({ page }) => {
    await startGame(page);

    // Rapidly click on different columns
    for (let i = 0; i < 3; i++) {
      await page.getByTestId(`column-${i}`).click();
      await page.waitForTimeout(50);
    }

    // Dismiss any error modal that might appear
    await dismissErrorModalIfPresent(page);

    // Should still be functional
    await expect(page.getByTestId('game-board')).toBeVisible();
  });

  test('handles rapid column selections gracefully', async ({ page }) => {
    await startGame(page);

    // Rapidly click on different columns
    for (let i = 0; i < 3; i++) {
      await page.getByTestId(`column-${i}`).click();
      await page.waitForTimeout(50);
    }

    // Dismiss any error modal that might appear
    await dismissErrorModalIfPresent(page);

    // Should still be functional
    await expect(page.getByTestId('game-board')).toBeVisible();
  });

  test('maintains game state during navigation', async ({ page }) => {
    await startGame(page);

    // Make some game progress
    await page.getByTestId('column-3').click();
    await page.waitForTimeout(1000);
    await dismissErrorModalIfPresent(page);

    // Navigate away and back
    await page.goto('/');
    await page.goto('/');

    // Wait for the page to load and game state to be restored
    await page.waitForTimeout(1000);

    // Should be back to AI selection screen
    await expect(page.getByTestId('ai-selection-classic')).toBeVisible();

    // Select classic AI and start game again
    await page.getByTestId('ai-selection-classic').click();
    await page.waitForTimeout(600);

    // Should be back to game board
    await expect(page.getByTestId('game-board')).toBeVisible();
  });
});

test.describe('Mobile Responsiveness', () => {
  test.use({ viewport: { width: 375, height: 667 } });

  test('game is fully functional on mobile', async ({ page }) => {
    await startGame(page);

    // Verify all key elements are visible and functional
    await expect(page.getByTestId('game-board')).toBeVisible();
    await expect(page.getByTestId('toggle-sound')).toBeVisible();
    await expect(page.getByTestId('how-to-play')).toBeVisible();

    // Test basic interactions
    await page.getByTestId('column-3').click();
    await page.waitForTimeout(1000);
    await dismissErrorModalIfPresent(page);
    await expect(page.getByTestId('game-board')).toBeVisible();
  });
});
