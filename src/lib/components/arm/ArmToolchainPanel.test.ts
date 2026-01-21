/**
 * Tests for ArmToolchainPanel Component
 * 
 * This file contains unit tests for the ARM Toolchain Panel component.
 * **Validates: Requirement 16**
 */

import { describe, it, expect, beforeEach, afterEach } from 'vitest';
import { render, screen, cleanup } from '@testing-library/svelte';
import userEvent from '@testing-library/user-event';
import ArmToolchainPanel from './ArmToolchainPanel.svelte';
import { armToolchainStore } from '$lib/stores/armToolchain';
import { complianceStore } from '$lib/stores/compliance';

describe('ArmToolchainPanel', () => {
  beforeEach(() => {
    // Reset stores before each test
    armToolchainStore.toolchains.set([]);
    armToolchainStore.selectedToolchain.set(null);
    armToolchainStore.loading.set(false);
    armToolchainStore.error.set(null);
    
    complianceStore.complianceModes.set(new Set());
    complianceStore.traceabilityMatrix.set(null);
    complianceStore.coverageReport.set(null);
    complianceStore.loading.set(false);
    complianceStore.error.set(null);
  });
  
  afterEach(() => {
    cleanup();
  });

  /**
   * Test: Tabs render correctly
   * **Validates: Requirement 16 - Task 1.8**
   * 
   * Verifies that all five tabs (Settings, Build, Visualizer, Compliance, Memory)
   * are rendered with correct labels and icons.
   */
  it('should render all tabs with correct labels and icons', () => {
    render(ArmToolchainPanel);
    
    // Check that all tabs are present
    expect(screen.getByRole('tab', { name: /Settings/i })).toBeTruthy();
    expect(screen.getByRole('tab', { name: /Build/i })).toBeTruthy();
    expect(screen.getByRole('tab', { name: /Visualizer/i })).toBeTruthy();
    expect(screen.getByRole('tab', { name: /Compliance/i })).toBeTruthy();
    expect(screen.getByRole('tab', { name: /Memory/i })).toBeTruthy();
    
    // Check that icons are present (emojis)
    const settingsTab = screen.getByRole('tab', { name: /Settings/i });
    expect(settingsTab.textContent).toContain('⚙️');
    
    const buildTab = screen.getByRole('tab', { name: /Build/i });
    expect(buildTab.textContent).toContain('🔨');
    
    const visualizerTab = screen.getByRole('tab', { name: /Visualizer/i });
    expect(visualizerTab.textContent).toContain('👁️');
    
    const complianceTab = screen.getByRole('tab', { name: /Compliance/i });
    expect(complianceTab.textContent).toContain('✓');
    
    const memoryTab = screen.getByRole('tab', { name: /Memory/i });
    expect(memoryTab.textContent).toContain('📊');
  });

  /**
   * Test: Tab switching updates active tab
   * **Validates: Requirement 16 - Task 1.9**
   * 
   * Verifies that clicking on a tab updates the active tab state and displays
   * the corresponding content.
   */
  it('should switch active tab when clicked', async () => {
    const user = userEvent.setup();
    render(ArmToolchainPanel);
    
    // Initially, Settings tab should be active
    const settingsTab = screen.getByRole('tab', { name: /Settings/i });
    expect(settingsTab.getAttribute('aria-selected')).toBe('true');
    expect(screen.getByText(/Toolchain Settings/i)).toBeTruthy();
    
    // Click on Build tab
    const buildTab = screen.getByRole('tab', { name: /Build/i });
    await user.click(buildTab);
    
    // Build tab should now be active
    expect(buildTab.getAttribute('aria-selected')).toBe('true');
    expect(settingsTab.getAttribute('aria-selected')).toBe('false');
    expect(screen.getByText(/Build Panel/i)).toBeTruthy();
    
    // Click on Visualizer tab
    const visualizerTab = screen.getByRole('tab', { name: /Visualizer/i });
    await user.click(visualizerTab);
    
    // Visualizer tab should now be active
    expect(visualizerTab.getAttribute('aria-selected')).toBe('true');
    expect(buildTab.getAttribute('aria-selected')).toBe('false');
    expect(screen.getByText(/Compiler Visualizer/i)).toBeTruthy();
    
    // Click on Compliance tab
    const complianceTab = screen.getByRole('tab', { name: /Compliance/i });
    await user.click(complianceTab);
    
    // Compliance tab should now be active
    expect(complianceTab.getAttribute('aria-selected')).toBe('true');
    expect(visualizerTab.getAttribute('aria-selected')).toBe('false');
    expect(screen.getByText(/Compliance Panel/i)).toBeTruthy();
    
    // Click on Memory tab
    const memoryTab = screen.getByRole('tab', { name: /Memory/i });
    await user.click(memoryTab);
    
    // Memory tab should now be active
    expect(memoryTab.getAttribute('aria-selected')).toBe('true');
    expect(complianceTab.getAttribute('aria-selected')).toBe('false');
    expect(screen.getByText(/Memory Map Viewer/i)).toBeTruthy();
  });

  /**
   * Test: Keyboard navigation with Tab key
   * **Validates: Requirement 16 - Task 1.5**
   * 
   * Verifies that pressing Tab key cycles through tabs forward,
   * and Shift+Tab cycles backward.
   */
  it('should navigate tabs with Tab and Shift+Tab keys', async () => {
    const user = userEvent.setup();
    const { container } = render(ArmToolchainPanel);
    
    // Focus the panel
    const panel = container.querySelector('.arm-toolchain-panel') as HTMLElement;
    panel?.focus();
    
    // Initially on Settings tab
    expect(screen.getByRole('tab', { name: /Settings/i }).getAttribute('aria-selected')).toBe('true');
    
    // Press Tab to go to Build
    await user.keyboard('{Tab}');
    expect(screen.getByRole('tab', { name: /Build/i }).getAttribute('aria-selected')).toBe('true');
    
    // Press Tab to go to Visualizer
    await user.keyboard('{Tab}');
    expect(screen.getByRole('tab', { name: /Visualizer/i }).getAttribute('aria-selected')).toBe('true');
    
    // Press Shift+Tab to go back to Build
    await user.keyboard('{Shift>}{Tab}{/Shift}');
    expect(screen.getByRole('tab', { name: /Build/i }).getAttribute('aria-selected')).toBe('true');
    
    // Press Shift+Tab to go back to Settings
    await user.keyboard('{Shift>}{Tab}{/Shift}');
    expect(screen.getByRole('tab', { name: /Settings/i }).getAttribute('aria-selected')).toBe('true');
  });

  /**
   * Test: Keyboard navigation with Arrow keys
   * **Validates: Requirement 16 - Task 1.5**
   * 
   * Verifies that arrow keys can be used to navigate between tabs.
   */
  it('should navigate tabs with Arrow keys', async () => {
    const user = userEvent.setup();
    const { container } = render(ArmToolchainPanel);
    
    // Focus the panel
    const panel = container.querySelector('.arm-toolchain-panel') as HTMLElement;
    panel?.focus();
    
    // Initially on Settings tab
    expect(screen.getByRole('tab', { name: /Settings/i }).getAttribute('aria-selected')).toBe('true');
    
    // Press ArrowRight to go to Build
    await user.keyboard('{ArrowRight}');
    expect(screen.getByRole('tab', { name: /Build/i }).getAttribute('aria-selected')).toBe('true');
    
    // Press ArrowRight to go to Visualizer
    await user.keyboard('{ArrowRight}');
    expect(screen.getByRole('tab', { name: /Visualizer/i }).getAttribute('aria-selected')).toBe('true');
    
    // Press ArrowLeft to go back to Build
    await user.keyboard('{ArrowLeft}');
    expect(screen.getByRole('tab', { name: /Build/i }).getAttribute('aria-selected')).toBe('true');
    
    // Press ArrowLeft to go back to Settings
    await user.keyboard('{ArrowLeft}');
    expect(screen.getByRole('tab', { name: /Settings/i }).getAttribute('aria-selected')).toBe('true');
  });

  /**
   * Test: Keyboard shortcuts (number keys)
   * **Validates: Requirement 16 - Task 1.5**
   * 
   * Verifies that pressing number keys 1-5 directly switches to the corresponding tab.
   */
  it('should switch tabs with number key shortcuts', async () => {
    const user = userEvent.setup();
    const { container } = render(ArmToolchainPanel);
    
    // Focus the panel
    const panel = container.querySelector('.arm-toolchain-panel') as HTMLElement;
    panel?.focus();
    
    // Press 2 to go to Build tab
    await user.keyboard('2');
    expect(screen.getByRole('tab', { name: /Build/i }).getAttribute('aria-selected')).toBe('true');
    
    // Press 4 to go to Compliance tab
    await user.keyboard('4');
    expect(screen.getByRole('tab', { name: /Compliance/i }).getAttribute('aria-selected')).toBe('true');
    
    // Press 1 to go to Settings tab
    await user.keyboard('1');
    expect(screen.getByRole('tab', { name: /Settings/i }).getAttribute('aria-selected')).toBe('true');
    
    // Press 5 to go to Memory tab
    await user.keyboard('5');
    expect(screen.getByRole('tab', { name: /Memory/i }).getAttribute('aria-selected')).toBe('true');
    
    // Press 3 to go to Visualizer tab
    await user.keyboard('3');
    expect(screen.getByRole('tab', { name: /Visualizer/i }).getAttribute('aria-selected')).toBe('true');
  });

  /**
   * Test: Loading overlay displays when stores are loading
   * **Validates: Requirement 16 - Task 1.6**
   * 
   * Verifies that the loading overlay is displayed when either
   * armToolchainStore or complianceStore is in loading state.
   */
  it('should display loading overlay when armToolchainStore is loading', () => {
    // Set loading state
    armToolchainStore.loading.set(true);
    
    render(ArmToolchainPanel);
    
    // Loading overlay should be visible
    expect(screen.getByText(/Loading\.\.\./i)).toBeTruthy();
    const loadingOverlay = screen.getByText(/Loading\.\.\./i).closest('.loading-overlay');
    expect(loadingOverlay).toBeTruthy();
    expect(loadingOverlay?.getAttribute('aria-live')).toBe('polite');
    expect(loadingOverlay?.getAttribute('aria-busy')).toBe('true');
  });

  it('should display loading overlay when complianceStore is loading', () => {
    // Set loading state
    complianceStore.loading.set(true);
    
    render(ArmToolchainPanel);
    
    // Loading overlay should be visible
    expect(screen.getByText(/Loading\.\.\./i)).toBeTruthy();
    const loadingOverlay = screen.getByText(/Loading\.\.\./i).closest('.loading-overlay');
    expect(loadingOverlay).toBeTruthy();
    expect(loadingOverlay?.getAttribute('aria-live')).toBe('polite');
    expect(loadingOverlay?.getAttribute('aria-busy')).toBe('true');
  });

  it('should hide loading overlay when stores are not loading', () => {
    // Ensure both stores are not loading
    armToolchainStore.loading.set(false);
    complianceStore.loading.set(false);
    
    render(ArmToolchainPanel);
    
    // Loading overlay should not be visible
    expect(screen.queryByText(/Loading\.\.\./i)).toBeNull();
  });

  /**
   * Test: Tab content displays correctly
   * **Validates: Requirement 16 - Task 1.3**
   * 
   * Verifies that the correct placeholder content is displayed for each tab.
   */
  it('should display correct content for each tab', async () => {
    const user = userEvent.setup();
    render(ArmToolchainPanel);
    
    // Settings tab content
    expect(screen.getByText(/Toolchain Settings/i)).toBeTruthy();
    expect(screen.getByText(/Configure ARM toolchain and MCU settings/i)).toBeTruthy();
    
    // Build tab content
    await user.click(screen.getByRole('tab', { name: /Build/i }));
    expect(screen.getByText(/Build Panel/i)).toBeTruthy();
    expect(screen.getByText(/Compile, link, and generate binaries/i)).toBeTruthy();
    
    // Visualizer tab content
    await user.click(screen.getByRole('tab', { name: /Visualizer/i }));
    expect(screen.getByText(/Compiler Visualizer/i)).toBeTruthy();
    expect(screen.getByText(/View preprocessor, assembly, and disassembly output/i)).toBeTruthy();
    
    // Compliance tab content
    await user.click(screen.getByRole('tab', { name: /Compliance/i }));
    expect(screen.getByText(/Compliance Panel/i)).toBeTruthy();
    expect(screen.getByText(/Manage DO-178C, DO-330, and ARP4754A compliance/i)).toBeTruthy();
    
    // Memory tab content
    await user.click(screen.getByRole('tab', { name: /Memory/i }));
    expect(screen.getByText(/Memory Map Viewer/i)).toBeTruthy();
    expect(screen.getByText(/Visualize memory layout and usage/i)).toBeTruthy();
  });

  /**
   * Test: Accessibility - ARIA attributes
   * **Validates: Requirement 20 - Accessibility**
   * 
   * Verifies that proper ARIA attributes are set for accessibility.
   */
  it('should have proper ARIA attributes for accessibility', () => {
    render(ArmToolchainPanel);
    
    // Tab list should have proper role and label
    const tablist = screen.getByRole('tablist', { name: /ARM Toolchain tabs/i });
    expect(tablist).toBeTruthy();
    
    // Each tab should have proper ARIA attributes
    const tabs = screen.getAllByRole('tab');
    expect(tabs.length).toBe(5);
    
    tabs.forEach((tab, index) => {
      // Should have aria-selected attribute
      expect(tab.getAttribute('aria-selected')).toBeDefined();
      
      // Should have aria-controls attribute
      expect(tab.getAttribute('aria-controls')).toBeDefined();
      
      // Should have id attribute
      expect(tab.getAttribute('id')).toBeDefined();
      
      // Active tab should have tabindex 0, others -1
      if (index === 0) {
        expect(tab.getAttribute('tabindex')).toBe('0');
      } else {
        expect(tab.getAttribute('tabindex')).toBe('-1');
      }
    });
    
    // Tab panel content should have proper role and attributes
    const tabpanels = screen.getAllByRole('tabpanel');
    // There should be 2 tabpanels: the outer container and the inner content
    expect(tabpanels.length).toBe(2);
    
    // Find the inner tabpanel (the one with aria-labelledby)
    const innerTabpanel = tabpanels.find(panel => panel.getAttribute('aria-labelledby'));
    expect(innerTabpanel).toBeTruthy();
    expect(innerTabpanel?.getAttribute('aria-labelledby')).toBeDefined();
  });

  /**
   * Test: Tab wrapping behavior
   * **Validates: Requirement 16 - Task 1.5**
   * 
   * Verifies that tab navigation wraps around at the ends.
   */
  it('should wrap to first tab when navigating forward from last tab', async () => {
    const user = userEvent.setup();
    const { container } = render(ArmToolchainPanel);
    
    // Focus the panel and navigate to last tab
    const panel = container.querySelector('.arm-toolchain-panel') as HTMLElement;
    panel?.focus();
    
    // Navigate to Memory tab (last tab)
    await user.keyboard('5');
    expect(screen.getByRole('tab', { name: /Memory/i }).getAttribute('aria-selected')).toBe('true');
    
    // Press ArrowRight to wrap to first tab
    await user.keyboard('{ArrowRight}');
    expect(screen.getByRole('tab', { name: /Settings/i }).getAttribute('aria-selected')).toBe('true');
  });

  it('should wrap to last tab when navigating backward from first tab', async () => {
    const user = userEvent.setup();
    const { container } = render(ArmToolchainPanel);
    
    // Focus the panel (starts on Settings tab)
    const panel = container.querySelector('.arm-toolchain-panel') as HTMLElement;
    panel?.focus();
    
    // Press ArrowLeft to wrap to last tab
    await user.keyboard('{ArrowLeft}');
    expect(screen.getByRole('tab', { name: /Memory/i }).getAttribute('aria-selected')).toBe('true');
  });
});
