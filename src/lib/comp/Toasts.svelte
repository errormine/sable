<script lang="ts">
    import Toast from "./Toast.svelte";
    
    import { dismissToast, toasts } from "../stores/notifications";
</script>

{#if $toasts}
    <section>
        {#each $toasts as toast (toast.id)}
            <Toast type={toast.type} dismissible={toast.dismissible} on:dismiss={() => dismissToast(toast.id)}>
                {toast.message}
            </Toast>
        {/each}
    </section>
{/if}

<style lang="postcss">
    section {
        position: fixed;
        top: 0;
        left: 0;
        right: 0;
        width: 100%;
        margin-top: 1rem;
        display: flex;
        justify-content: center;
        flex-direction: column;
        gap: 0.5rem;
        z-index: 1000;
    }
</style>
