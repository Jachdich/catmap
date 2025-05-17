<script lang="ts">
    import { type Cat } from "./cat";
    import "./popup.css";
    interface Props {
        cat: Cat;
        close: () => void;
    }
    let { cat, close } : Props = $props();
    let editing = $state(false);
    function change(ev: InputEvent) {
        console.log(ev);
        cat.markings = ev.data === null ? undefined : ev.target.innerText;
        console.log(cat.markings);
    }
</script>

<div id="main" class="popup centre-window">
    <button id="close" onclick={close}>X</button>
    <div id="content">
        <div id="left-content">
            <h1>{cat.name}</h1>
            <h2>Colour</h2>
            <p contenteditable={editing}>{cat.colour}</p>
            <h2>Distinctive Markings</h2>
            <p contenteditable={editing} oninput={change}>
                {#if cat.markings !== undefined}
                    {cat.markings}
                {:else}
                    <i>None</i>
                {/if}
            </p>
            <h2>Collar</h2>
            <p contenteditable={editing}>
                {#if cat.collar !== undefined}
                    {cat.collar}
                {:else}
                    <i>None</i>
                {/if}
            </p>
            <h2>Description</h2>
            <p contenteditable={editing}>{cat.description}</p>
            <h2>Friendliness</h2>
            <p>{cat.friendliness() === undefined ? "Unknown" : `${cat.friendliness()} (${cat.friendliness_desc()})`}</p>

            <button onclick={() => editing = true}>Edit</button>
        </div>
        <div id="right-content">
            {#each cat.sightings as s}
                <div id="sighting-card">
                    <h2>Notes</h2>
                    <p>{s.notes}</p>
                    {#each s.image_urls as url}
                        <img id="sighting-image" src={url} alt="Cat sighting"/>
                    {/each}
                </div>
            {/each}
        </div>
    </div>
</div>


<style>
    #main {
        display: flex;
        flex-direction: column;
        width: 85%;
        height: 85%;
    }

    #content {
        display: flex;
        flex-direction: row;
        height: 100%;
        padding-bottom: 20px;
        box-sizing: border-box;
    }
    
    #left-content {
        width: 50%;
        min-width: 50%;
        padding-right: 5px;
    }

    #right-content {
        height: 100%;
        overflow-y: scroll;
        padding-left: 5px;
    }

    #sighting-image {
        width: 100%;
    }

    p {
        margin: 0px;
        width: 100%;
        padding: 4px;
        background-color: var(--panel-1);
        -webkit-box-sizing: border-box;
        -moz-box-sizing: border-box;
        -ms-box-sizing: border-box;
        box-sizing: border-box;
    }

    h1 {
        font-size: 18px;
        margin-top: 4px;
    }
    h2 {
        font-size: 14px;
    }

    #close {
        width: 24px;
        margin-left: auto;
    }
</style>
