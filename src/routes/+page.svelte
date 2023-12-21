<script lang="ts">
    import {invoke} from '@tauri-apps/api/tauri'
    import {onMount} from 'svelte';

    import Block from "./Block.svelte";
    import Output from "./Output.svelte";

    let blocks: string[] = [];
    let selectedCell: number | null = null;
    let currentInterpreter: string | null = null;

    let runResult: string  = "pypad v0.1.0 > ";

    async function fetch_sys_py_interpreters() {
        return await invoke("fetch_sys_py_interpreters");
    }

    async function runEvery() {
        if (runResult === "pypad v0.1.0 > ") {
            runResult = "";
        }

        for (const block of blocks) {
            await invoke("run_code", { code: block, interpreter: currentInterpreter }).then((result) => {
                if (result[1]) { // STD:ERR
                    runResult += `pypad v0.1.0 > ${result[0]}\n`;
                    runResult += `ERROR > ${result[1]}\n`;
                }
                else {
                    runResult += `pypad v0.1.0 > ${result[0]}\n`;
                }
            });
        }
    }

    async function runCode() {
        if (runResult === "pypad v0.1.0 > ") {
            runResult = "";
        }

        const code = blocks[selectedCell];

        await invoke("run_code", { code: code, interpreter: currentInterpreter }).then((result) => {
            if (result[1]) { // STD:ERR
                runResult += `pypad v0.1.0 > ${result[0]}\n`;
                runResult += `ERROR > ${result[1]}\n`;
            }
            else {
                runResult += `pypad v0.1.0 > ${result[0]}\n`;
            }
        });
    }

    async function run() {
        if (selectedCell === null) await runEvery();
        await runCode();
    }

    function selectCell(index: number | null) {
        selectedCell = index;

        const buttonsIds = ["cell-del", "sel-clear", "move-up", "move-down"];

        if (index === null) {
            for (const id of buttonsIds) {
                const button = document.getElementById(id);
                const buttonParent = button.parentElement;
                buttonParent.classList.remove("hover:bg-neutral-900");
                button.classList.remove("cursor-pointer");
                button.classList.add("cursor-not-allowed");
                button.classList.add("text-neutral-500");
                button.classList.remove("text-white");
            }
            const runButton = document.getElementById("run");
            runButton.classList.remove("text-white");
            runButton.classList.add("text-cyan-800");
            runButton.title = "Run all cells";
        }
        else {
            for (const id of buttonsIds) {
                const button = document.getElementById(id);
                const buttonParent = button.parentElement;
                buttonParent.classList.add("hover:bg-neutral-900");
                button.classList.add("cursor-pointer");
                button.classList.remove("cursor-not-allowed");
                button.classList.remove("text-neutral-500");
                button.classList.add("text-white");
            }
            const runButton = document.getElementById("run");
            runButton.classList.add("text-white");
            runButton.classList.remove("text-cyan-800");
            runButton.title = "Run cell";
        }
    }

    function addCell(index: number | null) {
        const initialLength = blocks.length;

        if (index === null) {
            blocks = [...blocks, ""];
        }
        else {
            blocks = [...blocks.slice(0, index), "", ...blocks.slice(index)];
        }

        if (initialLength === 0) {
            const runButton = document.getElementById("run");
            const runParent = runButton.parentElement;
            runParent.classList.add("hover:bg-neutral-900");
            runButton.classList.add("text-white");
            runButton.classList.remove("text-neutral-500");
            runButton.classList.remove("cursor-not-allowed")
            runButton.classList.add("cursor-pointer")
        }

        if (initialLength === 1) {
            const runButton = document.getElementById("run");
            runButton.classList.remove("text-white");
            runButton.classList.add("text-cyan-800");
        }
    }

    function deleteCell(index: number | null) {
        if (index === null) return;
        blocks = blocks.filter((_, i) => i !== index);
        selectCell(null)

        if (blocks.length === 0) {
            const runButton = document.getElementById("run");
            const runParent = runButton.parentElement;
            runParent.classList.remove("hover:bg-neutral-900");
            runButton.classList.remove("text-white");
            runButton.classList.add("text-neutral-500");
            runButton.classList.add("cursor-not-allowed")
            runButton.classList.remove("cursor-pointer")
            runButton.title = "Run cell";
        }

        if (blocks.length === 1) {
            const runButton = document.getElementById("run");
            runButton.classList.add("text-white");
            runButton.classList.remove("text-cyan-800");
            runButton.title = "Run cell";
        }
    }

    function handleKeyDown(event: any) {
        if ((event.key === 'Delete' || event.key === 'Backspace') && selectedCell !== null) {
            deleteCell(selectedCell);
        }

        // Run the code if Ctrl+Enter is pressed
        if (event.ctrlKey && event.key === 'Enter') {
            run();
        }
    }

    function changeInterpreter(event: any) {
        currentInterpreter = event.target.value;
    }

    function moveCellUp(index: number | null) {
        if (index === null) return;
        if (index === 0) return;
        blocks = [...blocks.slice(0, index - 1), blocks[index], blocks[index - 1], ...blocks.slice(index + 1)];
        selectCell(index - 1);
    }

    function moveCellDown(index: number | null) {
        if (index === null) return;
        if (index === blocks.length - 1) return;
        blocks = [...blocks.slice(0, index), blocks[index + 1], blocks[index], ...blocks.slice(index + 2)];
        selectCell(index + 1);
    }

    function handleChange(event) {
        selectedCell = null;
        const { index, content } = event.detail;
        blocks[index] = content;
        selectedCell = index;
    }

    function showOutput() {
        const output = document.getElementById("output-window");
        output.classList.toggle("hidden");
        output.classList.toggle("flex");
    }

    onMount(() => {
        const addButton = document.getElementById("cell-add");
        addButton.addEventListener("click", function() {
            addCell(selectedCell);
        });

        const delButton = document.getElementById("cell-del");
        delButton.addEventListener("click", function() {
            deleteCell(selectedCell);
        });

        const clearButton = document.getElementById("sel-clear");
        clearButton.addEventListener("click", function() {
            selectCell(null);
        });

        // Fetch python interpreters
        fetch_sys_py_interpreters().then((interpreters) => {
            const interSelect = document.getElementById("inter-sel");

            const interList = interpreters.split("\n");
            currentInterpreter = interList[0];
            interList.forEach((inter) => {
                const option = document.createElement("option");
                option.text = inter;
                interSelect.appendChild(option)
            });

            const indicator = document.getElementById("indicator");
            indicator.classList.remove("bg-red-700");
            indicator.classList.add("bg-green-700");
            indicator.title = "Python environment set up";
        });

        const interSelect = document.getElementById("inter-sel");
        interSelect.addEventListener("change", changeInterpreter);

        const upButton = document.getElementById("move-up");
        upButton.addEventListener("click", function() {
            moveCellUp(selectedCell);
        });

        const downButton = document.getElementById("move-down");
        downButton.addEventListener("click", function() {
            moveCellDown(selectedCell);
        });

        const flushButton = document.getElementById("flush-output");
        flushButton.addEventListener("click", function() {
            runResult = "pypad v0.1.0 > ";
        });

        window.addEventListener('keydown', handleKeyDown);
    });
