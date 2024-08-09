<script>
  import { createEventDispatcher } from "svelte";
  import { invoke } from "@tauri-apps/api";

  /** @type {string} */
  export let selectedContact;

  /** @typedef {Object} Contact
   * @property {string} id
   * @property {string} nick
   */

  /** @typedef {Object} Message
   * @property {string} content
   */

  /** @typedef {Object} newMessage
   * @property {string} sender
   * @property {string | null} receiver
   * @property {string} content
   * @property {string | null} close_one_point
   * @property {string | null} connected_person
   */

  /** @type {Contact[]} */
  let contacts = [];

  /** @type {Message[]} */
  let messages = [];

  /** @type {string} */
  let newMessageContent = "";

  /** @type {string} */
  let sender = "";

  /** @type {string} */
  let closeOnePoint = "";

  /** @type {newMessage} */
  let newMessage = {
    sender: "",
    receiver: "",
    content: "",
    close_one_point: "",
    connected_person: "",
  };

  const dispatch = createEventDispatcher();

  /**
   * Fetch contacts from Tauri command.
   */

  async function fetchContacts() {
    try {
      const contactsResponse = await invoke("get_contacts_my_client");
      contacts = contactsResponse;
    } catch (error) {
      console.error("Error fetching contacts:", error);
    }
  }

  /**
   * Fetch messages for the selected contact.
   */
  async function fetchMessages() {
    if (!selectedContact) return;
    console.log(selectedContact);

    try {
      let messagesResponse = await invoke("get_messages_my_client", {
        connected_person: selectedContact,
      });
      messages = messagesResponse;
    } catch (error) {
      console.error("Error fetching messages:", error);
    }
  }

  /**
   * Send a new message to the selected contact.
   * @param {Event} event
   */
  async function sendMessage(event) {
    event.preventDefault();

    if (!newMessageContent || !sender || !selectedContact) {
      alert("All fields must be filled out");
      return;
    }

    newMessage = {
      sender: sender,
      receiver: selectedContact,
      content: newMessageContent,
      close_one_point: closeOnePoint || null,
      connected_person: selectedContact || null,
    };

    try {
      console.log("Sending message:", newMessage);

      await invoke("send_message_my_client", { new_message: newMessage });

      newMessageContent = "";
      sender = "";
      closeOnePoint = "";
      selectedContact = "";

      await fetchMessages();
    } catch (error) {
      console.error("Error sending message:", error);
      alert("Failed to send message. Please check the console for details.");
    }
  }
  /**
   * Select a contact to view messages.
   * @param {string} contact
   */
  function selectContact(contact) {
    selectedContact = contact;
    fetchMessages(); // Fetch messages for the selected contact
    dispatch("contactSelected", { contact });
  }

  // Fetch contacts on component mount
  fetchContacts();
</script>

<div class="app-container">
  <!-- Contacts List -->
  <div class="contacts-container">
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
  </div>

  <!-- Message Area -->
  <div class="message-area" class:visible={selectedContact}>
    {#if selectedContact}
      <h2>Messages with {selectedContact}</h2>
      <div class="messages">
        {#each messages as message}
          <p>{message.content}</p>
        {/each}
      </div>
      <form on:submit={sendMessage}>
        <textarea
          bind:value={newMessageContent}
          placeholder="Enter your message"
          required
        ></textarea>
        <input
          type="text"
          bind:value={sender}
          placeholder="Sender ID"
          required
        />
        <input
          type="text"
          bind:value={closeOnePoint}
          placeholder="Close One Point"
        />
        <input
          type="text"
          bind:value={selectedContact}
          placeholder="Connected Person ID"
          required
        />
        <button type="submit">Send Message</button>
      </form>
    {:else}
      <p>Select a contact to view messages.</p>
    {/if}
  </div>
</div>

<style>
  .app-container {
    display: flex;
    height: 100vh;
  }

  .contacts-container {
    width: 30%;
    padding: 1em;
    border-radius: var(--border-radius);
    box-shadow: 0 0 5px var(--glitch-shadow);
    background-color: rgba(112, 13, 13, 0.219);
    overflow-y: auto;
  }

  .contacts-container ul {
    list-style: none;
    padding: 0;
    margin: 0;
  }

  .contacts-container li {
    margin: 0.5em 0;
  }

  .contacts-container button {
    width: 100%;
    margin: 0.5em 0;
    padding: 1em;
    border: none;
    border-radius: var(--border-radius);
    background-color: rgba(0, 0, 0, 0.9);
    color: #00ff00;
    cursor: pointer;
    text-align: center;
    font-size: 1em;
    transition: background-color 0.3s;
  }

  .contacts-container button.selected {
    background-color: rgba(0, 0, 0, 0.7);
    color: #ff00ff;
  }

  .message-area {
    width: 70%;
    padding: 1em;
    background-color: rgba(0, 0, 0, 0.8);
    border-radius: var(--border-radius);
    box-shadow: 0 0 5px var(--glitch-shadow);
    height: calc(100vh - 6em); /* Adjusted for nav and footer */
    overflow-y: auto;
    display: none;
  }

  .message-area.visible {
    display: block;
  }

  .messages p {
    margin: 0.5em 0;
    padding: 0.5em;
    background-color: rgba(112, 13, 13, 0.219);
  }

  textarea,
  input {
    width: 100%;
    padding: 0.5em;
    border-radius: var(--border-radius);
    border: 1px solid #ccc;
    margin-bottom: 1em;
  }

  button {
    padding: 0.5em;
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
  }
</style>
