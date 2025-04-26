<script lang="ts">
	import { SignedIn, SignedOut, SignInButton, UserButton } from 'svelte-clerk';
	import { goto } from '$app/navigation';
</script>

<header class="header">
	<div class="header-left" on:click={() => goto('/')}>
		<div class="logo"></div>
		<h1 class="title">Health Quest</h1>
	</div>
	
	<div class="header-right">
		<SignedOut>
			<SignInButton />
		</SignedOut>
		
		<SignedIn>
			<div class="user-stats">
				<div class="stat">
					<span class="stat-icon">⭐</span>
					<span class="stat-value">120</span>
				</div>
				<div class="stat">
					<span class="stat-icon">🔥</span>
					<span class="stat-value">7</span>
				</div>
			</div>
			<UserButton />
		</SignedIn>
	</div>
</header>

<style>
.header {
	display: flex;
	justify-content: space-between;
	align-items: center;
	padding: 0.75rem 1rem;
	background: linear-gradient(135deg, rgba(52, 211, 153, 0.1), rgba(99, 102, 241, 0.1));
	backdrop-filter: blur(8px);
	border-bottom: 1px solid rgba(0, 0, 0, 0.05);
}

.header-left {
	display: flex;
	align-items: center;
	cursor: pointer;
}

.logo {
	font-size: 1.5rem;
	margin-right: 0.5rem;
}

.title {
	font-size: 1.25rem;
	font-weight: 700;
	background: linear-gradient(135deg, #34d399, #6366f1);
	-webkit-background-clip: text;
	background-clip: text;
	-webkit-text-fill-color: transparent;
}

.header-right {
	display: flex;
	align-items: center;
	gap: 1rem;
}

.user-stats {
	display: flex;
	gap: 0.75rem;
}

.stat {
	display: flex;
	align-items: center;
	gap: 0.25rem;
	background: rgba(255, 255, 255, 0.7);
	border-radius: 1rem;
	padding: 0.35rem 0.6rem;
	font-size: 0.85rem;
	font-weight: 600;
}

.stat-icon {
	font-size: 0.9rem;
}

.stat-value {
	color: hsl(222.2 47.4% 11.2%);
}

@media (max-width: 380px) {
	.title {
		font-size: 1.1rem;
	}
	
	.user-stats {
		gap: 0.5rem;
	}
	
	.stat {
		padding: 0.25rem 0.5rem;
	}
}
</style>