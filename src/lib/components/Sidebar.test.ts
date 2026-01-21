/**
 * Tests for Sidebar Component
 * 
 * This file contains unit tests for the Sidebar component.
 * **Validates: Requirement 16**
 */

import { describe, it, expect, beforeEach, afterEach } from 'vitest';
import { render, screen, cleanup } from '@testing-library/svelte';
import userEvent from '@testing-library/user-event';
import Sidebar from './Sidebar.svelte';
import { armToolchainStore } from '$lib/stores/armToolchain';

describe('Sidebar', () => {
  beforeEach(() => {
    // Reset stores before each test
    armToolchainStore.loading.set(false);
    armToolchainStore.error.set(null);
  });
  
  afterEach(() => {
    cleanup();
  });

  /**
   * Test: Build progress indicator appears when ARM toolchain is loading
   * **Validates: Requirement 16 - Task 2.4**
   * 
   * Verifies that a build progress indicator (spinner) is displayed on the ARM icon
   * when the armToolchainStore is in loading state.
   */
  it('should display build progress indicator on ARM icon when building', () => {
    // Set loading state to true
    armToolchainStore.loading.set(true);
    
    render(Sidebar, { props: { activePanel: 'files' } });
    
    // Find the ARM button
    const armButton = screen.getByRole('button', { name: /ARM Toolchain/i });
    expect(armButton).toBeTruthy();
    
    // Check that the build progress indicator is present
    const progressIndicator = armButton.querySelector('.build-progress-indicator');
    expect(progressIndicator).toBeTruthy();
    expect(progressIndicator?.getAttribute('aria-label')).toBe('Build in progress');
    
    // Check that the button has the 'building' class
    expect(armButton.classList.contains('building')).toBe(true);
    
    // Check that the spinner SVG is present
    const spinnerSvg = progressIndicator?.querySelector('svg');
    expect(spinnerSvg).toBeTruthy();
  });

  /**
   * Test: Build progress indicator is hidden when not building
   * **Validates: Requirement 16 - Task 2.4**
   * 
   * Verifies that the build progress indicator is not displayed when
   * the armToolchainStore is not in loading state.
   */
  it('should not display build progress indicator when not building', () => {
    // Ensure loading state is false
    armToolchainStore.loading.set(false);
    
    render(Sidebar, { props: { activePanel: 'files' } });
    
    // Find the ARM button
    const armButton = screen.getByRole('button', { name: /ARM Toolchain/i });
    expect(armButton).toBeTruthy();
    
    // Check that the build progress indicator is NOT present
    const progressIndicator = armButton.querySelector('.build-progress-indicator');
    expect(progressIndicator).toBeNull();
    
    // Check that the button does NOT have the 'building' class
    expect(armButton.classList.contains('building')).toBe(false);
  });

  /**
   * Test: Build progress indicator only appears on ARM icon
   * **Validates: Requirement 16 - Task 2.4**
   * 
   * Verifies that the build progress indicator only appears on the ARM icon,
   * not on other sidebar icons.
   */
  it('should only display build progress indicator on ARM icon', () => {
    // Set loading state to true
    armToolchainStore.loading.set(true);
    
    render(Sidebar, { props: { activePanel: 'files' } });
    
    // Check that only the ARM button has the progress indicator
    const allButtons = screen.getAllByRole('button');
    
    allButtons.forEach((button) => {
      const progressIndicator = button.querySelector('.build-progress-indicator');
      
      if (button.getAttribute('aria-label')?.includes('ARM')) {
        // ARM button should have the indicator
        expect(progressIndicator).toBeTruthy();
      } else {
        // Other buttons should NOT have the indicator
        expect(progressIndicator).toBeNull();
      }
    });
  });

  /**
   * Test: ARM icon appears in sidebar
   * **Validates: Requirement 16 - Task 2.5**
   * 
   * Verifies that the ARM/Embedded icon is rendered in the sidebar
   * and is accessible to users.
   */
  it('should display ARM icon in sidebar', () => {
    render(Sidebar, { props: { activePanel: 'files' } });
    
    // Find the ARM button by its aria-label
    const armButton = screen.getByRole('button', { name: /ARM Toolchain/i });
    expect(armButton).toBeTruthy();
    
    // Verify the button has the correct label
    expect(armButton.getAttribute('aria-label')).toBe('ARM Toolchain');
    
    // Verify the icon SVG is present
    const iconSpan = armButton.querySelector('.icon');
    expect(iconSpan).toBeTruthy();
    
    // Verify the SVG element exists within the icon
    const svg = iconSpan?.querySelector('svg');
    expect(svg).toBeTruthy();
    
    // Verify the SVG has the expected viewBox attribute (ARM icon specific)
    expect(svg?.getAttribute('viewBox')).toBe('0 0 24 24');
    
    // Verify the button is clickable (not disabled)
    expect(armButton.hasAttribute('disabled')).toBe(false);
  });

  /**
   * Test: Clicking ARM icon shows ARM panel
   * **Validates: Requirement 16 - Task 2.6**
   * 
   * Verifies that clicking the ARM icon in the sidebar updates the activePanel
   * to 'arm', which would trigger the ARM panel to be displayed.
   */
  it('should update activePanel to arm when ARM icon is clicked', async () => {
    const user = userEvent.setup();
    render(Sidebar, { props: { activePanel: 'files' } });
    
    // Find the ARM button
    const armButton = screen.getByRole('button', { name: /ARM Toolchain/i });
    
    // Initially, ARM button should not have the 'active' class
    expect(armButton.classList.contains('active')).toBe(false);
    
    // The files button should have the 'active' class
    const filesButton = screen.getByRole('button', { name: /Files/i });
    expect(filesButton.classList.contains('active')).toBe(true);
    
    // Click the ARM button
    await user.click(armButton);
    
    // After clicking, the ARM button should have the 'active' class
    expect(armButton.classList.contains('active')).toBe(true);
    
    // The files button should no longer have the 'active' class
    expect(filesButton.classList.contains('active')).toBe(false);
  });
});
