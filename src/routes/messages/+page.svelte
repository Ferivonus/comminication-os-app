<script>
  import { invoke } from "@tauri-apps/api/tauri";
  import { listen } from "@tauri-apps/api/event";
  import { onMount } from "svelte";

  let name = "";
  let greetMsg = "";
  let message = "";
  // @ts-ignore
  /**
   * @type {any[]}
   */
  let sentMessages = [];
  /**
   * @type {any[]}
   */
  let nodeMessages = [];
  /**
   * @type {any[]}
   */
  let nodeErrors = [];

  async function greet() {
    greetMsg = await invoke("greet", { name });
  }

  async function fetchWailingExampleData() {
    try {
      const data = await invoke("fetch_wailing_example_data");
      console.log("Fetched data:", data);
    } catch (error) {
      console.error("Error fetching data:", error);
    }
  }

  async function startMessageNode() {
    try {
      await invoke("start_message_node");
    } catch (error) {
      console.error("Error starting message node:", error);
    }
  }

  async function sendMessage() {
    try {
      await invoke("send_message", { message });
      sentMessages = [...sentMessages, message];
      message = "";
    } catch (error) {
      console.error("Error sending message:", error);
    }
  }

  function listenToNodeEvents() {
    listen("node-event", (event) => {
      nodeMessages = [...nodeMessages, event.payload];
    });
  }

  function listenToNodeErrors() {
    listen("node-error", (event) => {
      nodeErrors = [...nodeErrors, event.payload];
    });
  }

  onMount(() => {
    listenToNodeEvents();
    listenToNodeErrors();
    startMessageNode();
  });
</script>

<main>
  <h1>Welcome to Tauri!</h1>

  <div>
    <input bind:value={name} placeholder="Enter your name" />
    <button on:click={greet}>Greet</button>
    <p>{greetMsg}</p>
  </div>

  <div>
    <button on:click={fetchWailingExampleData}
      >Fetch Wailing Example Data</button
    >
  </div>

  <div>
    <h2>Node Messages</h2>
    <ul>
      {#each nodeMessages as message}
        <li>{message}</li>
      {/each}
    </ul>
    <h2>Node Errors</h2>
    <ul>
      {#each nodeErrors as error}
        <li>{error}</li>
      {/each}
    </ul>
  </div>

  <div>
    <input bind:value={message} placeholder="Enter your message" />
    <button on:click={sendMessage}>Send Message</button>
    <h2>Sent Messages</h2>
    <ul>
      {#each sentMessages as sentMessage}
        <li>{sentMessage}</li>
      {/each}
    </ul>
  </div>
</main>
