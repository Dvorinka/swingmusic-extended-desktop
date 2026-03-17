<template>
  <div class="desktop-app">
    <!-- Native Title Bar -->
    <div class="title-bar" v-if="!isMacOS">
      <div class="title-bar-left">
        <img src="/icons/logo.png" class="app-icon" />
        <span class="app-title">SwingMusic</span>
      </div>
      <div class="title-bar-right">
        <button @click="minimizeWindow" class="title-bar-button">
          <Minus />
        </button>
        <button @click="maximizeWindow" class="title-bar-button">
          <Square />
        </button>
        <button @click="closeWindow" class="title-bar-button close">
          <X />
        </button>
      </div>
    </div>
    
    <!-- Main Application Layout -->
    <div class="app-layout">
      <!-- Sidebar Navigation -->
      <div class="sidebar" :class="{ collapsed: sidebarCollapsed }">
        <div class="sidebar-header">
          <button @click="toggleSidebar" class="sidebar-toggle">
            <Menu />
          </button>
          <h2 v-if="!sidebarCollapsed" class="sidebar-title">SwingMusic</h2>
        </div>
        
        <nav class="sidebar-nav">
          <button 
            v-for="item in navigationItems" 
            :key="item.id"
            @click="navigateTo(item.route)"
            class="nav-item"
            :class="{ active: currentRoute === item.route }"
          >
            <component :is="item.icon" />
            <span v-if="!sidebarCollapsed">{{ item.name }}</span>
          </button>
        </nav>
        
        <!-- Quick Actions -->
        <div class="quick-actions" v-if="!sidebarCollapsed">
          <div class="quick-action-card">
            <button @click="openWebAppConnection" class="quick-action primary" :class="{ connected: isWebAppConnected }">
              <Cloud /> {{ isWebAppConnected ? 'Connected' : 'Connect Web App' }}
            </button>
            <div class="connection-status" v-if="isWebAppConnected">
              <div class="status-indicator connected"></div>
              <span>Syncing with {{ webAppUrl }}</span>
            </div>
          </div>
          
          <div class="quick-action-card">
            <button @click="openUniversalDownloader" class="quick-action secondary">
              <Download /> Quick Download
            </button>
          </div>
          
          <div class="quick-action-card">
            <button @click="openSettings" class="quick-action secondary">
              <Settings /> Settings
            </button>
          </div>
        </div>
      </div>
      
      <!-- Main Content Area -->
      <div class="main-content">
        <!-- Header -->
        <header class="app-header">
          <div class="header-left">
            <h1 class="page-title">{{ currentPageTitle }}</h1>
            <div class="breadcrumb">
              <span class="breadcrumb-item">Home</span>
              <span class="breadcrumb-separator">/</span>
              <span class="breadcrumb-item active">{{ currentPageTitle }}</span>
            </div>
          </div>
          <div class="header-right">
            <div class="search-bar">
              <Search />
              <input 
                v-model="searchQuery" 
                @input="handleSearch"
                placeholder="Search tracks, artists, albums..."
                type="text"
                class="search-input"
              />
            </div>
            <div class="user-controls">
              <button @click="toggleTheme" class="theme-toggle">
                <Sun v-if="isDarkMode" />
                <Moon v-else />
              </button>
              <button @click="showNotifications" class="notification-btn">
                <Bell />
                <span v-if="unreadNotifications > 0" class="notification-badge">
                  {{ unreadNotifications }}
                </span>
              </button>
            </div>
          </div>
        </header>
        
        <!-- Page Content -->
        <main class="page-content">
          <div class="content-wrapper">
            <component :is="currentPageComponent" />
          </div>
        </main>
        
        <!-- Mini Player -->
        <div class="mini-player" v-if="currentTrack">
          <div class="track-info">
            <div class="track-cover-wrapper">
              <img :src="currentTrack.cover" class="track-cover" />
              <div class="playing-indicator" v-if="isPlaying">
                <div class="bar"></div>
                <div class="bar"></div>
                <div class="bar"></div>
              </div>
            </div>
            <div class="track-details">
              <h4 class="track-title">{{ currentTrack.title }}</h4>
              <p class="track-artist">{{ currentTrack.artist }}</p>
            </div>
          </div>
          <div class="player-controls">
            <button @click="previousTrack" class="control-btn">
              <SkipBack />
            </button>
            <button @click="togglePlayPause" class="control-btn play-pause">
              <Pause v-if="isPlaying" />
              <Play v-else />
            </button>
            <button @click="nextTrack" class="control-btn">
              <SkipForward />
            </button>
          </div>
          <div class="progress-section">
            <div class="progress-bar">
              <div class="progress-fill" :style="{ width: playbackProgress + '%' }"></div>
            </div>
            <div class="time-info">
              <span class="current-time">2:34</span>
              <span class="total-time">4:21</span>
            </div>
          </div>
          <div class="volume-control">
            <Volume2 />
            <input 
              v-model="volume" 
              @input="updateVolume"
              type="range" 
              min="0" 
              max="100" 
              class="volume-slider"
            />
            <span class="volume-value">{{ volume }}%</span>
          </div>
        </div>
      </div>
    </div>
    
    <!-- Web App Connection Modal -->
    <WebAppConnectionModal
      :isVisible="showConnectionModal"
      :webAppUrl="webAppUrl"
      :isConnected="isWebAppConnected"
      @close="closeConnectionModal"
      @connection-changed="handleConnectionChange"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'
