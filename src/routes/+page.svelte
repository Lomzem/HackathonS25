<script lang="ts">
	import { goto } from '$app/navigation';
</script>

<section class="mobile-landing">
	<div class="logo"></div>
	<h1 class="title">Health Quest</h1>
	<p class="tagline">Turn self-care into an adventure. Complete missions, earn rewards, and level up your wellbeing!</p>

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
		<button class="btn primary" on:click={() => goto('./sign-in')}>
			Login
		</button>
		<button class="btn secondary" on:click={() => goto('./sign-up')}>
			Create Account
		</button>
	</div>
</section>

<style>
.mobile-landing {
	height: 100dvh;
	width: 100dvw;
	display: flex;
	flex-direction: column;
	justify-content: center;
	align-items: center;
	padding: 1.5rem;
	background: linear-gradient(135deg, hsl(190, 85%, 93%), hsl(250, 85%, 93%));
	color: hsl(var(--color-foreground));
	text-align: center;
}

.logo {
	font-size: 3.5rem;
	margin-bottom: 1rem;
	animation: pulse 2s infinite;
}

.title {
	font-size: 2.5rem;
	font-weight: 800;
	margin-bottom: 0.5rem;
	background: linear-gradient(135deg, #34d399, #6366f1);
	-webkit-background-clip: text;
	background-clip: text;
	-webkit-text-fill-color: transparent;
}

.tagline {
	font-size: 1rem;
	opacity: 0.85;
	margin-bottom: 2rem;
	max-width: 320px;
	line-height: 1.6;
}

.features {
	display: flex;
	flex-wrap: wrap;
	justify-content: center;
	gap: 0.75rem;
	margin-bottom: 2rem;
	max-width: 320px;
}

.feature {
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

.feature:hover {
	transform: translateY(-2px);
}

.feature-icon {
	font-size: 1.75rem;
	margin-bottom: 0.5rem;
}

.feature-text {
	font-size: 0.9rem;
	font-weight: 600;
}

.button-group {
	display: flex;
	flex-direction: column;
	gap: 1rem;
	width: 100%;
	max-width: 280px;
}

.btn {
	padding: 0.9rem 1rem;
	font-size: 1rem;
	border-radius: var(--border-radius);
	border: none;
	cursor: pointer;
	transition: all 0.2s ease;
	font-weight: 600;
}

.btn.primary {
	background: linear-gradient(135deg, #34d399, #6366f1);
	color: white;
	box-shadow: 0 4px 10px rgba(99, 102, 241, 0.3);
}

.btn.primary:hover {
	transform: translateY(-2px);
	box-shadow: 0 6px 15px rgba(99, 102, 241, 0.4);
}

.btn.secondary {
	background: rgba(255, 255, 255, 0.7);
	color: hsl(222.2 47.4% 11.2%);
	border: 1px solid rgba(0, 0, 0, 0.1);
}

.btn.secondary:hover {
	background: rgba(255, 255, 255, 0.9);
}

@keyframes pulse {
	0% { transform: scale(1); }
	50% { transform: scale(1.05); }
	100% { transform: scale(1); }
}

@media (max-width: 320px) {
	.title {
		font-size: 2rem;
	}
	
	.feature {
		padding: 0.75rem 0.5rem;
	}
	
	.feature-icon {
		font-size: 1.5rem;
	}
}
</style>