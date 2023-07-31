<script>
    export let userID = 0;

    import MyMessage from "./my-message.svelte";
    import OtherMessage from "./other-message.svelte";
    import { onMount, onDestroy, tick } from "svelte";

    let data = [];
    let lastMessageId = 0;
    let divRef;

    async function fetchData() {
        try {
            const response = await fetch("http://127.0.0.1:8000/messages");

            if (!response.ok) {
                throw new Error("Network response was not ok.");
            }
            const responseData = await response.json();
            data = responseData.messages || [];
            console.log("Server IP address:", window.location.hostname);

            // Update lastMessageId with the message_id of the last message received
            if (data.length > 0) {
                lastMessageId = data[data.length - 1].message_id;
                await tick();
                scrollToBottom();
            }
        } catch (error) {
            console.error("Error fetching data:", error);
            data = null; // Set 'data' to null or an error object in case of an error.
        }
    }

    async function fetchNewMessages() {
        try {
            const url = `http://127.0.0.1:8000/messages/${lastMessageId}`;
            const response = await fetch(url);
            if (!response.ok) {
                throw new Error("Network response was not ok.");
            }
            const responseData = await response.json();
            const newMessages = responseData.messages || [];
            console.log(newMessages);
            if (newMessages.length > 0) {
                lastMessageId = newMessages[newMessages.length - 1].message_id;
                data = [...data, ...newMessages];
                await tick();
                scrollToBottomIf();
            }
        } catch (error) {
            console.error("Error fetching new messages:", error);
        }
    }

    async function startFetchInterval() {
        await fetchData();
        setInterval(fetchNewMessages, 1000);
    }

    onMount(async () => {
        await startFetchInterval();
    });

    onDestroy(() => {
        clearInterval(fetchNewMessages);
    });

    function scrollToBottom() {
        if (divRef) {
            divRef.scrollTop = divRef.scrollHeight;
        }
    }

    function scrollToBottomIf() {
        if (divRef) {
            const scrollThreshold = 0.5; // The threshold in div heights
            const distanceFromBottom =
                divRef.scrollHeight - divRef.scrollTop - divRef.clientHeight;
            if (distanceFromBottom <= divRef.clientHeight * scrollThreshold) {
                divRef.scrollTop = divRef.scrollHeight;
            }
        }
    }
</script>

<div bind:this={divRef}>
    {#if data}
        {#each data as message}
            {#if message.id == userID}
                <MyMessage message={message.message} />
            {:else}
                <OtherMessage message={message.message} />
            {/if}
        {/each}
    {:else}
        <p>Loading data...</p>
    {/if}
</div>

<style>
    div {
        background-color: rgb(54, 51, 51);
        position: absolute;
        bottom: 10%;
        left: 50%;
        transform: translate(-50%, 0%);
        height: 85vh;
        width: 120vh;
        overflow: auto;
    }
    div::-webkit-scrollbar {
        width: 12px;
    }
    div::-webkit-scrollbar-track {
        background: rgba(0, 0, 0, 0.2);
    }
    div::-webkit-scrollbar-thumb {
        background-color: rgba(255, 255, 255, 0.7);
        border-radius: 10px;
    }
    div::-webkit-scrollbar-thumb:hover {
        background-color: rgba(255, 255, 255, 0.9);
    }
</style>
