<script>
    export let size = '2rem';
    export let color = 'inherit';
    export let shape = 'circle';
    export let active = false;
</script>

<button on:click class="icon-button shape-{shape}" class:active={active} style="width: {size}; height: {size}; color: {color};" {...$$restProps}>
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

        &.active {
            background: var(--clr-gray-5);
        }

        &.shape-circle {
            padding: 4px;
            border-radius: 50%;
        }
    }
</style>