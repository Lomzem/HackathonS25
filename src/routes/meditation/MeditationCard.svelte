<script lang="ts">
    import { HeartIcon } from "@lucide/svelte";

    let {
        title,
        description,
        favorited,
    }: {
        title: string;
        description: string;
        favorited: boolean;
    } = $props();
</script>

<card>
    <div class="card-content">
        <h2>{title}</h2>
        <p>{description}</p>
    </div>
    <HeartIcon color={favorited ? "#ff0000" : "#000000"} />
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
    h2 {
        font-weight: 400;
        font-size: 1.5rem;
    }
    p {
        font-size: 1rem;
    }
</style>
