<script lang="ts">
    import { goto } from "$app/navigation";
</script>

<div class="screen">
    <section class="mobile-landing">
        <div class="logo">🩺</div>
        <h1 class="title">Health Quest</h1>
        <p class="tagline">
            Turn self-care into an adventure. Complete missions, earn rewards,
            and level up your wellbeing!
        </p>

        <div class="features">
            <div class="feature">
                <span class="feature-icon">💧</span>
                <span class="feature-text">Hydration</span>
            </div>
            <div class="feature">
                <span class="feature-icon">🧘</span>
                <span class="feature-text">Meditation</span>
            </div>
            <div class="feature">
                <span class="feature-icon">📝</span>
                <span class="feature-text">Journal</span>
            </div>
            <div class="feature">
                <span class="feature-icon">✨</span>
                <span class="feature-text">Affirmations</span>
            </div>
        </div>

        <div class="button-group">
            <button class="btn primary" on:click={() => goto("./sign-in")}>
                Login
            </button>
            <button class="btn secondary" on:click={() => goto("./sign-up")}>
                Create Account
            </button>
        </div>
    </section>
</div>

<style>
    .device {
        position: relative;
        width: 360px;
        max-width: 90vw;
        aspect-ratio: 375 / 812;
        margin: 2rem auto;
        background: #000;
        border-radius: 50px;
        box-shadow:
            0 20px 40px rgba(0, 0, 0, 0.4),
            inset 0 0 0 1px rgba(255, 255, 255, 0.1);
        overflow: hidden;
    }

    .device::before {
        content: "";
        position: absolute;
        top: 10%;
        left: -20%;
        width: 140%;
        height: 60%;
        background: radial-gradient(
            circle at 30% 30%,
            rgba(255, 255, 255, 0.15),
            transparent 60%
        );
        transform: rotate(-25deg);
        pointer-events: none;
    }

    .device::after {
        content: "";
        position: absolute;
        top: 0;
        left: 50%;
        width: 100px;
        height: 6px;
        background: #111;
        border-bottom-left-radius: 3px;
        border-bottom-right-radius: 3px;
        transform: translateX(-50%);
    }

    .screen {
        width: 100%;
        height: 100%;
        border-radius: 44px;
        overflow: hidden;
        display: flex;
        flex-direction: column;
    }

    .screen .mobile-landing {
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        padding: 1.5rem;
        text-align: center;
        color: hsl(var(--color-foreground), 1);
    }

    .screen .logo {
        font-size: 3.5rem;
        margin-bottom: 1rem;
        animation: pulse 2s infinite;
    }

    .screen .title {
        font-size: 2.5rem;
        font-weight: 800;
        margin-bottom: 0.5rem;
        background: linear-gradient(135deg, #34d399, #6366f1);
        -webkit-background-clip: text;
        background-clip: text;
        -webkit-text-fill-color: transparent;
    }

    .screen .tagline {
        font-size: 1rem;
        opacity: 0.85;
        margin-bottom: 2rem;
        max-width: 320px;
        line-height: 1.6;
    }

    .screen .features {
        display: flex;
        flex-wrap: wrap;
        justify-content: center;
        gap: 0.75rem;
        margin-bottom: 2rem;
        max-width: 320px;
    }

    .screen .feature {
        display: flex;
        flex-direction: column;
        align-items: center;
        width: calc(50% - 0.75rem);
        background: rgba(255, 255, 255, 0.7);
        border-radius: 1rem;
        padding: 1rem 0.5rem;
        box-shadow: 0 4px 6px rgba(0, 0, 0, 0.05);
        transition: transform 0.2s;
    }
    .screen .feature:hover {
        transform: translateY(-2px);
    }
    .screen .feature-icon {
        font-size: 1.75rem;
        margin-bottom: 0.5rem;
    }
    .screen .feature-text {
        font-size: 0.9rem;
        font-weight: 600;
    }

    .screen .button-group {
        display: flex;
        flex-direction: column;
        gap: 1rem;
        width: 100%;
        max-width: 280px;
    }

    .screen .btn {
        padding: 0.9rem 1rem;
        font-size: 1rem;
        border-radius: var(--border-radius);
        border: none;
        cursor: pointer;
        transition: all 0.2s ease;
        font-weight: 600;
    }

    .screen .btn.primary {
        background: linear-gradient(135deg, #34d399, #6366f1);
        color: white;
        box-shadow: 0 4px 10px rgba(99, 102, 241, 0.3);
    }
    .screen .btn.primary:hover {
        transform: translateY(-2px);
        box-shadow: 0 6px 15px rgba(99, 102, 241, 0.4);
    }

    .screen .btn.secondary {
        background: rgba(255, 255, 255, 0.7);
        color: hsl(222.2 47.4% 11.2%);
        border: 1px solid rgba(0, 0, 0, 0.1);
    }
    .screen .btn.secondary:hover {
        background: rgba(255, 255, 255, 0.9);
    }

    /* PULSE KEYFRAMES */
    @keyframes pulse {
        0% {
            transform: scale(1);
        }
        50% {
            transform: scale(1.05);
        }
        100% {
            transform: scale(1);
        }
    }

    @media (max-width: 320px) {
        .screen .title {
            font-size: 2rem;
        }
        .screen .feature {
            padding: 0.75rem 0.5rem;
        }
        .screen .feature-icon {
            font-size: 1.5rem;
        }
    }
</style>
