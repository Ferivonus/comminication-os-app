<script>
  import AddContactMyClient from "../../components/message_components/AddContactMyClient.svelte";
  import AddContactOtherClient from "../../components/message_components/AddContactOtherClient.svelte";
  import MyServerMessages from "../../components/MyServerMessages.svelte";
  import ConnectedMessageServers from "../../components/ConnectedMessageServers.svelte";

  /** @type {string} */
  let selectedContact = "";

  /** @type {string | null} */
  let selectedView = null;

  function showMyServerMessages() {
    selectedView = "myServerMessages";
  }

  function showConnectedMessageServers() {
    selectedView = "connectedMessageServers";
  }

  function showAddContactMyClient() {
    selectedView = "addContactMyClient";
  }

  function showAddContactOtherClient() {
    selectedView = "addContactOtherClient";
  }

  // Handle contact selection from ConnectedMessageServers
  /**
   * @param {CustomEvent} event
   */
  function handleContactSelection(event) {
    selectedContact = event.detail.contact;
  }
</script>

<div class="page-wrapper">
  <nav>
    <a href="/">home</a>
    <a href="/about">about</a>
    <a href="/form">form</a>
    <a href="/messages">messages</a>
    <a href="/wailing-wall">wailing wall</a>
  </nav>

  <div class="container">
    <h2>Welcome to the Communication OS</h2>
    <div class="button-container">
      <button on:click={showMyServerMessages}>My Server Messages</button>
      <button on:click={showConnectedMessageServers}
        >Connected Message Servers</button
      >
      <button on:click={showAddContactMyClient}>Add Contact My Client</button>
      <button on:click={showAddContactOtherClient}
        >Add Contact Other Client</button
      >
    </div>

    {#if selectedView === "myServerMessages"}
      <MyServerMessages
        {selectedContact}
        on:contactSelected={handleContactSelection}
      />
    {:else if selectedView === "connectedMessageServers"}
      <ConnectedMessageServers
        {selectedContact}
        on:contactSelected={handleContactSelection}
      />
    {:else if selectedView === "addContactMyClient"}
      <AddContactMyClient />
    {:else if selectedView === "addContactOtherClient"}
      <AddContactOtherClient />
    {/if}

    {#if selectedContact}
      <ConnectedMessageServers
        {selectedContact}
        on:contactSelected={handleContactSelection}
      />
    {/if}
  </div>
</div>

<style>
  :root {
    font-family: "Courier New", Courier, monospace;
    font-size: 18px;
    line-height: 24px;
    color: #00ff00;
    background-color: #000000;
    --nav-background-color: #003300;
    --border-radius: 8px;
    --glitch-color: #ff00ff;
    --glitch-shadow: 0 0 5px rgba(255, 0, 255, 0.7);
    --box-border: 2px solid #00ff00;
  }

  .button-container {
    display: flex;
    gap: 1em;
    margin: 2em 0;
  }

  .page-wrapper {
    display: flex;
    flex-direction: column;
    min-height: 100vh;
  }

  nav {
    display: flex;
    justify-content: center;
    gap: 1em;
    padding: 1em;
    background: var(--nav-background-color);
    z-index: 2;
    border-radius: var(--border-radius);
  }

  nav a {
    text-decoration: none;
    color: #00ff00;
    font-weight: bold;
    text-shadow: 0 0 2px var(--glitch-color);
    transition:
      color 0.3s,
      text-shadow 0.3s;
  }

  nav a:hover {
    color: #ff00ff;
    text-shadow: 0 0 10px var(--glitch-color);
  }

  .container {
    flex: 1;
    padding: 5vh 2vw;
    display: flex;
    flex-direction: column;
    align-items: center;
    text-align: center;
  }

  button {
    padding: 1em;
    border: none;
    border-radius: var(--border-radius);
    background-color: rgba(0, 0, 0, 0.9);
    color: #00ff00;
    cursor: pointer;
    font-size: 1em;
    transition: background-color 0.3s;
  }

  button:hover {
    background-color: rgba(0, 0, 0, 0.7);
    color: #ff00ff;
  }
</style>
