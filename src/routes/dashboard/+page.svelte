<script lang="ts">
    import { onMount } from "svelte";
    import { fade, scale, fly } from "svelte/transition";
    import {
        SignedIn,
        SignedOut,
        SignInButton,
        UserButton,
    } from "svelte-clerk";
    import { slide } from "svelte/transition";

    let progressData = {
        completed: 4,
        total: 6,
        streak: 7,
        points: 450,
        level: 5,
        nextLevelPoints: 500,
    };

    $: progressWidth = (progressData.completed / progressData.total) * 100;
    $: levelProgress = Math.min(
        100,
        (progressData.points / progressData.nextLevelPoints) * 100,
    );

    let toast = { message: "", show: false, type: "info" };
    let currentTime = new Date().toLocaleTimeString([], {
        hour: "2-digit",
        minute: "2-digit",
    });
    let selectedTab = "today";
    let showMissionDetails = false;
    let currentMission = null;
    let showRewards = false;
    let isSpeaking = false;
    let affirmationText = "I am strong, capable, and worthy of good things";

    let currentMood = "";
    let moodOptions = [
        { emoji: "😊", label: "Happy", color: "#FFD700" },
        { emoji: "😌", label: "Calm", color: "#90EE90" },
        { emoji: "😐", label: "Neutral", color: "#ADD8E6" },
        { emoji: "😔", label: "Sad", color: "#A0A0A0" },
        { emoji: "😡", label: "Angry", color: "#FF6347" },
        { emoji: "😰", label: "Anxious", color: "#9370DB" },
    ];

    let waterGlasses = 4;
    const totalWaterGoal = 8;

    onMount(() => {
        const timeInterval = setInterval(() => {
            currentTime = new Date().toLocaleTimeString([], {
                hour: "2-digit",
                minute: "2-digit",
            });
        }, 60000);

        return () => clearInterval(timeInterval);
    });

    function showToast(msg: string, type = "info") {
        toast = { message: msg, show: true, type };
        setTimeout(() => (toast.show = false), 2500);
    }

    const handleNavClick = (name: string) => {
        if (name === "add") {
            showToast("Create a new mission");
        } else {
            showToast(`Navigating to ${name}`);
        }
    };

    const openMissionDetails = (mission) => {
        currentMission = mission;
        showMissionDetails = true;
    };

    const completeMission = (missionId) => {
        showMissionDetails = false;
        progressData.completed += 1;
        progressData.points += 50;

        const missionIndex = todaysMissions.findIndex(
            (m) => m.id === missionId,
        );
        if (missionIndex !== -1) {
            const mission = {
                ...todaysMissions[missionIndex],
                completed: true,
            };
            todaysMissions.splice(missionIndex, 1);
            completedMissions = [...completedMissions, mission];
            todaysMissions = todaysMissions;

            showToast("Mission completed! +50 points", "success");

            if (progressData.points >= progressData.nextLevelPoints) {
                progressData.level += 1;
                progressData.nextLevelPoints += 100;
                showToast(
                    "🎉 Level Up! You reached level " + progressData.level,
                    "success",
                );
            }
        }
    };

    const startVoiceDetection = () => {
        isSpeaking = true;
        showToast("Voice detected! Say your affirmation...", "info");

        setTimeout(() => {
            isSpeaking = false;
            showToast("Great job! Affirmation completed", "success");
        }, 3000);
    };

    const updateWater = (change) => {
        const newValue = Math.max(
            0,
            Math.min(totalWaterGoal, waterGlasses + change),
        );

        if (newValue !== waterGlasses) {
            waterGlasses = newValue;

            const waterMission = todaysMissions.find((m) => m.id === "water");
            if (waterMission) {
                waterMission.subtitle = `${waterGlasses}/${totalWaterGoal} glasses`;
                todaysMissions = [...todaysMissions];
            }

            showToast(
                `Water intake updated: ${waterGlasses}/${totalWaterGoal} glasses`,
                "info",
            );

            if (
                waterGlasses === totalWaterGoal &&
                waterMission &&
                !waterMission.completed
            ) {
                completeMission("water");
            }
        }
    };

    const setMood = (mood) => {
        currentMood = mood.label;
        showToast(`Mood logged: ${mood.label}`, "success");

        if (mood.label === "Anxious" || mood.label === "Sad") {
            const breathingMission = {
                id: "breathing",
                icon: "fa-wind",
                title: "Deep Breathing",
                subtitle: "2 minute exercise",
                color: "#9370DB",
                completed: false,
                description: "Take 5 deep breaths, holding each for 4 seconds.",
                reward: 30,
                category: "mindfulness",
            };

            if (!todaysMissions.some((m) => m.id === "breathing")) {
                todaysMissions = [...todaysMissions, breathingMission];
                showToast(
                    "Added a breathing exercise to help with your mood",
                    "info",
                );
            }
        }
    };

    const toggleRewards = () => {
        showRewards = !showRewards;
    };

    let todaysMissions = [
        {
            id: "meditation",
            icon: "fa-om",
            title: "Morning Meditation",
            subtitle: "Take 10 minutes out of your day to calm your mind!",
            color: "#8b5cf6",
            completed: false,
            description:
                "Start your day with a calm mind. Sit comfortably, focus on your breath for 10 minutes.",
            reward: 50,
            category: "mindfulness",
        },
        {
            id: "water",
            icon: "fa-tint",
            title: "Stay Hydrated",
            subtitle:
                "Drink 8 glasses of water throughout the day to stay hydrated and energized.",
            color: "#34d399",
            completed: false,
            description:
                "Drink 8 glasses of water throughout the day to stay hydrated and energized.",
            reward: 40,
            category: "physical",
        },
        {
            id: "affirmation",
            icon: "fa-microphone",
            title: "Daily Affirmation",
            subtitle:
                "Speak your affirmation aloud to reinforce positive thoughts.",
            color: "#ff79c6",
            completed: false,
            description:
                "Speak your daily affirmation aloud to reinforce positive thoughts.",
            reward: 30,
            category: "mental",
        },
        {
            id: "meal",
            icon: "fa-utensils",
            title: "Healthy Meal",
            subtitle: "Show off your healthy meal to the world!",
            color: "#f97316",
            completed: false,
            description:
                "Have a balanced meal and take a photo to verify completion.",
            reward: 45,
            category: "physical",
        },
    ];

    let completedMissions = [
        {
            id: "journal",
            icon: "fa-check",
            title: "Daily Journal",
            subtitle: "Entry completed",
            color: "#0cff00",
            completed: true,
            description: "Write about your thoughts and feelings for the day.",
            reward: 50,
            category: "mental",
        },
    ];

    let characters = [
        { id: 1, name: "Zen Master", icon: "🧘", unlocked: true, level: 3 },
        { id: 2, name: "Hydration Hero", icon: "💧", unlocked: true, level: 2 },
        {
            id: 3,
            name: "Gratitude Guardian",
            icon: "🙏",
            unlocked: false,
            level: 0,
        },
        {
            id: 4,
            name: "Sleep Sorcerer",
            icon: "💤",
            unlocked: false,
            level: 0,
        },
        {
            id: 5,
            name: "Nutrition Ninja",
            icon: "🥗",
            unlocked: true,
            level: 1,
        },
        { id: 6, name: "Mood Master", icon: "😊", unlocked: false, level: 0 },
    ];

    let upcomingMissions = [
        {
            id: "stretch",
            icon: "fa-running",
            title: "Stretch Break",
            subtitle: "5 min stretch",
            color: "#4ade80",
            category: "physical",
            timeToUnlock: "3h",
        },
        {
            id: "gratitude",
            icon: "fa-heart",
            title: "Gratitude List",
            subtitle: "List 3 things",
            color: "#f472b6",
            category: "mental",
            timeToUnlock: "6h",
        },
    ];

    const navItems = [
        { icon: "fa-home", label: "Home", active: true },
        { icon: "fa-compass", label: "Discover" },
        { icon: "fa-plus", label: "", isAdd: true },
        { icon: "fa-chart-line", label: "Progress" },
        { icon: "fa-user", label: "Profile" },
    ];
