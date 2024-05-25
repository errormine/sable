<script>
    export let title = "Window";
</script>

<section class="window">
    <header>
        <p>{title}</p>
    </header>
    <slot />
</section>

<style>
    .window {
        height: inherit;
    }

    .window header {
        background: var(--clr-gray-1);
        text-align: center;
        text-transform: uppercase;
        color: var(--clr-gray-6);
        height: var(--menu-bar-height);
    }

    .window header p {
        display: inline;
        vertical-align: middle;
    }

    .window > :global(section) {
        max-height: calc(100% - var(--menu-bar-height));
    }
</style>