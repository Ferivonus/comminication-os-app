<script>
  /** @type {string[]} */
  let contacts = ["Alice", "Bob", "Charlie"];
  /** @type {string | null} */
  let selectedContact = null;
  /** @type {{[key: string]: string[]}} */
  let messages = {
    Alice: ["Hello Alice!", "How are you?"],
    Bob: ["Hi Bob!", "Let's catch up soon."],
    Charlie: ["Hey Charlie!", "Long time no see!"],
  };

  /** @type {(contact: string) => void} */
  function selectContact(contact) {
    selectedContact = contact;
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
    <h2>Welcome to the Communication OS Messages</h2>
    <div class="intro">
      <p>
        Dive into the depths of your communication network. Select a contact
        from the list to unveil your private conversations. Remember, messages
        exist only in this moment—stored locally for two users and never
        touching the server's void.
      </p>
    </div>

    <div class="messages-container">
      <aside class="contacts">
        <h1>Contacts</h1>
        <ul>
          {#each contacts as contact}
            <li>
              <button
                type="button"
                on:click={() => selectContact(contact)}
                on:keydown={(e) => {
                  if (e.key === "Enter" || e.key === " ") {
                    selectContact(contact);
                  }
                }}
                class:selected={contact === selectedContact}
              >
                {contact}
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
              <p>{message}</p>
            {/each}
          </div>
        {:else}
          <p>Select a contact to view messages.</p>
        {/if}
      </main>
    </div>
  </div>

  <footer class="footer">
    <p>Embrace the Chaos to Be Done and Gone | Created by Ferivonus</p>
  </footer>
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

  h1 {
    font-size: 1em;
    margin-bottom: 0.5em;
  }

  .intro {
    font-size: 0.9em; /* Slightly smaller text */
    line-height: 1.3;
    padding: 0.4em;
    border: 2px solid var(--glitch-color);
    border-radius: var(--border-radius);
    background-color: rgba(0, 0, 0, 0.8);
    box-shadow: 0 0 5px var(--glitch-shadow);
    text-align: center;
    color: #00ff00; /* Retro green text */
    max-width: 100%; /* Adjust width for better fit */
    margin: 0 auto; /* Center align the intro section */
    margin-bottom: 1em;
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

  footer {
    font-size: 0.9em;
    text-align: center;
    background: rgba(0, 0, 0, 0.8);
    padding: 1em;
    box-shadow: 0 -2px 5px rgba(0, 0, 0, 0.5);
  }

  @keyframes glitch {
    0% {
      transform: translate(0, 0);
      opacity: 1;
    }
    20% {
      transform: translate(-5px, -5px);
      opacity: 0.7;
    }
    40% {
      transform: translate(5px, 5px);
      opacity: 0.7;
    }
    60% {
      transform: translate(-5px, 5px);
      opacity: 0.7;
    }
    80% {
      transform: translate(5px, -5px);
      opacity: 0.7;
    }
    100% {
      transform: translate(0, 0);
      opacity: 1;
    }
  }

  @media (max-width: 600px) {
    .intro {
      font-size: 0.8em; /* Smaller text on smaller screens */
      padding: 0.3em;
      max-width: calc(100% - 1em); /* Adjust width for smaller screens */
    }
  }
</style>
