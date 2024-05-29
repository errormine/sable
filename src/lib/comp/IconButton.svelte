<script>
    export let size = '2rem';
    export let color = 'inherit';
</script>

<button on:click class="icon-button" style="width: {size}; height: {size}; color: {color};">
    <slot />
</button>

<style>
    :global(.icon-button > svg) {
        width: 100%;
        height: 100%;
        color: inherit;
    }

    button {
        display: block;
        border-radius: 0.25rem;
        transition: all 200ms;
        padding: 2px;
    
        &:hover {
            background: var(--clr-gray-4);
        }
    }
</style>