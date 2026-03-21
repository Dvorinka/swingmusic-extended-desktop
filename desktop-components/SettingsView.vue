<template>
  <div class="settings-view">
    <div class="settings-header">
      <h1>Settings</h1>
      <p class="settings-subtitle">Configure your SwingMusic desktop experience</p>
    </div>

    <div class="settings-sections">
      <!-- Desktop Settings -->
      <section class="settings-section">
        <h2 class="section-title">
          <Monitor />
          Desktop
        </h2>
        
        <div class="setting-item">
          <div class="setting-info">
            <span class="setting-label">Launch at startup</span>
            <span class="setting-description">Automatically start SwingMusic when you log in</span>
          </div>
          <label class="toggle-switch">
            <input 
              type="checkbox" 
              v-model="settings.autoStart"
              @change="updateAutoStart"
            />
            <span class="toggle-slider"></span>
          </label>
        </div>

        <div class="setting-item">
          <div class="setting-info">
            <span class="setting-label">Minimize to system tray</span>
            <span class="setting-description">Keep running in background when window is closed</span>
          </div>
          <label class="toggle-switch">
            <input 
              type="checkbox" 
              v-model="settings.minimizeToTray"
              @change="saveSettings"
            />
            <span class="toggle-slider"></span>
          </label>
        </div>

        <div class="setting-item">
          <div class="setting-info">
            <span class="setting-label">Global hotkeys</span>
            <span class="setting-description">Control playback with media keys from any app</span>
          </div>
          <label class="toggle-switch">
            <input 
              type="checkbox" 
              v-model="settings.globalHotkeys"
              @change="saveSettings"
            />
            <span class="toggle-slider"></span>
          </label>
        </div>
      </section>

      <!-- Appearance Settings -->
      <section class="settings-section">
        <h2 class="section-title">
          <Palette />
          Appearance
        </h2>
        
        <div class="setting-item">
          <div class="setting-info">
            <span class="setting-label">Theme</span>
            <span class="setting-description">Choose your preferred color scheme</span>
          </div>
          <div class="theme-options">
            <button 
              v-for="theme in themeOptions" 
              :key="theme.value"
              @click="setTheme(theme.value)"
              class="theme-option"
              :class="{ active: settings.theme === theme.value }"
            >
              <component :is="theme.icon" />
              {{ theme.label }}
            </button>
          </div>
        </div>
      </section>

      <!-- Audio Settings -->
      <section class="settings-section">
        <h2 class="section-title">
          <Volume2 />
          Audio
        </h2>
        
        <div class="setting-item">
          <div class="setting-info">
            <span class="setting-label">Default volume</span>
            <span class="setting-description">{{ Math.round(settings.volume * 100) }}%</span>
          </div>
          <input 
            type="range"
            v-model.number="settings.volume"
            @input="saveSettings"
            min="0"
            max="1"
            step="0.01"
            class="volume-slider"
          />
        </div>

        <div class="setting-item">
          <div class="setting-info">
            <span class="setting-label">Audio quality</span>
            <span class="setting-description">Higher quality uses more bandwidth</span>
          </div>
          <select v-model="settings.quality" @change="saveSettings" class="setting-select">
            <option value="low">Low (128 kbps)</option>
            <option value="medium">Medium (256 kbps)</option>
            <option value="high">High (320 kbps)</option>
            <option value="lossless">Lossless (FLAC)</option>
          </select>
        </div>
      </section>

      <!-- Downloads Settings -->
      <section class="settings-section">
        <h2 class="section-title">
          <Download />
          Downloads
        </h2>
        
        <div class="setting-item">
          <div class="setting-info">
            <span class="setting-label">Download location</span>
            <span class="setting-description">{{ settings.downloadDir }}</span>
          </div>
          <button @click="selectDownloadDir" class="setting-button">
            <FolderOpen />
            Browse
          </button>
        </div>

        <div class="setting-item">
          <div class="setting-info">
            <span class="setting-label">Download quality</span>
            <span class="setting-description">Quality for downloaded tracks</span>
          </div>
          <select v-model="settings.quality" @change="saveSettings" class="setting-select">
            <option value="medium">Medium (256 kbps)</option>
            <option value="high">High (320 kbps)</option>
            <option value="lossless">Lossless (FLAC)</option>
          </select>
        </div>
      </section>

      <!-- Updates Section -->
      <section class="settings-section">
        <h2 class="section-title">
          <RefreshCw />
          Updates
        </h2>
        
        <div class="setting-item">
          <div class="setting-info">
            <span class="setting-label">Automatic updates</span>
            <span class="setting-description">Check for updates on startup</span>
          </div>
          <label class="toggle-switch">
            <input 
              type="checkbox" 
              v-model="settings.autoUpdate"
              @change="saveSettings"
            />
            <span class="toggle-slider"></span>
          </label>
        </div>

        <div class="setting-item update-check">
          <div class="setting-info">
            <span class="setting-label">Current version</span>
            <span class="setting-description">v{{ currentVersion }}</span>
          </div>
          <button 
            @click="checkForUpdates" 
            class="setting-button primary"
            :disabled="checkingForUpdate"
          >
            <RefreshCw :class="{ spinning: checkingForUpdate }" />
            {{ checkingForUpdate ? 'Checking...' : 'Check for updates' }}
          </button>
        </div>

        <div v-if="updateAvailable" class="update-banner">
          <div class="update-info">
            <span class="update-title">Update available!</span>
            <span class="update-version">Version {{ updateInfo.version }} is ready to install</span>
          </div>
          <button @click="installUpdate" class="update-button">
            <Download />
            Install Update
          </button>
        </div>
      </section>

      <!-- Connection Section -->
      <section class="settings-section">
        <h2 class="section-title">
          <Cloud />
          Web App Connection
        </h2>
        
        <div class="setting-item">
          <div class="setting-info">
            <span class="setting-label">Connection status</span>
            <span class="setting-description">
              <span :class="['status-dot', connectionStatus]"></span>
              {{ connectionStatusText }}
            </span>
          </div>
          <button 
            v-if="isConnected"
            @click="disconnectFromWebApp"
            class="setting-button danger"
          >
            <Unlink />
            Disconnect
          </button>
          <button 
            v-else
            @click="showConnectionModal"
            class="setting-button primary"
          >
            <Link />
            Connect
          </button>
        </div>

        <div v-if="isConnected" class="connection-info">
          <div class="info-item">
            <span class="info-label">Server URL</span>
            <span class="info-value">{{ connectionInfo.url }}</span>
          </div>
          <div class="info-item">
            <span class="info-label">Username</span>
            <span class="info-value">{{ connectionInfo.username }}</span>
          </div>
          <div class="info-item">
            <span class="info-label">Connected since</span>
            <span class="info-value">{{ formatDate(connectionInfo.connectedAt) }}</span>
          </div>
        </div>
      </section>

      <!-- About Section -->
      <section class="settings-section">
        <h2 class="section-title">
          <Info />
          About
        </h2>
        
        <div class="about-content">
          <div class="app-logo">
            <img src="/icons/logo.png" alt="SwingMusic" />
          </div>
          <div class="app-info">
            <h3>SwingMusic Desktop</h3>
            <p>Version {{ currentVersion }}</p>
            <p class="app-description">
              A powerful desktop music player with Universal Source integration, 
              intelligent recommendations, and comprehensive audio management.
            </p>
            <div class="app-links">
              <a href="https://github.com/Dvorinka/swingmusic-extended" target="_blank">
                <Github />
                GitHub
              </a>
              <a href="https://github.com/Dvorinka/swingmusic-extended/issues" target="_blank">
                <MessageCircle />
                Report Issue
              </a>
            </div>
          </div>
        </div>
      </section>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'