import { listen } from '@tauri-apps/api/event'
import { appWindow } from '@tauri-apps/api/window'

// Reactive state
const sidebarCollapsed = ref(false)
const currentRoute = ref('home')
const searchQuery = ref('')
const isDarkMode = ref(true)
const currentTrack = ref(null)
const isPlaying = ref(false)
const playbackProgress = ref(0)
const volume = ref(75)
const unreadNotifications = ref(0)
const isWebAppConnected = ref(false)
const webAppUrl = ref('http://localhost:6081')
const pairingCode = ref('')
const showConnectionModal = ref(false)

// Computed properties
const currentPageTitle = computed(() => {
  const titles = {
    home: 'Home',
    library: 'Your Library',
    downloads: 'Downloads',
    search: 'Search'
  }
  return titles[currentRoute.value] || 'SwingMusic'
})

const currentPageComponent = computed(() => {
  const components = {
    home: 'HomeView',
    library: 'LibraryView',
    downloads: 'DownloadsView',
    search: 'SearchView'
  }
  return components[currentRoute.value] || 'HomeView'
})

const navigationItems = ref([
  { id: 'home', name: 'Home', icon: 'Home', route: 'home' },
  { id: 'search', name: 'Search', icon: 'Search', route: 'search' },
  { id: 'library', name: 'Your Library', icon: 'Music', route: 'library' },
  { id: 'downloads', name: 'Downloads', icon: 'Download', route: 'downloads' }
])

// Window controls
const minimizeWindow = () => appWindow.minimize()
const maximizeWindow = () => appWindow.maximize()
const closeWindow = () => appWindow.close()

// Navigation
const navigateTo = (route: string) => {
  currentRoute.value = route
}

const toggleSidebar = () => {
  sidebarCollapsed.value = !sidebarCollapsed.value
}

// Theme management
const toggleTheme = () => {
  isDarkMode.value = !isDarkMode.value
  document.documentElement.classList.toggle('dark', isDarkMode.value)
}

// Player controls
const togglePlayPause = () => {
  isPlaying.value = !isPlaying.value
  // Send play/pause command to backend
}

const previousTrack = () => {
  // Send previous track command
}

const nextTrack = () => {
  // Send next track command
}

const updateVolume = () => {
  // Send volume update command
}

// Lifecycle hooks
onMounted(async () => {
  // Initialize app
  await initializeApp()
  
  // Set up event listeners
  await setupEventListeners()
  
  // Load initial data
  await loadInitialData()
})

onUnmounted(() => {
  // Cleanup
})

const initializeApp = async () => {
  try {
    // Load app settings
    const settings = await invoke('get_app_settings')
    isDarkMode.value = settings.theme === 'dark'
    volume.value = settings.volume || 75
    
    // Apply theme
    document.documentElement.classList.toggle('dark', isDarkMode.value)
  } catch (error) {
    console.error('Failed to initialize app:', error)
  }
}

const setupEventListeners = async () => {
  // Listen for download progress updates
  await listen('download-progress', (event) => {
    console.log('Download progress:', event.payload)
    // Update download progress in UI
  })
  
  // Listen for system tray events
  await listen('system-tray-event', (event) => {
    handleSystemTrayEvent(event.payload)
  })
  
  // Listen for media control events
  await listen('play_pause', () => {
    togglePlayPause()
  })
  
  await listen('next_track', () => {
    nextTrack()
  })
  
  await listen('prev_track', () => {
    previousTrack()
  })
}

