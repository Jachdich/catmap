<script lang="ts">
    interface Props {
        editing: boolean,
        val: string | undefined,
    }
    let {editing, val}: Props = $props();
</script>


{#if editing}
    {#if val !== undefined}
        <input type="text" bind:value={val}/>
    {:else}
        <input type="text" placeholder="None" bind:value={val}/>
    {/if}
{:else}
    <p>
        {#if val !== undefined}
            {val}
        {:else}
            <i>None</i>
        {/if}
    </p>
{/if}

<style>
    p {
        margin: 0px;
        width: 100%;
        padding: 4px;
        background-color: var(--panel-3);
        -webkit-box-sizing: border-box;
        -moz-box-sizing: border-box;
        -ms-box-sizing: border-box;
        box-sizing: border-box;
    }

    input {
        background-color: var(--panel-1);
        color: var(--white-1);
        border: 1px solid var(--accent-1-dark);
        border-radius: 2px;
        font-size: 16px;
        width: 100%;
    }
    input:focus {
        outline: none;
        border-color: var(--accent-1-light);
    }

</style>

