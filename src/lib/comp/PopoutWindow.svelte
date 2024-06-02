<script>
    import { draggable } from '@neodrag/svelte';
    import IonIosClose from "virtual:icons/ion/ios-close";
    import IconButton from "./IconButton.svelte";

    export let title = 'Window';
    export let dialog;
    export let onClose = () => dialog.close();
</script>

<dialog bind:this={dialog} use:draggable={{ bounds:'body', handle: '.title-bar'}} class="song-edit-dialog">
    <header class="title-bar">
        <h3>{title}</h3>
        <IconButton on:click={onClose}><IonIosClose/></IconButton>
    </header>
    <slot />
</dialog>

<style>
    dialog {
        position: absolute;
        inset: 0;
        margin: auto;
        z-index: 999;
        background: var(--clr-gray-1);
        border: 1px solid var(--clr-gray-5);
        border-radius: 0.5rem;
        padding: 0;

        &::backdrop {
            background: rgba(0, 0, 0, 0.5);
        }
    
        & > .title-bar {
            background: white;
            gap: 1rem;
            padding: 0 0.5rem;
            border-bottom: 1px solid var(--clr-gray-5);
            display: flex;
            justify-content: space-between;
            align-items: center;
        }

        & > .title-bar:hover {
            cursor: grab;
        }
    
        & > .title-bar > * {
            font-family: "Nimbus", sans-serif;
            font-weight: normal;
            line-height: 1;
            margin: 0;
        }
    }
</style>