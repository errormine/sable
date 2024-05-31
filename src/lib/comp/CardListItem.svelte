<script>
    export let highlighted = false;
    export let title = '';
    export let subtitle = '';
    export let floatingText = '';

    export let onClick = () => {};
</script>

<li class="card" class:active={highlighted}>
    <button on:click={onClick}>
        <slot />
        <section class="no-wrap">
            <p class="title no-wrap" title={title}><strong>{title}</strong></p>
            <p class="subtitle no-wrap" title={subtitle}>{subtitle}</p>
        </section>
        <p class="floating no-wrap">{floatingText}</p>
    </button>
</li>

<style>
    .card {
        color: var(--clr-gray-7);
        border-radius: 0.25rem;
        overflow: hidden;
    
        &.active {
            color: var(--clr-gray-9);
            background: var(--clr-gray-3);
        }
    }

    button {
        display: grid;
        grid-template-columns: 3rem 1fr auto;
        align-items: center;
        gap: 0.5rem;
        padding: 0.25rem;
        width: 100%;
        color: inherit;
    
        & .floating {
            text-align: right;
        }
    }
</style>