import { open } from '@tauri-apps/plugin-dialog'
import { 
  Monitor, Palette, Volume2, Download, RefreshCw, Cloud, Info,
  FolderOpen, Link, Unlink, Github, MessageCircle, Sun, Moon, Monitor as MonitorIcon
} from 'lucide-vue-next'

// Current version from package
const currentVersion = ref('1.0.0')

// Settings state
const settings = reactive({
  autoStart: false,
  minimizeToTray: true,
  globalHotkeys: true,
  theme: 'dark',
  volume: 0.75,
  quality: 'high',
  downloadDir: '',
  autoUpdate: true
})

// Update state
const checkingForUpdate = ref(false)
const updateAvailable = ref(false)
const updateInfo = ref<any>(null)

// Connection state
const isConnected = ref(false)
const connectionInfo = ref<any>({
  url: '',
  username: '',
  connectedAt: null
})

// Theme options
const themeOptions = [
  { value: 'light', label: 'Light', icon: Sun },
  { value: 'dark', label: 'Dark', icon: Moon },
  { value: 'system', label: 'System', icon: MonitorIcon }
]

// Connection status
const connectionStatus = computed(() => {
  return isConnected.value ? 'connected' : 'disconnected'
})

const connectionStatusText = computed(() => {
  return isConnected.value ? 'Connected' : 'Not connected'
})

// Load settings on mount
onMounted(async () => {
  await loadSettings()
  await checkConnectionStatus()
})

