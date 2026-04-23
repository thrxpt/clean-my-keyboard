<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  let isActive = $state(false);

  function toggle() {
    if (isActive) {
      invoke("unlock_keyboard");
      isActive = false;
    } else {
      invoke("lock_keyboard");
      isActive = true;
    }
  }
</script>

<main>
  <div class="content">
    <h1>Clean My Keyboard</h1>
    <p class="subtitle">Lock your keyboard while you wipe it down</p>

    <button
      id="toggle-btn"
      class="toggle"
      class:active={isActive}
      onclick={toggle}
    >
      <span class="icon">{isActive ? "■" : "▶"}</span>
    </button>

    <p class="status" class:active={isActive}>
      {isActive ? "Keyboard locked" : "Ready"}
    </p>
  </div>
</main>

<style>
  :global(body) {
    margin: 0;
    background-color: #0a0a0a;
    overflow: hidden;
    user-select: none;
  }

  main {
    height: 100vh;
    display: flex;
    align-items: center;
    justify-content: center;
    font-family:
      "Inter",
      "SF Pro Display",
      -apple-system,
      system-ui,
      sans-serif;
    color: #e0e0e0;
  }

  .content {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 8px;
  }

  h1 {
    font-size: 1.1rem;
    font-weight: 500;
    letter-spacing: 0.08em;
    text-transform: uppercase;
    color: #888;
    margin: 0;
  }

  .subtitle {
    font-size: 0.75rem;
    color: #555;
    margin: 0 0 32px;
    font-weight: 400;
  }

  .toggle {
    width: 120px;
    height: 120px;
    border-radius: 50%;
    border: 2px solid #222;
    background: #111;
    color: #666;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: all 0.4s cubic-bezier(0.4, 0, 0.2, 1);
    position: relative;
  }

  .toggle:hover {
    border-color: #333;
    background: #161616;
    color: #999;
  }

  .toggle.active {
    border-color: #e74c3c;
    background: rgba(231, 76, 60, 0.08);
    color: #e74c3c;
    animation: pulse 2s ease-in-out infinite;
  }

  .toggle.active:hover {
    background: rgba(231, 76, 60, 0.14);
  }

  .icon {
    font-size: 1.5rem;
    line-height: 1;
  }

  .status {
    font-size: 0.8rem;
    color: #444;
    margin: 24px 0 0;
    font-weight: 400;
    letter-spacing: 0.04em;
    transition: color 0.3s ease;
  }

  .status.active {
    color: #e74c3c;
  }

  @keyframes pulse {
    0%,
    100% {
      box-shadow: 0 0 0 0 rgba(231, 76, 60, 0.15);
    }
    50% {
      box-shadow: 0 0 0 20px rgba(231, 76, 60, 0);
    }
  }
</style>
