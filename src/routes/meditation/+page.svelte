<script>
    import MeditationCard from "./MeditationCard.svelte";
</script>

<section>
    <h1 class="title">Meditation</h1>
    <ul>
        <MeditationCard
            title="Mindfulness"
            description="Focus on the present moment"
            favorited={false}
            minuteDuration={3}
            secondDuration={30}
        />
        <MeditationCard
            title="Heartbeat"
            description="Connect with your body"
            favorited={true}
            minuteDuration={2}
            secondDuration={0}
        />
        <MeditationCard
            title="Body Scan"
            description="Pay attention to your body"
            favorited={false}
            minuteDuration={5}
            secondDuration={0}
        />
    </ul>
</section>

<style>
    section {
        justify-self: start;
        width: 100%;
        padding-left: 1rem;
        padding-right: 1rem;
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 2rem;
    }
    h1 {
        color: red;
    }
    ul {
        padding-left: 0;
        display: flex;
        flex-direction: column;
        gap: 1.5rem;
        width: 100%;
    }
</style>
