<script lang="ts">
    import { goto } from '$app/navigation';
    import { fade } from 'svelte/transition';
</script>

<div class="container" transition:fade={{ duration: 300 }}>
    <div class="header">
        <button class="back-button" on:click={() => goto('/dashboard')} aria-label="Go back to dashboard">
            <i class="fas fa-arrow-left"></i>
        </button>
        <h1 class="title">Mission Categories</h1>
    </div>

    <div class="mission-categories">
        <button class="category-button" on:click={() => goto('/exercise')} aria-label="Exercise missions">
            <div class="icon-container" style="background: linear-gradient(135deg, #4ade80, #3b82f6);">
                <i class="fas fa-dumbbell"></i>
            </div>
            <div class="button-content">
                <h2>Exercise</h2>
                <p>Physical activities to improve health</p>
            </div>
        </button>

        <button class="category-button" on:click={() => goto('/meditation')} aria-label="Meditation missions">
            <div class="icon-container" style="background: linear-gradient(135deg, #8b5cf6, #6366f1);">
                <i class="fas fa-om"></i>
            </div>
            <div class="button-content">
                <h2>Meditation</h2>
                <p>Mindfulness and relaxation practices</p>
            </div>
        </button>

        <button class="category-button" on:click={() => goto('/affirmations')} aria-label="Affirmation missions">
            <div class="icon-container" style="background: linear-gradient(135deg, #ff79c6, #f472b6);">
                <i class="fas fa-comment-dots"></i>
            </div>
            <div class="button-content">
                <h2>Affirmations</h2>
                <p>Positive statements to build confidence</p>
            </div>
        </button>

        <button class="category-button" on:click={() => goto('/sustenance')} aria-label="Sustenance missions">
            <div class="icon-container" style="background: linear-gradient(135deg, #f97316, #f59e0b);">
                <i class="fas fa-utensils"></i>
            </div>
            <div class="button-content">
                <h2>Sustenance</h2>
                <p>Nutrition and hydration goals</p>
            </div>
        </button>

        <button class="category-button" on:click={() => goto('/journaling')} aria-label="Journaling missions">
            <div class="icon-container" style="background: linear-gradient(135deg, #06b6d4, #3b82f6);">
                <i class="fas fa-book"></i>
            </div>
            <div class="button-content">
                <h2>Journaling</h2>
                <p>Reflective writing for mental clarity</p>
            </div>
        </button>
    </div>
</div>

<style>
    .container {
        display: flex;
        flex-direction: column;
        padding: 1rem;
        height: 100%;
        gap: 1.5rem;
        max-width: 600px;
        margin: 0 auto;
    }

    .header {
        display: flex;
        align-items: center;
        position: relative;
        margin-bottom: 0.5rem;
    }

    .back-button {
        position: absolute;
        left: 0;
        background: white;
        border: none;
        width: 40px;
        height: 40px;
        border-radius: 50%;
        display: flex;
        justify-content: center;
        align-items: center;
        box-shadow: 0 2px 6px rgba(0, 0, 0, 0.1);
        cursor: pointer;
        transition: transform 0.2s;
    }

    .back-button:hover {
        transform: translateY(-2px);
    }

    .title {
        flex: 1;
        text-align: center;
        font-size: 1.5rem;
        font-weight: 700;
        background: linear-gradient(135deg, #34d399, #6366f1);
        -webkit-background-clip: text;
        background-clip: text;
        color: transparent;
    }

    .mission-categories {
        display: flex;
        flex-direction: column;
        gap: 1rem;
        width: 100%;
    }

    .category-button {
        display: flex;
        align-items: center;
        gap: 1rem;
        padding: 1rem;
        border-radius: 16px;
        background: white;
        box-shadow: 0 4px 12px rgba(0, 0, 0, 0.05);
        border: none;
        text-align: left;
        cursor: pointer;
        transition: transform 0.2s, box-shadow 0.2s;
        width: 100%;
    }

    .category-button:hover {
        transform: translateY(-4px);
        box-shadow: 0 6px 20px rgba(0, 0, 0, 0.08);
    }

    .icon-container {
        min-width: 60px;
        height: 60px;
        border-radius: 12px;
        display: flex;
        justify-content: center;
        align-items: center;
        color: white;
        font-size: 1.5rem;
        box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
    }

    .button-content {
        flex: 1;
    }

    .button-content h2 {
        font-size: 1.2rem;
        font-weight: 600;
        margin: 0 0 0.3rem 0;
        color: #333;
    }

    .button-content p {
        font-size: 0.9rem;
        color: #666;
        margin: 0;
    }

    @media (max-width: 380px) {
        .category-button {
            padding: 0.8rem;
        }

        .icon-container {
            min-width: 50px;
            height: 50px;
            font-size: 1.2rem;
        }

        .button-content h2 {
            font-size: 1rem;
        }

        .button-content p {
            font-size: 0.8rem;
        }
    }
</style>