<script>
  import { invoke } from "@tauri-apps/api";

  /**
   * @typedef {Object} Contact
   * @property {string} id
   * @property {string} nick
   * @property {string} age
   * @property {string} location
   * @property {string} occupation
   * @property {string} extra_info
   */

  /** @type {Contact} */
  let contact = {
    id: "",
    nick: "",
    age: "", // Initialize as an empty string to avoid type errors
    location: "",
    occupation: "",
    extra_info: "",
  };

  /**
   * @typedef {Object} Message
   * @property {string} content
   */

  /** @type {Contact[]} */
  let contacts = [];

  /** @type {string | null} */
  let selectedContact = null;

  /** @type {Record<string, Message[]>} */
  let messages = {};

  /**
   * @param {Event} event
   */
  function handleAgeInput(event) {
    const target = event.target;

    if (target instanceof HTMLInputElement) {
      const value = target.value;

      // Ensure the value is either a number or an empty string
      contact.age = value === "" ? "" : value;
    }
  }

  async function addContact() {
    // Validate required fields
    if (!contact.id || !contact.nick) {
      alert("ID and Nick are required");
      return;
    }

    // Convert age to a number before processing
    const ageNumber = parseInt(contact.age, 10);
    if (contact.age !== "" && isNaN(ageNumber)) {
      alert("Age must be a valid number");
      return;
    }

    // Explicitly convert ageNumber to a string
    const age = contact.age === "" ? null : ageNumber;

    try {
      // Call the Tauri command
      await invoke("add_contact_my_client", {
        id: contact.id,
        nick: contact.nick,
        age: age,
        location: contact.location || null,
        occupation: contact.occupation || null,
        extra_info: contact.extra_info || null,
      });

      alert("Contact added successfully");

      // Optionally, reset the form or provide feedback
      contact = {
        id: "",
        nick: "",
        age: "",
        location: "",
        occupation: "",
        extra_info: "",
      };
    } catch (error) {
      console.error("Error adding contact:", error);
      alert("An error occurred while adding the contact");
    }
  }
</script>

<div>
  <h2>Add Contact</h2>
  <form on:submit|preventDefault={addContact}>
    <label for="id">ID:</label>
    <input type="text" id="id" bind:value={contact.id} required />

    <label for="nick">Nick:</label>
    <input type="text" id="nick" bind:value={contact.nick} required />

    <label for="age">Age:</label>
    <input
      type="number"
      id="age"
      bind:value={contact.age}
      on:input={handleAgeInput}
    />

    <label for="location">Location:</label>
    <input type="text" id="location" bind:value={contact.location} />

    <label for="occupation">Occupation:</label>
    <input type="text" id="occupation" bind:value={contact.occupation} />

    <label for="extra_info">Extra Info:</label>
    <textarea id="extra_info" bind:value={contact.extra_info}></textarea>

    <button type="submit">Add Contact</button>
  </form>
</div>

<style>
  form {
    display: flex;
    flex-direction: column;
    gap: 1em;
  }
  label {
    font-weight: bold;
  }
  input,
  textarea {
    padding: 0.5em;
    border-radius: 4px;
    border: 1px solid #ccc;
  }
  button {
    padding: 1em;
    border: none;
    border-radius: 4px;
    background-color: #007bff;
    color: white;
    cursor: pointer;
  }
  button:hover {
    background-color: #0056b3;
  }
</style>