// Load settings from backend
const loadSettings = async () => {
  try {
    const appSettings = await invoke('get_app_settings')
    Object.assign(settings, appSettings)
  } catch (error) {
    console.error('Failed to load settings:', error)
  }
}

// Save settings to backend
const saveSettings = async () => {
  try {
    await invoke('update_app_settings', { settings })
  } catch (error) {
    console.error('Failed to save settings:', error)
  }
}

// Update auto-start setting
const updateAutoStart = async () => {
  try {
    await invoke('set_auto_start', { enabled: settings.autoStart })
    await saveSettings()
  } catch (error) {
    console.error('Failed to update auto-start:', error)
    settings.autoStart = !settings.autoStart // Revert on error
  }
}

// Set theme
const setTheme = async (theme: string) => {
  settings.theme = theme
  document.documentElement.classList.toggle('dark', theme === 'dark')
  await saveSettings()
}

// Select download directory
const selectDownloadDir = async () => {
  try {
    const selected = await open({
      directory: true,
      multiple: false,
      defaultPath: settings.downloadDir
    })
    
    if (selected) {
      settings.downloadDir = selected as string
      await saveSettings()
    }
  } catch (error) {
    console.error('Failed to select directory:', error)
  }
}

// Check for updates
const checkForUpdates = async () => {
  checkingForUpdate.value = true
  
  try {
    const update = await invoke('check_for_updates')
    
    if (update) {
      updateInfo.value = update
      updateAvailable.value = true
    } else {
      updateAvailable.value = false
    }
  } catch (error) {
    console.error('Failed to check for updates:', error)
  } finally {
    checkingForUpdate.value = false
  }
}

// Install update
const installUpdate = async () => {
  try {
    await invoke('install_update')
    // App will restart automatically
  } catch (error) {
    console.error('Failed to install update:', error)
  }
}

// Check connection status
const checkConnectionStatus = async () => {
  try {
    // This would check the actual connection status from the backend
    // For now, we'll just set it based on stored settings
  } catch (error) {
    console.error('Failed to check connection status:', error)
  }
}

// Show connection modal
const showConnectionModal = () => {
  // Emit event to parent to show connection modal
  // This would be handled by the parent component
}

// Disconnect from web app
const disconnectFromWebApp = async () => {
  try {
    await invoke('disconnect_from_web_app')
    isConnected.value = false
    connectionInfo.value = {
      url: '',
      username: '',
      connectedAt: null
    }
  } catch (error) {
    console.error('Failed to disconnect:', error)
  }
}

// Format date
const formatDate = (dateString: string) => {
  if (!dateString) return ''
  return new Date(dateString).toLocaleDateString()
}
</script>

<style scoped>
.settings-view {
  height: 100%;
  overflow-y: auto;
  padding: 24px;
  background: var(--background-primary);
}

.settings-header {
  margin-bottom: 32px;
}

.settings-header h1 {
  font-size: 32px;
  font-weight: 700;
  margin-bottom: 8px;
  color: var(--text-primary);
}

.settings-subtitle {
  font-size: 14px;
  color: var(--text-secondary);
}

.settings-sections {
  display: flex;
  flex-direction: column;
  gap: 32px;
  max-width: 800px;
}

.settings-section {
  background: var(--background-secondary);
  border-radius: 12px;
  padding: 24px;
  border: 1px solid var(--border-primary);
}

.section-title {
  display: flex;
  align-items: center;
  gap: 12px;
  font-size: 18px;
  font-weight: 600;
  margin-bottom: 20px;
  color: var(--text-primary);
}

.section-title svg {
  width: 20px;
  height: 20px;
  color: var(--accent-primary);
}

.setting-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 16px 0;
  border-bottom: 1px solid var(--border-secondary);
}

.setting-item:last-child {
  border-bottom: none;
}

