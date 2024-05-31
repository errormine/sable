<script>
    import { setContext } from "svelte";
    import { registerGroup } from "../stores/windowManager";

    export let name;
    
    setContext('group', name);

    registerGroup(name);
</script>

<slot />