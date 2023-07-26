<script>
    export let userID = 0;

    import MyMessage from "./my-message.svelte";
    import OtherMessage from "./other-message.svelte";
    import { onMount, afterUpdate } from "svelte";
    import { onDestroy } from "svelte";

    let data;
    let divRef;

    async function fetchData() {
        try {
            const response = await fetch("http://127.0.0.1:8000/messages");

            if (!response.ok) {
                throw new Error("Network response was not ok.");
            }

            data = await response.json();
        } catch (error) {
            console.error("Error fetching data:", error);
            data = null; // Set 'data' to null or an error object in case of an error.
        }
    }

    async function startFetchInterval() {
        await fetchData();
        setInterval(fetchData, 1000);
    }

    onMount(async () => {
        startFetchInterval();
    });

    afterUpdate(() => {
        scrollToBottom();
    });

    onDestroy(() => {
        // Cleanup, clear the interval when the component is destroyed
        clearInterval(startFetchInterval);
    });

    function scrollToBottom() {
        if (divRef) {
            divRef.scrollTop = divRef.scrollHeight;
        }
    }
</script>

<div bind:this={divRef}>
    {#if data}
        {#each data.messages as message}
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
