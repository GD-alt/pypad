<script lang="ts">
    import {createEventDispatcher, onMount} from "svelte";
    import {invoke} from "@tauri-apps/api/tauri";
    const dispatch = createEventDispatcher();

    function select() {
        dispatch('select');
    }

    function handleKeyDown(event) {
        if (event.key === 'Delete' || event.key === 'Backspace') {
            event.stopPropagation();
        }
    }

    export function autoExpand(node) {
        function resize() {
            node.style.height = 'auto';
            node.style.height = node.scrollHeight + 'px';
        }

        node.addEventListener('input', resize);
        resize();

        return {
            destroy() {
                node.removeEventListener('input', resize);
            }
        };
    }

    export function synchronize(node) {
        node.addEventListener('input', () => {
            content = node.value;
        });

        onMount(() => {
            node.value = content;
        });

        return {
            destroy() {
                node.removeEventListener('input', () => {
                    content = node.value;
                });
            }
        };
    }

    export let selected = false;
    export let content = '';

    export let index = 0;

    function handleChange() {
        dispatch('change', { index, content });
    }
</script>

<div class="flex w-full p-2 flex-wrap justify-between items-center">
    <div class={`flex items-center justify-center w-3 h-[100px] mr-3 rounded-full ${selected ? 'bg-cyan-800' : 'bg-neutral-800'} hover:bg-neutral-700 active:bg-cyan-800 cursor-move`} on:click={select}></div>
    <div class={`flex ${selected ? 'bg-neutral-800' : 'bg-neutral-900'} w-[95%] p-3 h-full min-h-[80px] rounded`}>
        <textarea class="bg-transparent text-white w-full h-full outline-none text-sm font-mono" id="code-input" use:autoExpand use:synchronize bind:value={content} on:input={handleChange} placeholder="Here your code starts…" spellcheck="false" on:keydown={handleKeyDown}></textarea>
    </div>
</div>