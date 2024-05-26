<script>
    export let title = "Window";
</script>

<section class="window">
    <header>
        <p class="title">{title}</p>
    </header>
    <slot />
</section>

<style>
    .window {
        max-height: inherit;
    
        & > header {
            background: var(--clr-gray-1);
            text-align: center;
            text-transform: uppercase;
            color: var(--clr-gray-6);
            height: var(--menu-bar-height);
        }
    
        & > .title {
            display: inline;
            vertical-align: middle;
        }
    }

    /* Svelte doesn't seem to play nice if this is nested */
    .window > :global(section) {
        max-height: calc(100% - var(--menu-bar-height));
    }
</style>