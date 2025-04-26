<script lang="ts">
    import "./global.css";
    import { ClerkProvider } from 'svelte-clerk';
    import Header from './components/Header.svelte';
  
    const { children } = $props();
  </script>
  
  <ClerkProvider publishableKey={import.meta.env.PUBLIC_CLERK_PUBLISHABLE_KEY}>
    <div class="app-container">
      <Header />
      <main class="main-content">
        {@render children()}
      </main>
    </div>
  </ClerkProvider>
  
  <style>
    .app-container {
      display: flex;
      flex-direction: column;
      min-height: 100vh;
      background-color: hsl(var(--color-background));
      color: hsl(var(--color-foreground));
    }
    
    .main-content {
      flex: 1;
      display: flex;
      flex-direction: column;
      align-items: center;
      justify-content: center;
      padding: 1rem;
    }
  </style>