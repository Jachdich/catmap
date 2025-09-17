<script lang="ts">
    interface Props {
        editing: boolean,
        val: string | undefined,
    }
    let {editing, val = $bindable() }: Props = $props();

    $effect(() => {
        if (val === "") {
            val = undefined;
        }
    });

    $effect(() => {
        if (editing) resize();
    })

    let textarea: HTMLTextAreaElement | undefined = $state();

    function resize() {
        if (!textarea) return;
        textarea.style.height = "auto";
        textarea.style.height = textarea.scrollHeight + 2 + "px";
    }
</script>

{#if editing}
    <textarea placeholder="None" bind:value={val} rows="1" oninput={resize} bind:this={textarea}></textarea>
{:else}
    <div id="textarea">
        {#if val !== undefined}
            {val}
        {:else}
            <i>None</i>
        {/if}
    </div>
{/if}

<style>
    #textarea {
        margin: 0px;
        width: 100%;
        padding: 4px;
        background-color: var(--panel-3);
        -webkit-box-sizing: border-box;
        -moz-box-sizing: border-box;
        -ms-box-sizing: border-box;
        box-sizing: border-box;
        white-space: pre-wrap;
    }

    textarea {
        font-family: inherit;
        font-size: inherit;
        background-color: var(--panel-1);
        color: var(--white-1);
        border: 1px solid var(--accent-1-dark);
        border-radius: 2px;
        width: 100%;
        max-width: 100%;
        box-sizing: border-box;
        overflow-y: hidden;
    }
    textarea:focus {
        outline: none;
        border-color: var(--accent-1-light);
    }

</style>

