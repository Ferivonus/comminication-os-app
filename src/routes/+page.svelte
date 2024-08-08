<script>
  import { onMount } from "svelte";

  // Sample quotes from Serial Experiments Lain
  const quotes = [
    "The world is made up of walls.",
    "I am just a regular girl, nothing more.",
    "You cannot truly be alone.",
    "I need to understand everything.",
    "I am not me. I am not anyone. I am the world.",
  ];

  let currentQuote = "";

  function getRandomQuote() {
    return quotes[Math.floor(Math.random() * quotes.length)];
  }

  onMount(() => {
    currentQuote = getRandomQuote();
    setInterval(() => {
      currentQuote = getRandomQuote();
    }, 15000); // Change quote every 15 seconds
  });
</script>

<div class="page-wrapper">
  <div class="container">
    <h1>Welcome to the Communication OS</h1>

    <div class="intro">
      <p>
        Welcome to your new communication platform, inspired by *Serial
        Experiments Lain*. Dive into the depths of digital interaction with our
        retro-styled OS.
      </p>
    </div>

    <div class="quote-section">
      <p class="quote">{currentQuote}</p>
    </div>

    <div class="navigation-links">
      <h2>Explore More</h2>
      <ul>
        <li>
          <a href="/">
            Home
            <p class="link-description">
              Return to the main page of the Communication OS.
            </p>
          </a>
        </li>
        <li>
          <a href="/about">
            About
            <p class="link-description">
              Learn more about the concept and development of this application.
            </p>
          </a>
        </li>
        <li>
          <a href="/form">
            Form
            <p class="link-description">
              Access the form page to input and manage various forms.
            </p>
          </a>
        </li>
        <li>
          <a href="/messages">
            Messages
            <p class="link-description">
              View and manage messages in the communication system.
            </p>
          </a>
        </li>
        <li>
          <a href="/wailing-wall">
            Wailing Wall
            <p class="link-description">
              Contribute and read messages on the virtual wailing wall.
            </p>
            dd
          </a>
        </li>
      </ul>
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

  .container {
    flex: 1;
    padding: 5vh 2vw;
    display: flex;
    flex-direction: column;
    align-items: center;
    text-align: center;
  }

  h1 {
    font-size: 2em;
    margin-bottom: 0.5em;
  }

  .intro {
    margin-bottom: 1em;
    font-size: 1.2em;
    line-height: 1.5;
    padding: 1em;
    border: 2px solid var(--glitch-color);
    border-radius: var(--border-radius);
    background-color: rgba(0, 0, 0, 0.8);
    box-shadow: 0 0 5px var(--glitch-shadow);
  }

  .quote-section {
    margin: 1em 0;
  }

  .quote {
    font-size: 1.5em;
    margin: 0;
    padding: 1em;
    border: 2px solid var(--glitch-color);
    border-radius: var(--border-radius);
    box-shadow: 0 0 5px var(--glitch-shadow);
    background-color: rgba(0, 0, 0, 0.8);
    animation: glitch 1.5s infinite;
  }

  .navigation-links {
    margin: 1em 0;
    text-align: center;
  }

  .navigation-links h2 {
    font-size: 1.5em;
    margin-bottom: 0.5em;
    color: #ff00ff;
  }

  .navigation-links ul {
    list-style: none;
    padding: 0;
    margin: 0;
  }

  .navigation-links li {
    margin: 0.5em 0;
    position: relative;
    display: inline-block;
    width: 200px;
  }

  .navigation-links a {
    color: #00ff00;
    text-decoration: none;
    font-weight: bold;
    text-shadow: 0 0 2px var(--glitch-color);
    display: block;
    padding: 0.5em;
    border-radius: var(--border-radius);
    background-color: rgba(0, 0, 0, 0.8);
    box-shadow: 0 0 5px var(--glitch-shadow);
    transition:
      background-color 0.3s,
      color 0.3s;
    z-index: 1;
    border: var(--box-border);
  }

  .navigation-links a:hover {
    color: #ff00ff;
    background-color: rgba(0, 0, 0, 0.9);
    text-shadow: 0 0 10px var(--glitch-color);
  }

  .link-description {
    font-size: 0.9em;
    color: #00ff00;
    margin-top: 0.5em;
    display: none;
    position: absolute;
    top: 100%;
    left: 0;
    padding: 0.5em;
    background-color: rgba(0, 0, 0, 0.9);
    border-radius: var(--border-radius);
    box-shadow: 0 0 5px var(--glitch-shadow);
    width: 200px;
    text-align: center;
    z-index: 2;
  }

  .navigation-links li:hover .link-description {
    display: block;
  }

  .footer {
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

  @media (prefers-color-scheme: dark) {
  }
</style>
