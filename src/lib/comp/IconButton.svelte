<button on:click>
    <slot />
</button>

<style>
    button {
        display: block;
        width: min-content;
        height: min-content;
        border-radius: 0.25rem;
        transition: opacity 200ms;
    }

    button:hover {
        opacity: 0.8;
    }

    button:active {
        opacity: 1;
        background: var(--clr-gray-5);
    }
</style>