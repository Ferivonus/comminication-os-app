<script>
  import { invoke } from "@tauri-apps/api";

  export let data;

  // Function to trigger a notification
  async function sendNotification() {
    try {
      await invoke("notify_frontend", { message: "Hello from frontend!" });
      console.log("Notification sent successfully!");
    } catch (e) {
      console.error("Failed to send notification:", e);
      alert("Failed to send notification. Check console for details.");
    }
  }
</script>

<button on:click={sendNotification}>Send Notification</button>

<nav>
  <a href="/">home</a>
  <a href="/about">about</a>
  <a href="/form">form</a>
  <a href="/messages">messages</a>
  <a href="/wailing-wall">wailing wall</a>
</nav>
<div class="container">
  <h1>Welcome to form page!</h1>

  <p>this is the form page.</p>

  <ul>
    {#each data.summaries as { slug, title }}
      <li><a href="/form/{slug}">{title}</a></li>
    {/each}
  </ul>

  <div class="row"></div>
</div>

<style>
  :root {
    font-family: "Courier New", Courier, monospace;
    font-size: 18px;
    line-height: 24px;
    color: #00ff00; /* Retro green text */
    background-color: #000000; /* Retro black background */
    --nav-background-color: #003300;
    --border-radius: 8px;
    --glitch-color: #ff00ff;
    --glitch-shadow: 0 0 5px rgba(255, 0, 255, 0.7);
    overflow-x: hidden;
  }
  .container {
    margin: 0;
    padding-top: 10vh;
    display: flex;
    flex-direction: column;
    justify-content: center;
    text-align: center;
  }

  .row {
    display: flex;
    justify-content: center;
  }

  a {
    font-weight: 500;
    color: #646cff;
    text-decoration: inherit;
  }

  a:hover {
    color: #535bf2;
  }

  h1 {
    text-align: center;
  }

  button {
    border-radius: 8px;
    border: 1px solid transparent;
    padding: 0.6em 1.2em;
    font-size: 1em;
    font-weight: 500;
    font-family: inherit;
    color: #0f0f0f;
    background-color: #ffffff;
    transition: border-color 0.25s;
    box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
  }
  nav {
    position: relative;
    display: flex;
    gap: 1em;
    padding: 1em;
    background: var(--nav-background-color);
    z-index: 2;
    margin: 0 0 1em 0;
    border-radius: var(--border-radius);
  }

  nav a {
    text-decoration: none;
  }

  @media (prefers-color-scheme: dark) {
    nav a:hover {
      color: #ff00ff;
    }
  }
</style>
