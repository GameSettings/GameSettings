<body>
    <div class="grid gap-4 grid-cols-2 grid-flow-row auto-rows-min bg-[#111111] h-screen p-4">
        <div class="border-4 rounded-xl border-blue-500 p-4">
            <Main />
        </div>
        <div class="ring-4 rounded-xl ring-violet-700 p-4">
            <Social />
        </div>
        <div class="border-4 rounded-xl border-orange-500 p-4">
            <Sensitivity />
        </div>
        <div class="ring-4 rounded-xl ring-violet-700 p-4">
            4
        </div>
    </div>
</body>

<script lang="ts">
    import Sensitivity from "./components/Sensitivity.svelte";
    import Main from "./components/Main.svelte";
    import Social from "./components/Social.svelte";
</script>