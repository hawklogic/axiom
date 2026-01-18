<!-- SPDX-License-Identifier: Apache-2.0 -->
<!-- Copyright 2024 HawkLogic Systems -->
<script lang="ts">
  import '../app.css';
  import { onMount } from 'svelte';
  import { settingsStore } from '$lib/stores/settings';

  onMount(() => {
    // Apply theme from settings
    const unsubscribe = settingsStore.subscribe(settings => {
      if (settings) {
        document.documentElement.setAttribute('data-theme', settings.ui.theme);
      }
    });

    return unsubscribe;
  });
</script>

<slot />
