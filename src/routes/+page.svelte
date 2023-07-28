<script>
    import Chat from "../lib/chat.svelte";
    import { onMount } from "svelte";
    async function send_message() {
        const url = `http://127.0.0.1:8000/${encodeURIComponent(userId)}`;
        fetch(url, {
            method: "POST",
            body: input,
        });
        input = "";
    }
    function handleKeyPress(event) {
        if (event.key === "Enter" && input != "") {
            send_message();
        }
    }
    let input = "";
    function handleInput(event) {
        input = event.target.value;
    }
    let userId = 0;
    function generateRandomId() {
        return Math.floor(Math.random() * 2147483647);
    }
    onMount(() => {
        userId = generateRandomId();
    });
</script>

<h1>ksangsup</h1>
<Chat userID={userId} />
<input
    type="text"
    placeholder="Type your message..."
    on:keypress={handleKeyPress}
    value={input}
    on:input={handleInput}
/>

<!-- maybe add time in each msg-->
<!-- style the site more -->

<style>
    h1 {
        color: white;
        text-align: center;
        margin: 0;
    }
    input {
        background-color: darkgreen;
        width: 120vh; /* Adjust the width to your preference */
        height: 5vh; /* Adjust the height to your preference */
        position: absolute;
        left: 50%;
        transform: translate(-50%, 0%);
        bottom: 3%;
        border-radius: 20px; /* Increase the border-radius for a rounder appearance */
        font-size: large;
        color: white; /* Change the text color to white */
        padding: 10px; /* Add some padding inside the input box */
        border: none; /* Remove the default border */
        box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1), 0 2px 4px rgba(0, 0, 0, 0.06); /* Add a subtle box shadow */
    }

    /* Style the placeholder text */
    input::placeholder {
        color: rgba(
            255,
            255,
            255,
            0.7
        ); /* Adjust the placeholder text color and transparency */
        padding: 1px;
    }

    /* Style the input on focus */
    input:focus {
        outline: none; /* Remove the default focus outline */
        box-shadow: 0 0 0 3px rgba(34, 139, 34, 0.4); /* Add a green border on focus */
    }
</style>
