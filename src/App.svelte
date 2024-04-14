<script lang="ts">
  import { event } from "@tauri-apps/api";
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";

  let count = 0;
  let uiCount = 0;
  onMount(() => {
    setInterval(async () => {
      console.time("get_blob");
      await invoke("get_blob");
      console.timeEnd("get_blob");
      uiCount++;
    }, 1000);

    event.listen("from_backend", (args: any) => {
      console.log(args.payload.length, "bytes received");
      count++;
    });
  });
</script>

<main class="container">
  <h1>sending / receiving data via IPC</h1>

  <p>
    backend sent {count} blobs (see CLI for timing)
  </p>
  <p>
    UI sent {uiCount} requests for blobs (see console for timing)
  </p>
</main>
