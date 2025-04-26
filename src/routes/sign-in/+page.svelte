<script lang="ts">
	import { SignIn } from 'svelte-clerk';
	import { goto } from '$app/navigation';
  </script>
  
  <div class="device">
	<div class="screen">
	  <div class="auth-container">
		<div class="auth-header">
		  <div class="logo">Health Quest</div>
		  <h2>Welcome Back!</h2>
		  <p class="tagline">Resume your wellness journey</p>
		</div>
		
		<div class="sign-in-form">
		  <SignIn redirectUrl="/dashboard" />
		</div>
  
		<div class="auth-footer">
		  <p>
			New to Health Quest?
			<button class="text-button" on:click={() => goto('/sign-up')}>
			  Create Account
			</button>
		  </p>
		</div>
	  </div>
	</div>
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
		0 20px 40px rgba(0,0,0,0.4),
		inset 0 0 0 1px rgba(255,255,255,0.1);
	  overflow: hidden;
	}
	.device::before {
	  content: '';
	  position: absolute;
	  top: 10%;
	  left: -20%;
	  width: 140%;
	  height: 60%;
	  background: radial-gradient(
		circle at 30% 30%,
		rgba(255,255,255,0.15),
		transparent 60%
	  );
	  transform: rotate(-25deg);
	  pointer-events: none;
	}
	.device::after {
	  content: '';
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
	  background: #fff;
	  border-radius: 44px;
	  overflow: hidden;
	  display: flex;
	  flex-direction: column;
	  justify-content: center;
	  align-items: center;
	}
  
	.screen .auth-container {
	  width: 100%;
	  max-width: 400px;
	  height: 100%;
	  padding: 2rem 1rem;
	  background: linear-gradient(
		135deg,
		rgba(52, 211, 153, 0.05),
		rgba(99, 102, 241, 0.05)
	  );
	  display: flex;
	  flex-direction: column;
	  justify-content: center;
	  align-items: center;
	}
  
	.screen .auth-header {
	  text-align: center;
	  margin-bottom: 2rem;
	}
	.screen .logo {
	  font-size: 2.5rem;
	  font-weight: bold;
	  margin-bottom: 1rem;
	  color: #34d399;
	}
	.screen h2 {
	  font-size: 1.75rem;
	  margin-bottom: 0.5rem;
	  background: linear-gradient(135deg, #34d399, #6366f1);
	  -webkit-background-clip: text;
	  background-clip: text;
	  -webkit-text-fill-color: transparent;
	}
	.screen .tagline {
	  font-size: 1rem;
	  opacity: 0.75;
	  margin-bottom: 1.5rem;
	}
  
	.screen .sign-in-form {
	  width: 100%;
	  overflow: hidden;
	}
  
	.screen .auth-footer {
	  margin-top: 2rem;
	  text-align: center;
	  font-size: 0.9rem;
	}
	.screen .text-button {
	  background: none;
	  border: none;
	  padding: 0;
	  font-size: inherit;
	  color: #6366f1;
	  font-weight: 600;
	  cursor: pointer;
	}
	.screen .text-button:hover {
	  text-decoration: underline;
	}
  
	.screen .ClerkSignIn form {
	  width: 100%;
	  border-radius: 8px;
	  box-shadow: 0 4px 8px rgba(0,0,0,0.1);
	  padding: 1.5rem;
	  background: white;
	}
	.screen .ClerkSignIn input,
	.screen .ClerkSignIn select {
	  width: 100%;
	  padding: 0.8rem;
	  margin-bottom: 1rem;
	  border: 1px solid #ccc;
	  border-radius: 8px;
	  font-size: 1rem;
	}
	.screen .ClerkSignIn button {
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
	.screen .ClerkSignIn button:hover {
	  background-color: #6366f1;
	}
  
	@media (max-width: 320px) {
	  .screen .logo {
		font-size: 2rem;
	  }
	  .screen h2 {
		font-size: 1.5rem;
	  }
	}
  </style>
  