const loadInitialData = async () => {
  try {
    // Load library data
    // Load download history
    // Load user preferences
  } catch (error) {
    console.error('Failed to load initial data:', error)
  }
}

const handleSearch = () => {
  // Implement search functionality
}

const showNotifications = () => {
  // Show notifications panel
}

const openUniversalDownloader = () => {
  navigateTo('downloads')
}

const openWebAppConnection = () => {
  // Open web app connection modal/dialog
  if (isWebAppConnected.value) {
    disconnectFromWebApp()
  } else {
    connectToWebApp()
  }
}

const connectToWebApp = async () => {
  try {
    // Generate pairing code
    pairingCode.value = generatePairingCode()
    
    // Show connection dialog with pairing code
    showConnectionDialog()
    
    // Attempt connection to web app
    const connected = await invoke('connect_to_web_app', { 
      url: webAppUrl.value,
      pairingCode: pairingCode.value 
    })
    
    if (connected) {
      isWebAppConnected.value = true
      await invoke('show_notification', {
        title: 'Connected to Web App',
        body: `Successfully connected to ${webAppUrl.value}`
      })
    }
  } catch (error) {
    console.error('Failed to connect to web app:', error)
    await invoke('show_notification', {
      title: 'Connection Failed',
      body: 'Could not connect to web app. Please check the URL and try again.'
    })
  }
}

const disconnectFromWebApp = async () => {
  try {
    await invoke('disconnect_from_web_app')
    isWebAppConnected.value = false
    pairingCode.value = ''
    
    await invoke('show_notification', {
      title: 'Disconnected',
      body: 'Disconnected from web app'
    })
  } catch (error) {
    console.error('Failed to disconnect from web app:', error)
  }
}

const generatePairingCode = () => {
  return Math.random().toString(36).substring(2, 8).toUpperCase()
}

const showConnectionDialog = () => {
  showConnectionModal.value = true
}

const closeConnectionModal = () => {
  showConnectionModal.value = false
}

const handleConnectionChange = (connected: boolean) => {
  isWebAppConnected.value = connected
}

const openSettings = () => {
  navigateTo('settings')
}

const handleSystemTrayEvent = (event: any) => {
  // Handle system tray events
}
</script>

<style scoped>
.desktop-app {
  height: 100vh;
  display: flex;
  flex-direction: column;
  background: var(--background-primary);
  color: var(--text-primary);
}

.title-bar {
  height: 32px;
  background: var(--background-secondary);
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0 8px;
  -webkit-app-region: drag;
}

.title-bar-left {
  display: flex;
  align-items: center;
  gap: 8px;
}

.app-icon {
  width: 16px;
  height: 16px;
}

.app-title {
  font-size: 12px;
  font-weight: 600;
}

.title-bar-right {
  display: flex;
  gap: 4px;
  -webkit-app-region: no-drag;
}

.title-bar-button {
  width: 24px;
  height: 24px;
  border: none;
  background: transparent;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 4px;
  cursor: pointer;
  transition: background-color 0.2s;
}

.title-bar-button:hover {
  background: var(--background-tertiary);
}

.title-bar-button.close:hover {
  background: #ff5f57;
  color: white;
}

.app-layout {
  flex: 1;
  display: flex;
  overflow: hidden;
}

.sidebar {
  width: 240px;
  background: var(--background-secondary);
  border-right: 1px solid var(--border-primary);
  display: flex;
  flex-direction: column;
  transition: width 0.3s ease;
}

.sidebar.collapsed {
  width: 48px;
}

.sidebar-header {
  height: 48px;
  display: flex;
  align-items: center;
  padding: 0 12px;
  border-bottom: 1px solid var(--border-primary);
}

.sidebar-toggle {
  border: none;
  background: transparent;
  cursor: pointer;
  padding: 8px;
  border-radius: 6px;
  transition: background-color 0.2s;
}

.sidebar-toggle:hover {
  background: var(--background-tertiary);
}

.sidebar-title {
  font-size: 16px;
  font-weight: 600;
  margin-left: 8px;
}

.sidebar-nav {
  flex: 1;
  padding: 12px 0;
}

.nav-item {
  width: 100%;
  padding: 10px 12px;
  border: none;
  background: transparent;
  display: flex;
  align-items: center;
  gap: 12px;
  cursor: pointer;
  transition: background-color 0.2s;
  color: var(--text-secondary);
}

