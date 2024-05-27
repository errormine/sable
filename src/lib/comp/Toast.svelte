<script>
    import { createEventDispatcher } from "svelte";
    import { fade } from "svelte/transition";
    import IonIosCheckmarkCircle from "virtual:icons/ion/ios-checkmark-circle";
    import IonIosWarning from "virtual:icons/ion/ios-warning";
    import IonIosInformationCircle from "virtual:icons/ion/ios-information-circle";
    import IonIosClose from "virtual:icons/ion/ios-close";
    import IconButton from "./IconButton.svelte";
    
    const dispatch = createEventDispatcher();
    
    export let type = "error";
    export let dismissible = true;
</script>

<article class={type} role="alert" transition:fade>
    {#if type === "success"}
        <IonIosCheckmarkCircle />
    {:else if type === "error"}
        <IonIosWarning />
    {:else}
        <IonIosInformationCircle />
    {/if}

    <section class="text">
        <slot />
    </section>
    
    {#if dismissible}
        <IconButton on:click={() => dispatch("dismiss")}>
            <IonIosClose />
        </IconButton>
    {/if}
</article>

<style lang="postcss">
    article {
        font-family: system-ui, sans-serif;
        padding: 0.5rem 1rem;
        border-radius: 0.5rem;
        display: flex;
        align-items: center;
        justify-content: space-between;
        margin: auto;
        min-width: 20rem;
        background: white;
        color: var(--clr-gray-0);
    }
    
    .error {
        background: var(--clr-error);
        border: 1px solid var(--clr-error-dark);
    }
    
    .success {
        background: var(--clr-success);
    }

    .info {
        background: var(--clr-info);
    }

    .text {
        margin: 0 2rem;
    }
</style>
