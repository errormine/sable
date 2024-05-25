<script>
    import { onMount } from "svelte";

    export let id = "";

    let stack;
    let windows;

    function resizeWindows() {
        let bottomHeight = windows[1].clientHeight;
        windows[0].style.height = `calc(var(--main-window-height) - ${bottomHeight}px)`;
    }

    onMount(() => {
        windows = stack.children;
        let resizeObserver = new ResizeObserver(() => {
            resizeWindows();
        });

        resizeObserver.observe(windows[1]);

        addEventListener("resize", () => {
            resizeWindows();
        });

        resizeWindows();
    });
</script>

<section bind:this={stack} {id}>
    <slot />
</section>