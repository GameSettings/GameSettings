<body>
    <div class="flex flex-col">
        <label class="i600 text-orange-200" for="mouse_sens">Mouse Sensitivity</label>
        <div class="flex items-center">
            <input bind:value={mouse_sens} class="ease-in-out duration-150 hover:shadow-xl hover:shadow-orange-900 bg-white rounded-3xl h-4 shadow accent-orange-400 w-5/6 appearance-none" name="mouse_sens" type="range" min="0" max="100" step="0.1">
            <input bind:value={mouse_sens} class="outline-none caret-orange-400 ml-3 i600 w-9 h-5 bg-transparent appearance-none ring-2 ring-orange-400 text-white text-center rounded-sm" type="text">
        </div>
    </div>
    <div class="flex flex-col">
        <label class="i600 text-orange-200" for="mouse_sens">Aiming Sensitivity</label>
        <div class="flex items-center">
            <input bind:value={aiming_sens} class="ease-in-out duration-150 hover:shadow-xl hover:shadow-orange-900 bg-white rounded-3xl h-4 shadow accent-orange-400 w-5/6 appearance-none" name="mouse_sens" type="range" min="0" max="100" step="0.1">
            <input bind:value={aiming_sens} class="outline-none caret-orange-400 ml-3 i600 w-9 h-5 bg-transparent appearance-none ring-2 ring-orange-400 text-white text-center rounded-sm" type="text">
        </div>
    </div>
</body>

<script lang="ts">
    let mouse_sens: number = 50;
    let aiming_sens: number = 50;
</script>