<script>
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api";

  /**
   * @typedef {Object} FormPage
   * @property {string} slug
   * @property {string} title
   */

  /** @type {FormPage[]} */
  let formPages = [];

  /** @type {FormPage | null} */
  let selectedForm = null;

  /** @type {string | null} */
  let error = null;

  // Load function to fetch form pages from Tauri command
  async function load() {
    try {
      formPages = await invoke("fetch_form_pages");
    } catch (e) {
      error = "Failed to fetch form pages.";
      console.error(e);
    }
  }

  onMount(() => {
    load();
  });

  /**
   * Selects a form based on its title.
   * @param {string} title - The title of the form to select.
   */
  function selectForm(title) {
    // Find the selected form based on the title
    selectedForm = formPages.find((form) => form.title === title) || null;
  }

  function handleNotification() {
    // Function to trigger a notification
    invoke("notify_frontend", { message: "Hello from frontend!" })
      .then(() => console.log("Notification sent successfully!"))
      .catch((e) => {
        console.error("Failed to send notification:", e);
        alert("Failed to send notification. Check console for details.");
      });
  }
</script>

<nav>
  <a href="/">Home</a>
  <a href="/about">About</a>
  <a href="/form">Form</a>
  <a href="/messages">Messages</a>
  <a href="/wailing-wall">Wailing Wall</a>
</nav>

<div class="container">
  <h1>Form Pages</h1>

  {#if error}
    <p class="error">{error}</p>
  {:else}
    <ul>
      {#each formPages as { title }}
        <li>
          <button on:click={() => selectForm(title)}>{title}</button>
        </li>
      {/each}
    </ul>

    {#if selectedForm}
      <div>
        <h2>{selectedForm.title}</h2>
        <!-- Display form details here -->
        <!-- Example: -->
        <p>Details for the form: {selectedForm.title}</p>
      </div>
    {/if}
  {/if}

  <button on:click={handleNotification}>Send Notification</button>
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

  .error {
    color: #ff0000; /* Red text for errors */
  }

  @media (prefers-color-scheme: dark) {
    nav a:hover {
      color: #ff00ff;
    }
  }
</style>
