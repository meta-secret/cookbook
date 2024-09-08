<script lang="ts">
    // https://www.mbaraa.com/blog/using-wasm-with-sveltekit

    import init, { greet, ReactiveState } from "meta-secret-wasm"; // import the WASM module
    // we need onMount to run init
    import { onMount } from "svelte";

    let reactiveState: ReactiveState;
    let counter = 1;

    onMount(async () => {
        await init(); // init initializes memory addresses needed by WASM and that will be used by JS/TS

        reactiveState = ReactiveState.new();

        subscribe()
            .then(() => console.log("Finished subscribing"));

        console.log("Subscribed to reactive state");
    })

    async function subscribe() {
        while(true) {
            await reactiveState.receive().then((response) => {
                counter++;
                console.log(response);
            });
        }
    }

    async function send() {
        await reactiveState.send(`Hello hello: ${counter}`);
    }
</script>

<h2>MetaSecret reactive state</h2>

{#if reactiveState}
    <div>
        <button on:click={() => {send()}}>Send Signal to the Universe</button>
    </div>
{:else}
    <p>Loading...</p>
{/if}
