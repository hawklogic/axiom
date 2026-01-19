<script lang="ts">
  /**
   * Autocomplete UI Component
   * 
   * Displays autocomplete suggestions in a dropdown near the cursor.
   * Handles visual presentation and user interaction.
   */
  
  import type { Suggestion, Position } from '$lib/utils/autocomplete';
  
  // Props
  export let visible: boolean = false;
  export let suggestions: Suggestion[] = [];
  export let activeIndex: number = 0;
  export let position: Position = { x: 0, y: 0 };
  export let onSelect: (suggestion: Suggestion) => void = () => {};
  export let onDismiss: () => void = () => {};
  
  // Handle click outside to dismiss
  function handleClickOutside(event: MouseEvent) {
    const target = event.target as HTMLElement;
    const dropdown = document.querySelector('.autocomplete-dropdown');
    
    if (dropdown && !dropdown.contains(target)) {
      onDismiss();
    }
  }
  
  // Handle suggestion click
  function handleSuggestionClick(suggestion: Suggestion) {
    onSelect(suggestion);
  }
  
  // Get color for entry type
  function getTypeColor(type: string): string {
    const colors: Record<string, string> = {
      'keyword': '#569cd6',    // Blue
      'function': '#dcdcaa',   // Yellow
      'type': '#4ec9b0',       // Teal
      'constant': '#b5cea8',   // Light green
      'variable': '#9cdcfe'    // Light blue
    };
    return colors[type] || '#d4d4d4';
  }
  
  // Reactive statement to add/remove click listener
  $: if (visible) {
    setTimeout(() => {
      document.addEventListener('click', handleClickOutside);
    }, 0);
  } else {
    document.removeEventListener('click', handleClickOutside);
  }
</script>

{#if visible && suggestions.length > 0}
  <div 
    class="autocomplete-dropdown"
    style="left: {position.x}px; top: {position.y}px;"
  >
    <ul class="suggestion-list">
      {#each suggestions as suggestion, index}
        <li 
          class="suggestion-item"
          class:active={index === activeIndex}
          on:click={() => handleSuggestionClick(suggestion)}
          on:keydown={(e) => e.key === 'Enter' && handleSuggestionClick(suggestion)}
          role="option"
          aria-selected={index === activeIndex}
          tabindex="-1"
        >
          <span 
            class="suggestion-text"
            style="color: {getTypeColor(suggestion.type)}"
          >
            {suggestion.text}
          </span>
          {#if suggestion.description}
            <span class="suggestion-description">
              {suggestion.description}
            </span>
          {/if}
        </li>
      {/each}
    </ul>
  </div>
{/if}

<style>
  .autocomplete-dropdown {
    position: fixed;
    z-index: 1000;
    background: #1e1e1e;
    border: 1px solid #3c3c3c;
    border-radius: 4px;
    box-shadow: 0 4px 8px rgba(0, 0, 0, 0.3);
    max-height: 300px;
    overflow-y: auto;
    font-family: 'Consolas', 'Monaco', 'Courier New', monospace;
    font-size: 13px;
    animation: fadeIn 0.1s ease-in;
  }
  
  @keyframes fadeIn {
    from {
      opacity: 0;
      transform: translateY(-4px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }
  
  .suggestion-list {
    list-style: none;
    margin: 0;
    padding: 4px 0;
  }
  
  .suggestion-item {
    padding: 4px 12px;
    cursor: pointer;
    display: flex;
    align-items: center;
    gap: 8px;
    white-space: nowrap;
    transition: background-color 0.1s ease;
  }
  
  .suggestion-item:hover {
    background: #2a2d2e;
  }
  
  .suggestion-item.active {
    background: #094771;
  }
  
  .suggestion-text {
    font-weight: 500;
  }
  
  .suggestion-description {
    color: #858585;
    font-size: 11px;
    margin-left: auto;
    max-width: 200px;
    overflow: hidden;
    text-overflow: ellipsis;
  }
  
  /* Scrollbar styling */
  .autocomplete-dropdown::-webkit-scrollbar {
    width: 8px;
  }
  
  .autocomplete-dropdown::-webkit-scrollbar-track {
    background: #1e1e1e;
  }
  
  .autocomplete-dropdown::-webkit-scrollbar-thumb {
    background: #424242;
    border-radius: 4px;
  }
  
  .autocomplete-dropdown::-webkit-scrollbar-thumb:hover {
    background: #4e4e4e;
  }
</style>
