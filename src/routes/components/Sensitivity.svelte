<body>
    <div class="flex flex-col">
        <label class="i600 text-white" for="mouse_sens">Mouse Sensitivity</label>
        <div class="flex items-center">
            <input bind:value={mouse_sens} class="ease-in-out duration-150 hover:shadow-xl hover:shadow-violet-900 bg-white rounded-3xl h-4 shadow accent-violet-400 w-5/6 appearance-none" name="mouse_sens" type="range" min="0" max="100" step="0.1">
            <input bind:value={mouse_sens} class="ml-3 i600 w-9 h-5 bg-transparent appearance-none ring-2 ring-violet-400 text-white text-center rounded-sm" type="text">
        </div>
    </div>
    <div class="flex flex-col">
        <label class="i600 text-white" for="mouse_sens">Aiming Sensitivity</label>
        <div class="flex items-center">
            <input bind:value={aiming_sens} class="ease-in-out duration-150 hover:shadow-xl hover:shadow-violet-900 bg-white rounded-3xl h-4 shadow accent-violet-400 w-5/6 appearance-none" name="mouse_sens" type="range" min="0" max="100" step="0.1">
            <input bind:value={aiming_sens} class="ml-3 i600 w-9 h-5 bg-transparent appearance-none ring-2 ring-violet-400 text-white text-center rounded-sm" type="text">
        </div>
    </div>
</body>

<style>
    .i600 {
        font-family: 'Inter Tight', sans-serif;
        font-weight: 600;
    }
    .i400 {
        font-family: 'Inter Tight', sans-serif;
        font-weight: 400;
    }
</style>

<script lang="ts">
    let mouse_sens: number = 50;
    let aiming_sens: number = 50;
</script>