.nav-item:hover {
  background: var(--background-tertiary);
  color: var(--text-primary);
}

.nav-item.active {
  background: var(--accent-primary);
  color: var(--accent-on-primary);
}

.quick-actions {
  padding: 12px;
  border-top: 1px solid var(--border-primary);
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.quick-action {
  padding: 8px 12px;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  font-size: 14px;
  font-weight: 500;
  transition: all 0.2s;
  display: flex;
  align-items: center;
  gap: 8px;
}

.quick-action.primary {
  background: var(--accent-primary);
  color: var(--accent-on-primary);
}

.quick-action.primary:hover {
  background: var(--accent-primary-hover);
}

.quick-action.secondary {
  background: var(--background-tertiary);
  color: var(--text-primary);
}

.quick-action.secondary:hover {
  background: var(--background-quaternary);
}

.main-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.app-header {
  height: 56px;
  background: var(--background-secondary);
  border-bottom: 1px solid var(--border-primary);
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 16px;
}

.header-left {
  display: flex;
  align-items: center;
}

.page-title {
  font-size: 20px;
  font-weight: 600;
}

.header-right {
  display: flex;
  align-items: center;
  gap: 16px;
}

.search-bar {
  display: flex;
  align-items: center;
  background: var(--background-tertiary);
  border-radius: 8px;
  padding: 8px 12px;
  width: 300px;
}

.search-bar input {
  border: none;
  background: transparent;
  outline: none;
  flex: 1;
  margin-left: 8px;
  color: var(--text-primary);
}

.user-controls {
  display: flex;
  align-items: center;
  gap: 8px;
}

.theme-toggle,
.notification-btn {
  border: none;
  background: transparent;
  cursor: pointer;
  padding: 8px;
  border-radius: 6px;
  transition: background-color 0.2s;
  position: relative;
}

.theme-toggle:hover,
.notification-btn:hover {
  background: var(--background-tertiary);
}

.notification-badge {
  position: absolute;
  top: 4px;
  right: 4px;
  background: var(--accent-error);
  color: white;
  font-size: 10px;
  padding: 2px 4px;
  border-radius: 10px;
  min-width: 16px;
  text-align: center;
}

.page-content {
  flex: 1;
  overflow-y: auto;
  padding: 24px;
}

.mini-player {
  height: 64px;
  background: var(--background-secondary);
  border-top: 1px solid var(--border-primary);
  display: flex;
  align-items: center;
  padding: 0 16px;
  gap: 16px;
}

.track-info {
  display: flex;
  align-items: center;
  gap: 12px;
  min-width: 0;
  flex: 1;
}

.track-cover {
  width: 40px;
  height: 40px;
  border-radius: 4px;
  object-fit: cover;
}

.track-details {
  min-width: 0;
}

.track-title {
  font-size: 14px;
  font-weight: 500;
  margin: 0;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.track-artist {
  font-size: 12px;
  color: var(--text-secondary);
  margin: 0;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.player-controls {
  display: flex;
  align-items: center;
  gap: 8px;
}

.control-btn {
  border: none;
  background: transparent;
  cursor: pointer;
  padding: 8px;
  border-radius: 6px;
  transition: background-color 0.2s;
  display: flex;
  align-items: center;
  justify-content: center;
}

.control-btn:hover {
  background: var(--background-tertiary);
}

.control-btn.play-pause {
  background: var(--accent-primary);
  color: var(--accent-on-primary);
}

.control-btn.play-pause:hover {
  background: var(--accent-primary-hover);
}

.progress-bar {
  width: 120px;
  height: 4px;
  background: var(--background-tertiary);
  border-radius: 2px;
  overflow: hidden;
}

.progress-fill {
  height: 100%;
  background: var(--accent-primary);
  transition: width 0.1s linear;
}

.volume-control {
  display: flex;
  align-items: center;
  gap: 8px;
}

.volume-slider {
  width: 80px;
  -webkit-appearance: none;
  appearance: none;
  height: 4px;
  background: var(--background-tertiary);
  border-radius: 2px;
  outline: none;
}

.volume-slider::-webkit-slider-thumb {
  -webkit-appearance: none;
  appearance: none;
  width: 12px;
  height: 12px;
  background: var(--accent-primary);
  border-radius: 50%;
  cursor: pointer;
}

.volume-slider::-moz-range-thumb {
  width: 12px;
  height: 12px;
  background: var(--accent-primary);
  border-radius: 50%;
  cursor: pointer;
  border: none;
}
</style>
