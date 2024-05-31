<script>
    import { onMount, setContext } from "svelte";
    import { writable } from "svelte/store";

    const tabs = writable([]);
    setContext("tabs", tabs);

    let activeTab = 0;

    function updateActiveTab(index) {
        activeTab = index;
        $tabs[index].window.classList.remove("hidden");
        $tabs.forEach((tab, i) => {
            if (i !== index) {
                tab.window.classList.add("hidden");
            }
        });
    }

    onMount(() => {
        updateActiveTab(activeTab);
    })
</script>

<slot />