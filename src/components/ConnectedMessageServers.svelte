<script>
  import { invoke } from "@tauri-apps/api";

  /** @typedef {Object} Contact
   * @property {string} id
   * @property {string} nick
   */

  /** @typedef {Object} Message
   * @property {string} content
   */

  /** @type {Contact[]} */
  let contacts = [];

  /** @type {string | null} */
  let selectedContact = null;

  /** @type {Record<string, Message[]>} */
  let messages = {};

  /**
   * Fetch contacts and messages from Tauri commands.
   */
  async function fetchData() {
    try {
      // Fetch contacts
      const contactsResponse = await invoke("get_contacts_other_client");
      console.log("Contacts response:", contactsResponse); // Debug log
      contacts = contactsResponse;

      // Fetch messages for selected contact
      if (selectedContact) {
        const messagesResponse = await invoke("get_messages_other_client", {
          contact: selectedContact,
        });
        console.log("Messages response:", messagesResponse); // Debug log
        messages[selectedContact] = messagesResponse;
      }
    } catch (error) {
      console.error("Error fetching data:", error);
    }
  }

  /**
   * Select a contact and fetch related messages.
   * @param {string} contact
   */
  function selectContact(contact) {
    selectedContact = contact;
    fetchData();
  }

  import { onMount } from "svelte";
  onMount(fetchData);
</script>

<div class="messages-container">
  <aside class="contacts">
    <h1>Contacts</h1>
    <ul>
      {#each contacts as contact}
        <li>
          <button
            type="button"
            on:click={() => selectContact(contact.nick)}
            class:selected={contact.nick === selectedContact}
          >
            {contact.nick}
          </button>
        </li>
      {/each}
    </ul>
  </aside>
  <main class="message-area">
    {#if selectedContact}
      <h2>Messages with {selectedContact}</h2>
      <div class="messages">
        {#each messages[selectedContact] as message}
          <p>{message.content}</p>
        {/each}
      </div>
    {:else}
      <p>Select a contact to view messages.</p>
    {/if}
  </main>
</div>

<style>
  h1 {
    font-size: 1em;
    margin-bottom: 0.5em;
  }

  .messages-container {
    display: flex;
    width: 100%;
    height: calc(100vh - 6em); /* Adjusted for nav and footer */
  }

  .contacts {
    width: 30%;
    padding: 1em;
    border-radius: var(--border-radius);
    box-shadow: 0 0 5px var(--glitch-shadow);
    overflow-y: auto;
    background-color: rgba(112, 13, 13, 0.219);
  }

  .contacts ul {
    list-style: none;
    padding: 0;
    margin: 0;
  }

  .contacts li {
    margin: 0.5em 0;
  }

  .contacts button {
    width: 100%;
    margin: 0.5em 0;
    padding: 1em; /* Increased padding for better spacing */
    border: none;
    border-radius: var(--border-radius);
    background-color: rgba(0, 0, 0, 0.9);
    color: #00ff00;
    cursor: pointer;
    text-align: center; /* Center the text */
    font-size: 1em; /* Increase font size */
    transition: background-color 0.3s;
  }

  .contacts button.selected {
    background-color: rgba(0, 0, 0, 0.7);
    color: #ff00ff;
  }

  .message-area {
    width: 70%;
    background-color: rgba(0, 0, 0, 0.8);
    padding: 1em;
  }

  .messages p {
    margin: 0.5em 0;
    padding: 0.5em;
    background-color: rgba(112, 13, 13, 0.219);
  }

  @media (max-width: 600px) {
    /* Responsive styles */
  }
</style>