</script>

<div class="flex flex-col p-3.5 pb-9 max-h-[90%] self-start" id="blockspage">
    {#each blocks as _, index}
        <Block selected={selectedCell === index} on:select={() => selectCell(index)} on:change={handleChange} content={blocks[index]} index={index}/>
    {/each}
</div>

<div class="flex bg-black w-screen justify-between h-10 bottom-0 items-center fixed p-3">
    <div class="flex items-center">
        <div class="h-full w-4 bg-red-700 mr-2 p-2 cursor-pointer rounded" id="indicator" title="No Python environment set up"></div>
        <p class="text-white font-bold font-mono">pypad</p>
    </div>

    <!-- Add dropdown with python version selection: empty first, fill up on load -->
    <select class="flex ml-3.5 w-[55%] rounded bg-neutral-900 text-white text-xs py-1" id="inter-sel"></select>

    <div class="flex items-center">
    <!-- Buttons -->
        <div class="hover:bg-neutral-900 rounded-full h-7 w-7 flex items-center justify-center mr-2"
             title="Add cell" id="cell-add"><i class="fi fi-sr-add text-white flex cursor-pointer"></i></div>
        <div class="rounded-full h-7 w-7 flex items-center justify-center mr-2"
                title="Clear selection"><i class="fi fi-sr-square-dashed text-neutral-500 flex cursor-not-allowed" id="sel-clear"></i></div>
        <div class="rounded-full h-7 w-7 flex items-center justify-center mr-2"
                title="Delete cell"><i class="fi fi-sr-trash text-neutral-500 flex cursor-not-allowed" id="cell-del"></i></div>
        <!-- Move block up -->
        <div class="rounded-full h-7 w-7 flex items-center justify-center mr-2"
             title="Move block up"><i class="fi fi-sr-up text-neutral-500 flex cursor-not-allowed" id="move-up"></i></div>
        <!-- Move block down -->
        <div class="rounded-full h-7 w-7 flex items-center justify-center mr-2"
             title="Move block down"><i class="fi fi-sr-down text-neutral-500 flex cursor-not-allowed" id="move-down"></i></div>
        <!-- Open console -->
        <div class="hover:bg-neutral-900 rounded-full h-7 w-7 flex items-center justify-center mr-2"
             title="Open output"><i class="fi fi-sr-terminal text-white flex cursor-pointer" id="console" on:click={showOutput}></i></div>
        <!-- Run every block -->
        <div class="rounded-full h-7 w-7 flex items-center justify-center ml-auto mr-2"
             title="Run cell"><i class="fi fi-sr-play text-neutral-500 flex cursor-not-allowed" id="run" on:click={run}></i></div>
    </div>
</div>

<Output code={runResult}/>
