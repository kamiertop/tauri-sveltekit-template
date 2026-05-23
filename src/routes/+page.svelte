<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';

  let name = $state('SvelteKit');
  let message = $state('');

  async function greet() {
    message = await invoke<string>('greet', { name });
  }

  const setupItems = [
    {
      file: 'src-tauri/tauri.conf.json',
      fields: 'productName, identifier, app.windows[0].title, bundle.icon'
    },
    {
      file: 'package.json',
      fields: 'name, version, scripts'
    },
    {
      file: 'src-tauri/Cargo.toml',
      fields: 'package.name, description, authors, license, repository'
    },
    {
      file: 'src/routes/+page.svelte',
      fields: 'replace this starter screen with your app UI'
    }
  ];
</script>

<svelte:head>
  <title>Tauri + SvelteKit</title>
</svelte:head>

<main class="min-h-screen bg-slate-50 px-6 py-10 text-slate-950">
  <section
    class="mx-auto grid min-h-[calc(100vh-5rem)] w-full max-w-6xl items-center gap-8 lg:grid-cols-[1fr_24rem]"
  >
    <div class="space-y-6">
      <div class="space-y-3">
        <p class="text-sm font-semibold tracking-widest text-sky-700 uppercase">
          Starter checklist
        </p>
        <h1 class="max-w-3xl text-4xl font-semibold tracking-normal text-slate-950 sm:text-5xl">
          Rename this Tauri app before building.
        </h1>
        <p class="max-w-2xl text-base leading-7 text-slate-600">
          Update the template metadata first so your packaged app has the right name, bundle
          identifier, window title, and Rust package information.
        </p>
      </div>

      <ul class="grid gap-3">
        {#each setupItems as item}
          <li class="rounded-md border border-slate-200 bg-white px-4 py-3 shadow-sm">
            <p class="font-mono text-sm text-slate-950">{item.file}</p>
            <p class="mt-1 text-sm text-slate-600">{item.fields}</p>
          </li>
        {/each}
      </ul>
    </div>

    <form
      class="space-y-4 rounded-lg border border-slate-200 bg-white p-5 shadow-sm"
      onsubmit={(event) => {
        event.preventDefault();
        greet();
      }}
    >
      <div class="space-y-1">
        <h2 class="text-base font-semibold text-slate-950">Bridge check</h2>
        <p class="text-sm text-slate-500">
          Verify the SvelteKit frontend can invoke Rust commands.
        </p>
      </div>

      <label class="block space-y-2">
        <span class="text-sm font-medium text-slate-700">Name</span>
        <input
          bind:value={name}
          class="h-11 w-full rounded-md border border-slate-300 bg-white px-3 text-slate-950 transition outline-none focus:border-sky-500 focus:ring-2 focus:ring-sky-500/20"
          autocomplete="off"
        />
      </label>

      <button
        type="submit"
        class="h-11 w-full rounded-md bg-slate-950 px-4 text-sm font-semibold text-white transition hover:bg-slate-800"
      >
        Invoke Rust command
      </button>

      {#if message}
        <p
          class="rounded-md border border-emerald-200 bg-emerald-50 px-3 py-2 text-sm text-emerald-800"
        >
          {message}
        </p>
      {/if}
    </form>
  </section>
</main>