</script>

<div class="app-container">
    <header class="header">
        <div class="header-left">
            <div class="logo" aria-hidden="true">🩺</div>
            <h1 class="title">Health Quest</h1>
        </div>
        <div class="header-right">
            <SignedOut>
                <SignInButton>
                    <button class="btn sign-in">Sign In</button>
                </SignInButton>
            </SignedOut>
            <SignedIn>
                <div class="user-stats">
                    <div class="stat" title="Points">
                        <span class="icon">⭐</span>
                        <span>{progressData.points}</span>
                    </div>
                    <div class="stat" title="Streak">
                        <span class="icon">🔥</span>
                        <span>{progressData.streak}</span>
                    </div>
                    <div
                        class="stat user-level"
                        title="Level {progressData.level}"
                    >
                        <span>Lvl {progressData.level}</span>
                    </div>
                </div>
                <UserButton />
            </SignedIn>
        </div>
    </header>

    <section class="section card level-card">
        <div class="level-header">
            <h3>Level {progressData.level}</h3>
            <span>{progressData.points}/{progressData.nextLevelPoints} XP</span>
        </div>
        <div
            class="progress-bar"
            role="progressbar"
            aria-valuenow={progressData.points}
            aria-valuemin="0"
            aria-valuemax={progressData.nextLevelPoints}
        >
            <div class="fill" style:width="{levelProgress}%"></div>
            <div class="glow" style:width="{levelProgress}%"></div>
        </div>
    </section>

    <section class="section card mood-card">
        <h3>How are you feeling today?</h3>
        <div class="mood-selection">
            {#each moodOptions as mood}
                <button
                    class="mood-option {currentMood === mood.label
                        ? 'selected'
                        : ''}"
                    style:background={currentMood === mood.label
                        ? mood.color
                        : "transparent"}
                    on:click={() => setMood(mood)}
                >
                    <span class="mood-emoji">{mood.emoji}</span>
                    <span class="mood-label">{mood.label}</span>
                </button>
            {/each}
        </div>
    </section>

    <section class="section card progress-card">
        <div class="header-row">
            <h3>Daily Quest Progress</h3>
            <span class="counter"
                >{progressData.completed}/{progressData.total}</span
            >
        </div>
        <div
            class="progress-bar"
            role="progressbar"
            aria-valuenow={progressData.completed}
            aria-valuemin="0"
            aria-valuemax={progressData.total}
        >
            <div class="fill" style:width="{progressWidth}%"></div>
            <div class="glow" style:width="{progressWidth}%"></div>
        </div>
        <div class="markers">
            {#each Array(progressData.total) as _, i}
                <div
                    class="marker {i < progressData.completed ? 'done' : ''}"
                    aria-hidden="true"
                ></div>
            {/each}
        </div>
        <div class="stats-row">
            <div class="stat-row">
                <i class="fas fa-fire"></i> Streak: {progressData.streak}d
            </div>
            <div class="stat-row">
                <i class="fas fa-gem"></i> Points: {progressData.points}
            </div>
        </div>
    </section>

    <div class="tabs">
        <button
            class="tab {selectedTab === 'today' ? 'active' : ''}"
            on:click={() => (selectedTab = "today")}
        >
            Today's Missions
        </button>
        <button
            class="tab {selectedTab === 'completed' ? 'active' : ''}"
            on:click={() => (selectedTab = "completed")}
        >
            Completed
        </button>
        <button
            class="tab {selectedTab === 'upcoming' ? 'active' : ''}"
            on:click={() => (selectedTab = "upcoming")}
        >
            Upcoming
        </button>
    </div>

    {#if selectedTab === "today"}
        <section class="section mission-section" transition:fade>
            <div class="missions-grid">
                {#each todaysMissions as mission}
                    <div
                        class="mission-item"
                        on:click={() => openMissionDetails(mission)}
                        on:keydown={(e) =>
                            e.key === "Enter" && openMissionDetails(mission)}
                        tabindex="0"
                        role="button"
                        aria-label="View details for {mission.title}"
                    >
                        <div class="m-icon" style:background={mission.color}>
                            <i class="fas {mission.icon}"></i>
                        </div>
                        <div class="m-details">
                            <h4>{mission.title}</h4>
                            <p>{mission.subtitle}</p>
                            <div class="m-category">{mission.category}</div>
                        </div>
                    </div>
                {/each}
            </div>
        </section>
    {/if}

    {#if selectedTab === "completed"}
        <section class="section mission-section" transition:fade>
            {#if completedMissions.length > 0}
                {#each completedMissions as mission}
                    <div
                        class="mission-item completed"
                        on:click={() => openMissionDetails(mission)}
                        on:keydown={(e) =>
                            e.key === "Enter" && openMissionDetails(mission)}
                        tabindex="0"
                        role="button"
                        aria-label="View details for completed mission {mission.title}"
                    >
                        <div class="m-icon" style:background={mission.color}>
                            <i class="fas {mission.icon}"></i>
                        </div>
                        <div class="m-details">
                            <h4>{mission.title}</h4>
                            <p>{mission.subtitle}</p>
                            <div class="m-category">{mission.category}</div>
                        </div>
                        <div class="completion-badge">
                            <i class="fas fa-check-circle"></i>
                        </div>
                    </div>
                {/each}
            {:else}
                <div class="empty-state">
                    <i class="fas fa-tasks"></i>
                    <p>No completed missions yet. Start with something easy!</p>
                </div>
            {/if}
        </section>
    {/if}

    {#if selectedTab === "upcoming"}
        <section class="section mission-section" transition:fade>
            {#if upcomingMissions.length > 0}
                {#each upcomingMissions as mission}
                    <div class="mission-item upcoming">
                        <div class="m-icon" style:background={mission.color}>
                            <i class="fas {mission.icon}"></i>
                        </div>
                        <div class="m-details">
                            <h4>{mission.title}</h4>
                            <p>{mission.subtitle}</p>
                            <div class="m-category">{mission.category}</div>
                        </div>
                        <div class="unlock-time">
                            <i class="fas fa-clock"></i>
                            <span>{mission.timeToUnlock}</span>
                        </div>
                    </div>
                {/each}
            {:else}
                <div class="empty-state">
                    <i class="fas fa-calendar"></i>
                    <p>No upcoming missions. Check back later!</p>
                </div>
            {/if}
        </section>
    {/if}

    <section class="section card">
        <div class="header-row">
            <h3>Your Characters</h3>
            <button class="btn-link" on:click={toggleRewards}>View All</button>
        </div>
        <div class="characters-preview">
            {#each characters
                .filter((c) => c.unlocked)
                .slice(0, 3) as character}
                <div class="character">
                    <div class="character-icon">{character.icon}</div>
                    <div class="character-name">{character.name}</div>
                    <div class="character-level">Lvl {character.level}</div>
                </div>
            {/each}
            <div class="character locked">
                <div class="character-icon">❓</div>
                <div class="character-name">???</div>
                <div class="character-level">Locked</div>
            </div>
        </div>
    </section>
</div>

<nav class="nav-bar">
    {#each navItems as nav}
        <div
            class="nav-item {nav.active ? 'active' : ''} {nav.isAdd
                ? 'add'
                : ''}"
            on:click={() => handleNavClick(nav.label || "add")}
            on:keydown={(e) =>
                e.key === "Enter" && handleNavClick(nav.label || "add")}
            tabindex="0"
            role={nav.isAdd ? "button" : "link"}
            aria-label={nav.isAdd
                ? "Add new activity"
                : `Navigate to ${nav.label}`}
        >
            <i class="fas {nav.icon}"></i>
            {#if nav.label}<span>{nav.label}</span>{/if}
        </div>
    {/each}
</nav>

{#if toast.show}
    <div
        class="toast {toast.type}"
        transition:scale={{ duration: 200, start: 0.8 }}
    >
        {toast.message}
    </div>
{/if}

{#if showMissionDetails}
    <div
        class="modal-overlay"
        transition:fade={{ duration: 200 }}
        on:click={() => (showMissionDetails = false)}
    >
        <div class="modal-content" on:click|stopPropagation>
            {#if currentMission}
                <div
                    class="modal-header"
                    style:background={currentMission.color}
                >
                    <i class="fas {currentMission.icon} modal-icon"></i>
                    <h2>{currentMission.title}</h2>
                    <button
                        class="close-button"
                        on:click={() => (showMissionDetails = false)}
                    >
                        <i class="fas fa-times"></i>
                    </button>
                </div>
                <div class="modal-body">
                    <p class="mission-description">
                        {currentMission.description}
                    </p>

                    {#if currentMission.id === "water"}
                        <div class="water-tracker">
                            <h4>Water Tracker</h4>
                            <div class="water-controls">
                                <button
                                    class="water-btn"
                                    on:click={() => updateWater(-1)}
                                >
                                    <i class="fas fa-minus"></i>
                                </button>
                                <div class="water-display">
                                    {#each Array(totalWaterGoal) as _, i}
                                        <div
                                            class="water-glass {i < waterGlasses
                                                ? 'filled'
                                                : ''}"
                                        >
                                            <i class="fas fa-tint"></i>
                                        </div>
                                    {/each}
                                </div>
                                <button
                                    class="water-btn"
                                    on:click={() => updateWater(1)}
                                >
                                    <i class="fas fa-plus"></i>
                                </button>
                            </div>
                        </div>
                    {/if}

                    {#if currentMission.id === "affirmation"}
                        <div class="affirmation-section">
                            <h4>Today's Affirmation</h4>
                            <p class="affirmation-text">
                                "{affirmationText}"
                            </p>
                            <button
                                class="record-button {isSpeaking
                                    ? 'recording'
                                    : ''}"
                                on:click={startVoiceDetection}
                                disabled={isSpeaking}
                            >
                                <i
                                    class="fas {isSpeaking
                                        ? 'fa-microphone-alt'
                                        : 'fa-microphone'}"
                                ></i>
                                {isSpeaking
                                    ? "Recording..."
                                    : "Speak Affirmation"}
                            </button>
                        </div>
                    {/if}

                    {#if currentMission.id === "meal"}
                        <div class="photo-section">
                            <h4>Take a photo of your meal</h4>
                            <div class="photo-placeholder">
                                <i class="fas fa-camera"></i>
                                <p>Tap to take photo</p>
                            </div>
                        </div>
                    {/if}

                    <div class="reward-info">
                        <h4>Rewards</h4>
                        <div class="reward-item">
                            <span class="reward-icon">⭐</span>
                            <span>{currentMission.reward} points</span>
                        </div>
                        {#if progressData.streak >= 3}
                            <div class="reward-item streak-bonus">
                                <span class="reward-icon">🔥</span>
                                <span>+10 streak bonus</span>
                            </div>
                        {/if}
                    </div>

                    {#if !currentMission.completed}
                        <button
                            class="complete-button"
                            on:click={() => completeMission(currentMission.id)}
                        >
                            Mark as Completed
                        </button>
                    {:else}
                        <div class="mission-completed-badge">
                            <i class="fas fa-check-circle"></i>
                            Completed
                        </div>
                    {/if}
                </div>
            {/if}
        </div>
    </div>
{/if}

{#if showRewards}
    <div
        class="modal-overlay"
        transition:fade={{ duration: 200 }}
        on:click={() => (showRewards = false)}
    >
        <div class="modal-content rewards-modal" on:click|stopPropagation>
            <div class="modal-header">
                <h2>Character Collection</h2>
                <button
                    class="close-button"
                    on:click={() => (showRewards = false)}
                >
                    <i class="fas fa-times"></i>
                </button>
            </div>
            <div class="modal-body">
                <div class="characters-grid">
                    {#each characters as character}
                        <div
                            class="character-card {character.unlocked
                                ? ''
                                : 'locked'}"
                        >
                            <div class="character-icon">
                                {character.unlocked ? character.icon : "❓"}
                            </div>
                            <div class="character-name">
                                {character.unlocked ? character.name : "Locked"}
                            </div>
                            {#if character.unlocked}
                                <div class="character-level">
                                    Level {character.level}
                                </div>
                            {:else}
                                <div class="unlock-hint">
                                    Keep completing missions to unlock
                                </div>
                            {/if}
                        </div>
                    {/each}
                </div>
            </div>
        </div>
    </div>
{/if}

<style>
    :global(*) {
        margin: 0;
        padding: 0;
        box-sizing: border-box;
        font-family: "Inter", sans-serif;
    }
    :root {
        --bg-gradient-start: hsl(190, 85%, 93%);
        --bg-gradient-end: hsl(250, 85%, 93%);
        --bg-light: #f7f8fc;
        --card: #ffffff;
        --primary-start: #34d399;
        --primary-end: #6366f1;
        --accent: #8b5cf6;
        --success: #10b981;
        --warning: #f59e0b;
        --error: #ef4444;
        --info: #3b82f6;
        --text-dark: #333;
        --text-muted: #666;
        --border: #eee;
        --shadow-sm: rgba(0, 0, 0, 0.05);
        --shadow-md: rgba(0, 0, 0, 0.1);
    }

    .phone-container {
        width: 375px;
        height: 812px;
        background: linear-gradient(
            135deg,
            var(--bg-gradient-start),
            var(--bg-gradient-end)
        );
        margin: 2rem auto;
        border-radius: 40px;
        box-shadow: 0 10px 25px var(--shadow-md);
        overflow: hidden;
        position: relative;
    }

    .status-bar {
        height: 44px;
        display: flex;
        align-items: center;
        justify-content: space-between;
        padding: 0 16px;
        background: rgba(255, 255, 255, 0.7);
        color: var(--text-dark);
        backdrop-filter: blur(8px);
    }
    .status-bar .time {
        font-weight: 600;
    }
    .status-icons i {
        margin-left: 8px;
    }

    .app-container {
        padding: 16px 16px 0;
        overflow-y: auto;
        height: calc(100% - 44px - 70px);
        scrollbar-width: none;
    }
    .app-container::-webkit-scrollbar {
        display: none;
    }

    .header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding: 8px 0;
        margin-bottom: 16px;
    }
    .header-left {
        display: flex;
        align-items: center;
    }
    .logo {
        width: 28px;
        height: 28px;
        background: linear-gradient(
            135deg,
            var(--primary-start),
            var(--primary-end)
        );
        border-radius: 6px;
        margin-right: 8px;
        box-shadow: 0 2px 6px var(--shadow-sm);
        display: flex;
        justify-content: center;
        align-items: center;
        color: white;
        font-size: 1.2rem;
    }
    .title {
        font-size: 1.2rem;
        font-weight: 700;
        background: linear-gradient(
            135deg,
            var(--primary-start),
            var(--primary-end)
        );
        -webkit-background-clip: text;
        color: transparent;
    }

    .header-right {
        display: flex;
        align-items: center;
        gap: 12px;
    }
    .btn.sign-in {
        padding: 6px 12px;
        font-size: 0.9rem;
        background: linear-gradient(
            135deg,
            var(--primary-start),
            var(--primary-end)
        );
        border: none;
        border-radius: 6px;
        color: #fff;
        box-shadow: 0 2px 6px var(--shadow-sm);
        transition: transform 0.2s;
    }
    .btn.sign-in:hover {
        transform: translateY(-2px);
    }

    .user-stats {
        display: flex;
        gap: 6px;
    }
    .user-stats .stat {
        display: flex;
        align-items: center;
        gap: 4px;
        background: var(--card);
        padding: 3px 6px;
        border-radius: 12px;
        font-size: 0.75rem;
        box-shadow: 0 1px 4px var(--shadow-sm);
        color: var(--text-dark);
    }
    .user-stats .user-level {
        background: linear-gradient(
            135deg,
            var(--primary-start),
            var(--primary-end)
        );
        color: white;
        font-weight: 600;
    }

    .card {
        background: var(--card);
        border-radius: 16px;
        padding: 16px;
        box-shadow: 0 4px 12px var(--shadow-sm);
        margin-bottom: 16px;
    }

    .header-row {
        display: flex;
        justify-content: space-between;
        align-items: center;
        margin-bottom: 12px;
    }
    .header-row h3 {
        font-size: 1rem;
        color: var(--text-dark);
    }
    .counter {
        font-size: 0.9rem;
        font-weight: 600;
        color: var(--accent);
    }

    /* Level card */
    .level-card {
        padding: 12px 16px;
        background: linear-gradient(
            120deg,
            rgba(99, 102, 241, 0.1),
            rgba(52, 211, 153, 0.1)
        );
        margin-bottom: 16px;
    }
    .level-header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        margin-bottom: 8px;
    }
    .level-header h3 {
        font-size: 1rem;
        font-weight: 600;
        color: var(--accent);
    }
    .level-header span {
        font-size: 0.9rem;
        color: var(--text-muted);
    }

    .mood-card {
        padding: 16px;
        margin-bottom: 16px;
    }
    .mood-card h3 {
        font-size: 1rem;
        margin-bottom: 12px;
        text-align: center;
        color: var(--text-dark);
    }
    .mood-selection {
        display: flex;
        gap: 8px;
        flex-wrap: wrap;
        justify-content: center;
    }
    .mood-option {
        display: flex;
        flex-direction: column;
        align-items: center;
        padding: 8px 4px;
        border-radius: 12px;
        border: 2px solid transparent;
        background: transparent;
        cursor: pointer;
        transition: all 0.2s;
        width: calc(33.33% - 8px);
    }
    .mood-option:hover {
        transform: translateY(-2px);
        background: rgba(255, 255, 255, 0.5);
    }
    .mood-option.selected {
        border-color: rgba(255, 255, 255, 0.8);
        color: white;
        transform: translateY(-2px);
    }
    .mood-emoji {
        font-size: 1.8rem;
        margin-bottom: 4px;
    }
    .mood-label {
        font-size: 0.8rem;
        font-weight: 500;
    }

    .progress-bar {
        position: relative;
        height: 10px;
        background: var(--bg-light);
        border-radius: 5px;
        overflow: hidden;
        margin-bottom: 8px;
    }
    .progress-bar .fill {
        height: 100%;
        background: var(--accent);
        border-radius: 5px 0 0 5px;
        transition: width 0.5s ease-out;
    }
    .progress-bar .glow {
        position: absolute;
        top: 0;
        left: 0;
        height: 100%;
        filter: blur(4px);
        background: var(--primary-end);
        transition: width 0.5s ease-out;
    }

    .markers {
        display: flex;
        justify-content: space-between;
        margin-bottom: 12px;
    }
    .marker {
        width: 8px;
        height: 8px;
        background: #fff;
        border: 2px solid var(--bg-light);
        border-radius: 50%;
    }
    .marker.done {
        background: var(--accent);
        border-color: var(--accent);
    }

    .stats-row {
        display: flex;
        gap: 16px;
        color: var(--text-muted);
        font-size: 0.9rem;
    }
    .stats-row .stat-row i {
        margin-right: 4px;
    }

    .tabs {
        display: flex;
        margin-bottom: 16px;
        background: rgba(255, 255, 255, 0.5);
        border-radius: 12px;
        padding: 4px;
    }
    .tab {
        flex: 1;
        padding: 8px 4px;
        font-size: 0.85rem;
        border: none;
        background: transparent;
        border-radius: 8px;
        color: var(--text-muted);
        font-weight: 500;
        transition: all 0.2s;
        cursor: pointer;
    }
    .tab.active {
        background: white;
        color: var(--accent);
        box-shadow: 0 2px 8px rgba(0, 0, 0, 0.05);
    }

    .mission-section {
        margin-bottom: 16px;
    }
    .missions-grid {
        display: grid;
        grid-template-columns: 1fr;
        gap: 12px;
        margin-bottom: 12px;
    }
    .mission-item {
        display: flex;
        align-items: center;
        gap: 12px;
        padding: 12px;
        border-radius: 12px;
        background: var(--card);
        box-shadow: 0 2px 8px var(--shadow-sm);
        transition:
            transform 0.2s,
            box-shadow 0.2s;
        cursor: pointer;
        position: relative;
    }
    .mission-item:not(.missions-grid .mission-item) {
        margin-bottom: 12px;
    }
    .mission-item:hover,
    .mission-item:focus-visible {
        transform: translateY(-2px);
        box-shadow: 0 4px 16px var(--shadow-sm);
        outline: 2px solid var(--accent);
    }
    .mission-item.completed {
        border: 1px solid #0cff00;
    }
    .mission-item.upcoming {
        opacity: 0.8;
        background: rgba(255, 255, 255, 0.8);
        border: 1px dashed rgba(0, 0, 0, 0.1);
    }

    .m-icon {
        min-width: 40px;
        min-height: 40px;
        width: 40px;
        height: 40px;
        border-radius: 10px;
        display: flex;
        justify-content: center;
        align-items: center;
        color: #fff;
        font-size: 1.2rem;
        box-shadow: 0 2px 6px var(--shadow-sm);
    }
    .m-details {
        flex: 1;
        overflow: hidden;
    }
    .m-details h4 {
        font-size: 0.95rem;
        color: var(--text-dark);
        margin-bottom: 2px;
        white-space: nowrap;
        overflow: hidden;
        text-overflow: ellipsis;
    }
    .m-details p {
        font-size: 0.85rem;
        color: var(--text-muted);
        margin-bottom: 4px;
    }
    .m-category {
        display: inline-block;
        font-size: 0.7rem;
        background: rgba(0, 0, 0, 0.05);
        padding: 2px 6px;
        border-radius: 10px;
        text-transform: capitalize;
    }

    .completion-badge {
        position: absolute;
        top: 8px;
        right: 8px;
        color: #0cff00;
        font-size: 1rem;
    }

    .unlock-time {
        position: absolute;
        top: 8px;
        right: 8px;
        font-size: 0.8rem;
        color: var(--text-muted);
        display: flex;
        align-items: center;
        gap: 4px;
    }

    .empty-state {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        background: rgba(255, 255, 255, 0.7);
        border-radius: 12px;
        padding: 24px;
        color: var(--text-muted);
    }
    .empty-state i {
        font-size: 2rem;
        margin-bottom: 12px;
        opacity: 0.5;
    }
    .empty-state p {
        text-align: center;
        font-size: 0.9rem;
    }

    .characters-preview {
        display: flex;
        gap: 12px;
        overflow-x: auto;
        padding: 4px 0;
        scrollbar-width: none;
    }
    .characters-preview::-webkit-scrollbar {
        display: none;
    }
    .character {
        display: flex;
        flex-direction: column;
        align-items: center;
        min-width: 60px;
        background: rgba(255, 255, 255, 0.7);
        padding: 8px;
        border-radius: 12px;
        transition: transform 0.2s;
    }
    .character:hover {
        transform: translateY(-2px);
    }
    .character.locked {
        opacity: 0.6;
        background: rgba(0, 0, 0, 0.05);
    }
    .character-icon {
        font-size: 2rem;
        margin-bottom: 4px;
    }
    .character-name {
        font-size: 0.7rem;
        font-weight: 600;
        text-align: center;
        margin-bottom: 2px;
    }
    .character-level {
        font-size: 0.65rem;
        color: var(--accent);
        font-weight: 500;
    }

    .nav-bar {
        position: fixed; /* Makes the element stick to the viewport or nearest positioned ancestor */
        bottom: 0; /* Positions it at the bottom edge */
        left: 0; /* Aligns it to the left edge */
        right: 0; /* Aligns it to the right edge (makes it span the width) */
        height: 70px;
        background: rgba(255, 255, 255, 0.9);
        display: flex;
        justify-content: space-around;
        align-items: center;
        border-top: 1px solid var(--border);
        backdrop-filter: blur(8px);
        z-index: 100; /* Ensures it stays above other content */
    }
    .nav-item {
        display: flex;
        flex-direction: column;
        align-items: center;
        font-size: 0.7rem;
        color: var(--text-muted);
        transition: color 0.2s;
        cursor: pointer;
    }
    .nav-item.active,
    .nav-item:hover,
    .nav-item:focus-visible {
        color: var(--accent);
        outline: none;
    }
    .nav-item:focus-visible {
        text-decoration: underline;
    }
    .nav-item.add {
        width: 56px;
        height: 56px;
        background: linear-gradient(
            135deg,
            var(--primary-start),
            var(--primary-end)
        );
        border-radius: 50%;
        transform: translateY(-20%);
        display: flex;
        justify-content: center;
        align-items: center;
        color: #fff;
        box-shadow: 0 4px 16px var(--shadow-sm);
    }
    .nav-item.add:focus-visible {
        outline: 3px solid var(--accent);
        text-decoration: none;
    }

    .toast {
        position: absolute;
        bottom: 90px;
        left: 50%;
        transform: translateX(-50%);
        background: rgba(0, 0, 0, 0.7);
        color: #fff;
        padding: 10px 16px;
        border-radius: 20px;
        font-size: 0.85rem;
        z-index: 100;
        max-width: 300px;
    }
    .toast.success {
        background: rgba(16, 185, 129, 0.9);
    }
    .toast.error {
        background: rgba(239, 68, 68, 0.9);
    }
    .toast.warning {
        background: rgba(245, 158, 11, 0.9);
    }
    .toast.info {
        background: rgba(59, 130, 246, 0.9);
    }

    .modal-overlay {
        position: fixed;
        top: 0;
        left: 0;
        right: 0;
        bottom: 0;
        background: rgba(0, 0, 0, 0.5);
        display: flex;
        justify-content: center;
        align-items: center;
        z-index: 1000;
    }
    .modal-content {
        background: white;
        width: 90%;
        max-width: 340px;
        border-radius: 16px;
        overflow: hidden;
        max-height: 80vh;
        display: flex;
        flex-direction: column;
    }
    .modal-header {
        padding: 20px 16px;
        position: relative;
        display: flex;
        flex-direction: column;
        align-items: center;
        color: white;
    }
    .modal-icon {
        font-size: 2rem;
        margin-bottom: 10px;
    }
    .modal-header h2 {
        font-size: 1.4rem;
        font-weight: 600;
    }
    .close-button {
        position: absolute;
        top: 10px;
        right: 10px;
        background: rgba(255, 255, 255, 0.3);
        border: none;
        width: 30px;
        height: 30px;
        border-radius: 50%;
        display: flex;
        justify-content: center;
        align-items: center;
        color: white;
        font-size: 1rem;
        cursor: pointer;
    }
    .modal-body {
        padding: 16px;
        overflow-y: auto;
    }
    .mission-description {
        margin-bottom: 20px;
        line-height: 1.5;
        color: var(--text-dark);
    }
    .reward-info {
        margin: 20px 0;
        background: rgba(0, 0, 0, 0.03);
        padding: 12px;
        border-radius: 12px;
    }
    .reward-info h4 {
        margin-bottom: 8px;
        font-size: 1rem;
    }
    .reward-item {
        display: flex;
        align-items: center;
        gap: 8px;
        margin-bottom: 6px;
    }
    .reward-item.streak-bonus {
        color: #f97316;
    }
    .complete-button {
        width: 100%;
        padding: 12px;
        background: linear-gradient(
            135deg,
            var(--primary-start),
            var(--primary-end)
        );
        color: white;
        border: none;
        border-radius: 12px;
        font-weight: 600;
        font-size: 1rem;
        cursor: pointer;
        margin-top: 16px;
        transition: transform 0.2s;
    }
    .complete-button:hover {
        transform: translateY(-2px);
    }
    .mission-completed-badge {
        display: flex;
        align-items: center;
        justify-content: center;
        gap: 8px;
        background: #0cff00;
        color: white;
        padding: 12px;
        border-radius: 12px;
        font-weight: 600;
        margin-top: 16px;
    }

    .water-tracker {
        background: rgba(52, 211, 153, 0.1);
        padding: 16px;
        border-radius: 12px;
        margin-bottom: 16px;
    }
    .water-tracker h4 {
        text-align: center;
        margin-bottom: 12px;
        color: var(--text-dark);
    }
    .water-controls {
        display: flex;
        align-items: center;
        justify-content: space-between;
    }
    .water-btn {
        width: 36px;
        height: 36px;
        border-radius: 50%;
        background: white;
        border: 1px solid rgba(0, 0, 0, 0.1);
        display: flex;
        justify-content: center;
        align-items: center;
        cursor: pointer;
    }
    .water-display {
        display: flex;
        gap: 6px;
    }
    .water-glass {
        width: 20px;
        height: 28px;
        border-radius: 4px;
        background: rgba(255, 255, 255, 0.5);
        display: flex;
        justify-content: center;
        align-items: center;
        color: rgba(0, 0, 0, 0.2);
    }
    .water-glass.filled {
        background: #34d399;
        color: white;
    }

    .affirmation-section {
        background: rgba(255, 121, 198, 0.1);
        padding: 16px;
        border-radius: 12px;
        margin-bottom: 16px;
    }
    .affirmation-section h4 {
        text-align: center;
        margin-bottom: 12px;
    }
    .affirmation-text {
        text-align: center;
        font-style: italic;
        margin-bottom: 16px;
        padding: 0 10px;
        line-height: 1.5;
    }
    .record-button {
        width: 100%;
        padding: 10px;
        display: flex;
        align-items: center;
        justify-content: center;
        gap: 8px;
        background: white;
        border: 1px solid rgba(0, 0, 0, 0.1);
        border-radius: 10px;
        cursor: pointer;
        transition: all 0.2s;
    }
    .record-button:hover {
        background: #ff79c6;
        color: white;
    }
    .record-button.recording {
        background: #ff79c6;
        color: white;
        animation: pulse 1.5s infinite;
    }
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

    .photo-section {
        background: rgba(249, 115, 22, 0.1);
        padding: 16px;
        border-radius: 12px;
        margin-bottom: 16px;
    }
    .photo-section h4 {
        text-align: center;
        margin-bottom: 12px;
    }
    .photo-placeholder {
        height: 150px;
        background: rgba(255, 255, 255, 0.5);
        border: 2px dashed rgba(0, 0, 0, 0.1);
        border-radius: 10px;
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        cursor: pointer;
    }
    .photo-placeholder i {
        font-size: 2rem;
        margin-bottom: 10px;
        color: rgba(0, 0, 0, 0.3);
    }
    .photo-placeholder p {
        font-size: 0.9rem;
        color: rgba(0, 0, 0, 0.5);
    }

    .rewards-modal {
        max-width: 350px;
    }
    .characters-grid {
        display: grid;
        grid-template-columns: repeat(3, 1fr);
        gap: 12px;
    }
    .character-card {
        display: flex;
        flex-direction: column;
        align-items: center;
        background: rgba(255, 255, 255, 0.5);
        padding: 12px 8px;
        border-radius: 12px;
        transition: transform 0.2s;
    }
    .character-card:hover {
        transform: translateY(-4px);
    }
    .character-card.locked {
        opacity: 0.7;
        background: rgba(0, 0, 0, 0.05);
    }
    .character-card .character-icon {
        font-size: 2.5rem;
        margin-bottom: 8px;
    }
    .character-card .character-name {
        font-size: 0.85rem;
        text-align: center;
        margin-bottom: 4px;
    }
    .character-card .character-level {
        font-size: 0.75rem;
        color: var(--accent);
    }
    .unlock-hint {
        font-size: 0.65rem;
        text-align: center;
        color: var(--text-muted);
    }

    .btn-link {
        background: none;
        border: none;
        color: var(--accent);
        font-size: 0.9rem;
        font-weight: 500;
        cursor: pointer;
        padding: 0;
    }
</style>