.setting-info {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.setting-label {
  font-size: 14px;
  font-weight: 500;
  color: var(--text-primary);
}

.setting-description {
  font-size: 12px;
  color: var(--text-secondary);
}

/* Toggle Switch */
.toggle-switch {
  position: relative;
  display: inline-block;
  width: 44px;
  height: 24px;
}

.toggle-switch input {
  opacity: 0;
  width: 0;
  height: 0;
}

.toggle-slider {
  position: absolute;
  cursor: pointer;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: var(--background-tertiary);
  transition: 0.3s;
  border-radius: 24px;
}

.toggle-slider:before {
  position: absolute;
  content: "";
  height: 18px;
  width: 18px;
  left: 3px;
  bottom: 3px;
  background-color: white;
  transition: 0.3s;
  border-radius: 50%;
}

input:checked + .toggle-slider {
  background-color: var(--accent-primary);
}

input:checked + .toggle-slider:before {
  transform: translateX(20px);
}

/* Theme Options */
.theme-options {
  display: flex;
  gap: 8px;
}

.theme-option {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 8px 12px;
  border: 1px solid var(--border-secondary);
  background: var(--background-tertiary);
  border-radius: 8px;
  cursor: pointer;
  font-size: 13px;
  color: var(--text-primary);
  transition: all 0.2s;
}

.theme-option:hover {
  border-color: var(--accent-primary);
}

.theme-option.active {
  background: var(--accent-primary);
  border-color: var(--accent-primary);
  color: white;
}

.theme-option svg {
  width: 16px;
  height: 16px;
}

/* Volume Slider */
.volume-slider {
  width: 150px;
  height: 4px;
  background: var(--background-tertiary);
  border-radius: 2px;
  outline: none;
  -webkit-appearance: none;
}

.volume-slider::-webkit-slider-thumb {
  -webkit-appearance: none;
  width: 16px;
  height: 16px;
  background: var(--accent-primary);
  border-radius: 50%;
  cursor: pointer;
}

/* Select */
.setting-select {
  padding: 8px 12px;
  background: var(--background-tertiary);
  border: 1px solid var(--border-secondary);
  border-radius: 8px;
  color: var(--text-primary);
  font-size: 13px;
  cursor: pointer;
  outline: none;
}

.setting-select:hover {
  border-color: var(--accent-primary);
}

/* Buttons */
.setting-button {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 8px 16px;
  background: var(--background-tertiary);
  border: 1px solid var(--border-secondary);
  border-radius: 8px;
  color: var(--text-primary);
  font-size: 13px;
  cursor: pointer;
  transition: all 0.2s;
}

.setting-button:hover {
  background: var(--background-hover);
  border-color: var(--accent-primary);
}

.setting-button.primary {
  background: var(--accent-primary);
  border-color: var(--accent-primary);
  color: white;
}

.setting-button.primary:hover {
  background: var(--accent-hover);
}

.setting-button.danger {
  color: var(--error-color);
  border-color: var(--error-color);
}

.setting-button.danger:hover {
  background: var(--error-color);
  color: white;
}

.setting-button svg {
  width: 16px;
  height: 16px;
}

.setting-button:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

/* Update Banner */
.update-banner {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 16px;
  margin-top: 16px;
  background: var(--success-background);
  border: 1px solid var(--success-color);
  border-radius: 8px;
}

.update-info {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.update-title {
  font-weight: 600;
  color: var(--success-color);
}

.update-version {
  font-size: 12px;
  color: var(--text-secondary);
}

.update-button {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 8px 16px;
  background: var(--success-color);
  border: none;
  border-radius: 8px;
  color: white;
  font-size: 13px;
  cursor: pointer;
  transition: all 0.2s;
}

.update-button:hover {
  filter: brightness(1.1);
}

.update-button svg {
  width: 16px;
  height: 16px;
}

/* Spinning animation */
.spinning {
  animation: spin 1s linear infinite;
}

@keyframes spin {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}

/* Status Dot */
.status-dot {
  display: inline-block;
  width: 8px;
  height: 8px;
  border-radius: 50%;
  margin-right: 6px;
}

.status-dot.connected {
  background: var(--success-color);
}

.status-dot.disconnected {
  background: var(--text-secondary);
}

/* Connection Info */
.connection-info {
  margin-top: 16px;
  padding: 16px;
  background: var(--background-tertiary);
  border-radius: 8px;
}

.info-item {
  display: flex;
  justify-content: space-between;
  padding: 8px 0;
}

.info-label {
  font-size: 12px;
  color: var(--text-secondary);
}

.info-value {
  font-size: 12px;
  color: var(--text-primary);
  font-weight: 500;
}

/* About Section */
.about-content {
  display: flex;
  gap: 24px;
  align-items: flex-start;
}

.app-logo img {
  width: 80px;
  height: 80px;
  border-radius: 16px;
}

.app-info h3 {
  font-size: 20px;
  font-weight: 600;
  margin-bottom: 4px;
}

.app-info p {
  font-size: 12px;
  color: var(--text-secondary);
  margin-bottom: 8px;
}

.app-description {
  max-width: 400px;
  line-height: 1.5;
}

.app-links {
  display: flex;
  gap: 12px;
  margin-top: 12px;
}

.app-links a {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 6px 12px;
  background: var(--background-tertiary);
  border-radius: 6px;
  color: var(--text-primary);
  text-decoration: none;
  font-size: 12px;
  transition: all 0.2s;
}

.app-links a:hover {
  background: var(--background-hover);
}

.app-links a svg {
  width: 14px;
  height: 14px;
}
</style>
