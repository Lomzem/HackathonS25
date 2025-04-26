<script>
	import { SignIn } from 'svelte-clerk';
	import { goto } from '$app/navigation';
</script>

<div class="auth-container">
	<div class="auth-header">
		<div class="logo">Health Quest</div>
		<h2>Welcome Back!</h2>
		<p class="tagline">Resume your wellness journey</p>
	</div>
	
	<div class="sign-in-form">
		<SignIn />
	</div>

	<div class="auth-footer">
		<p>New to Health Quest? <button class="text-button" on:click={() => goto('/sign-up')}>Create Account</button></p>
	</div>
</div>

<style>
.auth-container {
	width: 100%;
	height: 100vh;
	max-width: 400px;
	margin: auto;
	padding: 2rem 1rem;
	background: linear-gradient(135deg, rgba(52, 211, 153, 0.05), rgba(99, 102, 241, 0.05));
	display: flex;
	flex-direction: column;
	justify-content: center;
	align-items: center;
}

.auth-header {
	text-align: center;
	margin-bottom: 2rem;
}

.logo {
	font-size: 2.5rem;
	font-weight: bold;
	margin-bottom: 1rem;
	color: #34d399;
}

h2 {
	font-size: 1.75rem;
	margin-bottom: 0.5rem;
	background: linear-gradient(135deg, #34d399, #6366f1);
	-webkit-background-clip: text;
	background-clip: text;
	-webkit-text-fill-color: transparent;
}

.tagline {
	font-size: 1rem;
	opacity: 0.75;
	margin-bottom: 1.5rem;
}

.sign-in-form {
	width: 100%;
	overflow: hidden;
}

.auth-footer {
	margin-top: 2rem;
	text-align: center;
	font-size: 0.9rem;
}

.text-button {
	background: none;
	border: none;
	padding: 0;
	font-size: inherit;
	color: #6366f1;
	font-weight: 600;
	cursor: pointer;
}

.text-button:hover {
	text-decoration: underline;
}

.ClerkSignIn form {
	width: 100%;
	border-radius: 8px;
	box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
	padding: 1.5rem;
	background: white;
}

.ClerkSignIn input,
.ClerkSignIn select {
	width: 100%;
	padding: 0.8rem;
	margin-bottom: 1rem;
	border: 1px solid #ccc;
	border-radius: 8px;
	font-size: 1rem;
}

.ClerkSignIn button {
	width: 100%;
	padding: 0.8rem;
	border: none;
	border-radius: 8px;
	background-color: #34d399;
	color: white;
	font-size: 1rem;
	font-weight: bold;
	cursor: pointer;
}

.ClerkSignIn button:hover {
	background-color: #6366f1;
}
</style>
