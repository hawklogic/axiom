<!-- SPDX-License-Identifier: Apache-2.0 -->
<!-- Copyright 2024 HawkLogic Systems -->
<script lang="ts">
  import { APP, ABOUT } from '$lib/strings';
  
  const version = '0.1.0';
  const commitHash = 'dev';
  const buildDate = new Date().toISOString().split('T')[0];
</script>

<div class="about-panel">
  <div class="logo-section">
    <div class="logo-mark">A</div>
    <h1 class="logo-text">{APP.name}</h1>
  </div>
  
  <div class="info-section">
    <div class="info-row">
      <span class="label">{ABOUT.version}</span>
      <span class="value">{version}</span>
    </div>
    <div class="info-row">
      <span class="label">Build</span>
      <span class="value mono">{commitHash} ({buildDate})</span>
    </div>
  </div>
  
  <p class="tagline">{APP.tagline}</p>
  
  <p class="philosophy">{ABOUT.philosophy}</p>
  
  <div class="steward">
    <a href="https://hawklogicsystems.com/" target="_blank" rel="noopener noreferrer" class="company-link">
      {APP.steward}
    </a>
  </div>
  
  <div class="links">
    <button class="link-btn">{ABOUT.viewLicense}</button>
    <button class="link-btn">{ABOUT.viewPhilosophy}</button>
  </div>
  
  <footer class="copyright">
    <span>{APP.copyright}</span>
    <span class="license">{APP.license}</span>
  </footer>
</div>

<style>
  .about-panel {
    padding: var(--space-xl);
    max-width: 400px;
    margin: 0 auto;
    text-align: center;
  }

  .logo-section {
    margin-bottom: var(--space-xl);
  }

  .logo-mark {
    width: 80px;
    height: 80px;
    margin: 0 auto var(--space-md);
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 48px;
    font-weight: 300;
    color: var(--color-text-primary);
    border: 2px solid var(--color-border);
    border-radius: 16px;
  }

  .logo-text {
    font-size: 32px;
    font-weight: 300;
    letter-spacing: 8px;
    color: var(--color-text-primary);
    margin: 0;
  }

  .info-section {
    background: var(--color-bg-tertiary);
    border-radius: var(--radius-md);
    padding: var(--space-md);
    margin-bottom: var(--space-lg);
  }

  .info-row {
    display: flex;
    justify-content: space-between;
    font-size: var(--font-size-sm);
    padding: var(--space-xs) 0;
  }

  .label {
    color: var(--color-text-muted);
  }

  .value {
    color: var(--color-text-primary);
  }

  .mono {
    font-family: var(--font-mono);
  }

  .tagline {
    font-size: var(--font-size-sm);
    color: var(--color-text-muted);
    letter-spacing: 1px;
    margin-bottom: var(--space-lg);
  }

  .philosophy {
    font-size: var(--font-size-sm);
    color: var(--color-text-secondary);
    line-height: 1.6;
    margin-bottom: var(--space-lg);
  }

  .steward {
    font-size: var(--font-size-sm);
    margin-bottom: var(--space-lg);
  }
  
  .company-link {
    color: var(--color-accent);
    text-decoration: none;
    transition: opacity 0.2s;
  }
  
  .company-link:hover {
    opacity: 0.8;
    text-decoration: underline;
  }

  .links {
    display: flex;
    gap: var(--space-md);
    justify-content: center;
    margin-bottom: var(--space-xl);
  }

  .link-btn {
    padding: var(--space-sm) var(--space-md);
    font-size: var(--font-size-sm);
    color: var(--color-text-secondary);
    background: var(--color-bg-tertiary);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-sm);
    cursor: pointer;
    transition: border-color var(--transition-fast);
  }

  .link-btn:hover {
    border-color: var(--color-accent);
    color: var(--color-text-primary);
  }

  .copyright {
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
    display: flex;
    flex-direction: column;
    gap: var(--space-xs);
  }

  .license {
    color: var(--color-text-muted);
  }
</style>
