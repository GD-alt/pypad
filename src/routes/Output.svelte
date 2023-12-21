<script>
    import { onMount } from 'svelte';

    export let code = "pypad v0.1.0 >";

    function minimize() {
        let output = document.getElementById("output-window");
        output.classList.toggle("flex");
        output.classList.toggle("hidden");
    }
</script>

<style>
    #output-window {
        resize: both;
        overflow: auto;
    }
</style>

<div class="w-screen fixed bottom-10 bg-black p-1 hidden" id="output-window">
<div class="flex-col w-full p-5">
    <div class="flex justify-between">
        <p class="font-bold text-neutral-50 flex"><i class="fi fi-sr-poll-h flex items-center pr-2"></i> Output</p>
        <div class="flex">
            <div class="hover:bg-neutral-900 rounded-full h-7 w-7 flex items-center justify-center ml-auto mr-2"
                 title="Clear output"><i class="fi fi-sr-broom text-neutral-50 flex cursor-pointer" id="flush-output"></i></div>
            <div class="hover:bg-neutral-900 rounded-full h-7 w-7 flex items-center justify-center ml-auto mr-2"
                 title="Minimize"><i class="fi fi-sr-window-minimize text-neutral-50 flex cursor-pointer" id="min-output" on:click={minimize}></i></div>
        </div>
    </div>

    <p id="output" class="m-2 p-1.5 rounded bg-neutral-800 text-white font-mono text-xs w-full" style="white-space: pre-wrap">{code}</p>
</div>
</div>