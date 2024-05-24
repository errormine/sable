<svelte:options accessors />
<script>
    import { onMount } from "svelte";

    export let input = null;
    export let color = 'var(--clr-primary)';
    export let width = '100%';

    export let name = '';
    export let id = '';

    export let min = 0;
    export let max = 100;
    export let step = 1;
    export let value = 0;

    export let disabled = false;
    
    let style = `--width: ${width}; --accent-color: ${color}; --min: ${min}; --max: ${max}; --value: ${value};`;

    export function setValue(newValue) {
        input.value = newValue;
        input.style.setProperty('--value', input.value);
        input.style.setProperty('--min', input.min);
        input.style.setProperty('--max', input.max);
    }

    onMount(() => {
        input.addEventListener('input', (event) => {
            input.style.setProperty('--value', event.target.value);
        });
    });
</script>

<input bind:this={input} class="styled-slider slider-progress" type="range" {style} {name} {id} {min} {value} {max} {step} {disabled} />

<style>
    input {
        width: var(--width);
    }
</style>