<script>
    import { getContext, onMount } from "svelte";
    import { groups, registerTab } from "../stores/windowManager";
    import ContextMenu from "svelte-contextmenu";
    import IonIosMenu from "virtual:icons/ion/ios-menu";

    const group = getContext("group");

    export let title = "Window";
    /**
     * @type {ContextMenu | null}
     */
    export let contextMenu = null;
    
    let window;
    
    onMount(() => {
        if (group) {
            registerTab(group, title, window);
        }
    });
</script>

<section bind:this={window} class="window">
    <header>
        <section class="title">
            <p>{title}</p>
        </section>
        {#if contextMenu}
            <button on:click={(e) => contextMenu.show(e)}>
                <IonIosMenu/>
            </button>
        {/if}
    </header>
    <slot />
</section>

<style>
    .window {
        max-height: inherit;
    
        & > header {
            background: var(--clr-gray-1);
            display: flex;
            justify-content: space-between;
            text-align: center;
            text-transform: uppercase;
            color: var(--clr-gray-6);
            height: var(--window-title-height);
        }
    
        & > header > .title {
            height: 100%;
            display: flex;
            align-items: flex-end;
            justify-content: center;
            flex: 1;
        }

        & > header button {
            padding-top: calc(var(--window-title-height) / 8);
        }
    }

    /* Svelte doesn't seem to play nice if this is nested */
    .window > :global(section) {
        max-height: calc(100% - var(--window-title-height));
    }
</style>