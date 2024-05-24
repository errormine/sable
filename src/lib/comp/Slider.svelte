<script>
    import { onMount } from "svelte";

    export let input = null;
    export let color = 'var(--clr-primary)';

    export let name = '';
    export let id = '';

    export let min = 0;
    export let max = 100;
    export let step = 1;
    export let value = 0;

    onMount(() => {
        if (!input) return;

        input.style.setProperty('--accent-color', color);
        input.style.setProperty('--value', value);
        input.style.setProperty('--min', min);
        input.style.setProperty('--max', max);

        input.addEventListener('input', (event) => {
            input.style.setProperty('--value', input.value);
            input.style.setProperty('--min', input.min);
            input.style.setProperty('--max', input.max);
        });
    });
</script>

<input bind:this={input} class="styled-slider slider-progress" type="range" {name} {id} {min} {max} {step} {value} />

<style>
    input {
        width: 100%;
    }
</style>
