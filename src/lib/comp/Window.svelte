<script>
    export let title = "Window";
</script>

<section class="window">
    <header>
        <p>{title}</p>
    </header>
    <slot/>
</section>

<style>
    .window {
        max-height: 100%;
        position: relative;
    }

    .window header {
        background: var(--clr-gray-1);
        padding: 0.25rem 0.5rem;
        text-align: center;
        text-transform: uppercase;
        color: var(--clr-gray-6);
    }
</style>