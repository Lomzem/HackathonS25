<script>
  import { fade } from 'svelte/transition';

  const moodCategories = [
    {
      id: 1,
      name: "Happy",
      emoji: "😄",
      subMoods: [
        { name: "Joyful", emoji: "😊" },
        { name: "Excited", emoji: "🤩" },
        { name: "Grateful", emoji: "❤️" }
      ]
    },
    {
      id: 2,
      name: "Calm",
      emoji: "😌",
      subMoods: [
        { name: "Relaxed", emoji: "😌" },
        { name: "Peaceful", emoji: "🌿" },
        { name: "Content", emoji: "😊" }
      ]
    },
    {
      id: 3,
      name: "Sad",
      emoji: "😔",
      subMoods: [
        { name: "Down", emoji: "😢" },
        { name: "Lonely", emoji: "🌑" },
        { name: "Tired", emoji: "😴" }
      ]
    },
    {
      id: 4,
      name: "Stressed",
      emoji: "😰",
      subMoods: [
        { name: "Anxious", emoji: "😟" },
        { name: "Overwhelmed", emoji: "😵‍💫" },
        { name: "Worried", emoji: "😨" }
      ]
    }
  ];

  let currentView = 'main';
  let selectedCategory = null;
  let selectedSubMood = null;

  function showSubMoods(category) {
    selectedCategory = category;
    currentView = 'sub';
  }

  function goBackToMain() {
    currentView = 'main';
    selectedSubMood = null;
  }

  function selectSubMood(subMood) {
    selectedSubMood = subMood;
  }
</script>

<div class="container">
  {#if currentView === 'main'}
    <div in:fade>
      <h2>How are you feeling?</h2>
      <div class="grid">
        {#each moodCategories as category}
          <button class="mood-btn" on:click={() => showSubMoods(category)}>
            <span class="emoji">{category.emoji}</span>
            <span>{category.name}</span>
          </button>
        {/each}
      </div>
    </div>
  {:else if currentView === 'sub' && selectedCategory}
    <div in:fade>
      <button class="back-btn" on:click={goBackToMain}>← Back</button>
      <h2>{selectedCategory.name} {selectedCategory.emoji}</h2>
      
      <div class="grid">
        {#each selectedCategory.subMoods as subMood}
          <button 
            class="mood-btn"
            class:selected={selectedSubMood === subMood}
            on:click={() => selectSubMood(subMood)}
          >
            <span class="emoji">{subMood.emoji}</span>
            <span>{subMood.name}</span>
          </button>
        {/each}
      </div>

      {#if selectedSubMood}
        <div class="selected">
          <p>You're feeling {selectedSubMood.name} {selectedSubMood.emoji}</p>
          <button class="save-btn">Save</button>
        </div>
      {/if}
    </div>
  {/if}
</div>

<style>
  .container {
    max-width: 600px;
    margin: 0 auto;
    padding: 20px;
  }

  .grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(120px, 1fr));
    gap: 12px;
    margin: 20px 0;
  }

  .mood-btn {
    display: flex;
    flex-direction: column;
    align-items: center;
    padding: 12px;
    border-radius: 8px;
    background: #f5f5f5;
    border: none;
    cursor: pointer;
    transition: transform 0.2s;
  }

  .mood-btn:hover {
    transform: translateY(-2px);
    background: #eee;
  }

  .mood-btn.selected {
    background: #e0e7ff;
  }

  .emoji {
    font-size: 24px;
    margin-bottom: 4px;
  }

  .back-btn {
    background: none;
    border: none;
    padding: 8px;
    cursor: pointer;
    color: #666;
  }

  .selected {
    text-align: center;
    margin-top: 20px;
    padding: 16px;
    background: #e0e7ff;
    border-radius: 8px;
  }

  .save-btn {
    background: #4f46e5;
    color: white;
    border: none;
    padding: 8px 16px;
    border-radius: 4px;
    cursor: pointer;
  }

  .save-btn:hover {
    background: #4338ca;
  }

  h2 {
    text-align: center;
    margin: 16px 0;
  }

  @media (max-width: 480px) {
    .grid {
      grid-template-columns: repeat(2, 1fr);
    }
  }
</style>