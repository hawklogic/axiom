<!-- SPDX-License-Identifier: Apache-2.0 -->
<!-- Copyright 2024 HawkLogic Systems -->
<script lang="ts">
  import { APP, SPLASH } from '$lib/strings';
</script>

<div class="splash">
  <div class="splash-content">
    <h1 class="logo">{APP.name}</h1>
    <p class="tagline">{APP.tagline}</p>
    <p class="loading">{SPLASH.loading}</p>
  </div>
</div>

<style>
  .splash {
    position: fixed;
    inset: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    background: #1a1a1a;
    z-index: 9999;
  }

  .splash-content {
    text-align: center;
  }

  .logo {
    font-family: var(--font-sans);
    font-size: 48px;
    font-weight: 300;
    letter-spacing: 8px;
    color: #ffffff;
    margin-bottom: var(--space-md);
    text-transform: uppercase;
  }

  .tagline {
    font-size: var(--font-size-sm);
    color: var(--color-text-muted);
    letter-spacing: 2px;
    margin-bottom: var(--space-xl);
  }

  .loading {
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
  }
</style>
