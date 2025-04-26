<script lang="ts">
    import { HeartIcon } from "@lucide/svelte";

    let {
        title,
        description,
        favorited,
        minuteDuration,
        secondDuration,
    }: {
        title: string;
        description: string;
        favorited: boolean;
        minuteDuration: number;
        secondDuration: number;
    } = $props();
</script>

<card>
    <div class="card-content">
        <h2>{title}</h2>
        <p>{description}</p>
        <p class="duration">
            {minuteDuration}:{String(secondDuration).padStart(2, "0")}
        </p>
    </div>
    {#if favorited}
        <HeartIcon color="#ff0000" fill={favorited ? "#ff0000" : ""} />
    {:else}
        <HeartIcon />
    {/if}
</card>

<style>
    card {
        padding: 1rem 1.5rem;
        width: 100%;
        border-radius: var(--border-radius);
        background: var(--color-card);
        box-shadow: 0 4px 6px rgba(0, 0, 0, 0.05);
        transition: transform 0.2s;
        display: flex;
        align-items: center;
        justify-content: space-between;
    }
    .duration {
        color: var(--color-primary);
    }
    h2 {
        font-weight: 400;
        font-size: 1.5rem;
    }
    p {
        font-size: 1rem;
    }
</style>
